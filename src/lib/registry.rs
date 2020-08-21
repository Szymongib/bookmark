use crate::storage::FileStorage;
use crate::types::URLRecord;
use crate::{Repository, Registry, RegistryReader, RegistryConfig};
use std::ops::Deref;
use std::error::Error;
use std::path::PathBuf;
use crate::util::create_temp_file;
use crate::filters::{Filter, NoopFilter};

// TODO: introduce custom errors

pub struct URLRegistry<'a, T: Repository> {
    storage: T,
    filter: Box<dyn Filter + 'a>,
}

impl<'a> URLRegistry<'a, FileStorage> {
    pub fn new_file_based<F: Filter + 'a>(file_path: String, filter: Option<F>) -> URLRegistry<'a, FileStorage> {
        let storage = FileStorage::new_urls_repository(file_path);

        let f :Box<dyn Filter> = if filter.is_none() {
            Box::new(NoopFilter::new())
        } else {
            Box::new(filter.unwrap())
        };

        URLRegistry {
            storage,
            filter: f,
        }
    }

    pub fn with_temp_file<F: Filter + 'a>(suffix: &str, filter: Option<F>) -> Result<(URLRegistry<'a, FileStorage>, PathBuf), Box<dyn std::error::Error>> {
        let file_path = create_temp_file(suffix)?;

        return match file_path.to_str() {
            Some(path) => {
                Ok((URLRegistry::new_file_based(path.to_string(), filter), file_path))
            },
            None => {
                Err(From::from("failed to initialized registry with temp file, path is None"))
            }
        }
    }
}

impl<T: Repository> Registry for URLRegistry<'_, T> {
    fn new(
        &self,
        name: &str,
        url: &str,
        group: Option<&str>,
        tags: Vec<&str>,
    ) -> Result<URLRecord, Box<dyn std::error::Error>> {
        let group = group.unwrap_or("default");

        let record = URLRecord::new(url, name, group, tags);

        self.storage.add(record)
    }

    fn add_url(&self, record: URLRecord) -> Result<URLRecord, Box<dyn Error>> {
        self.storage.add(record)
    }

    fn delete_by_id(&self, id: &str) -> Result<bool, Box<dyn Error>> {
        self.storage.delete_by_id(id)
    }

    fn list_groups(&self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        self.storage.list_groups()
    }

    fn tag_url(&self, id: String, tag: String) -> Result<Option<URLRecord>, Box<dyn Error>> {
        if tag == "" {
            return Err(From::from("Tag cannot be an empty string"))
        }

        let mut record = self.storage.get(id.clone())?; // TODO: what should be returned here

        record.map_or(Ok(None), |mut record| {
            record.tags.entry(tag.clone()).or_insert(true);
            self.storage.update(id, record)
        })
    }
}

impl<T: Repository> RegistryReader for URLRegistry<'_,T> {
    fn list_urls(&self) -> Result<Vec<URLRecord>, Box<dyn std::error::Error>> {
        let urls = self.storage.list()?;

        Ok(urls.iter().filter(|url|{
            self.filter.matches(*url)
        }).map(|u| {u.to_owned()})
            .collect())
    }

    fn get_url(&self, id: String) -> Result<Option<URLRecord>, Box<dyn Error>> {
        self.storage.get(id)
    }
}

impl<'a, T: Repository> RegistryConfig<'a> for URLRegistry<'a,T> {
    fn set_filter<F: Filter + 'a>(&mut self, filter: F) {
        self.filter = Box::new(filter);
    }
}

#[cfg(test)]
mod test {
    use crate::registry::URLRegistry;
    use crate::storage::FileStorage;
    use crate::types::{URLRecord, calculate_hash};
    use std::collections::HashMap;
    use std::{fs};
    use crate::{Registry, RegistryReader, RegistryConfig};
    use crate::filters::{NoopFilter, GroupFilter, TagsFilter};

    struct TestUrl {
        name: &'static str,
        url: &'static str,
        group: Option<&'static str>,
        tags: Vec<&'static str>,
    }

    #[test]
    fn registry_test() {
        let (mut registry, file_path) = URLRegistry::<FileStorage>::with_temp_file::<NoopFilter>("registry_tests.json", None).expect("Failed to initialize registry");

        let test_urls: Vec<TestUrl> = vec![
            TestUrl {
                name: "test1",
                url: "https://test.com",
                group: None,
                tags: vec![],
            },
            TestUrl {
                name: "test_tagged",
                url: "https://test2.com",
                group: None,
                tags: vec!["tagged"],
            },
            TestUrl {
                name: "test_group",
                url: "https://test_group.com",
                group: Some("test"),
                tags: vec!["tag2"],
            },
        ];

        let all_urls: Vec<&TestUrl> = test_urls.iter().collect();

        println!("Add URLs...");
        for tu in &all_urls {
            let result = registry
                .new(
                    tu.name.clone(),
                    tu.url.clone(),
                    tu.group.clone(),
                    tu.tags.clone(),
                )
                .expect("Failed to add URL record");
            assert_eq!(tu.name, result.name);
            assert_eq!(tu.url, result.url);
            assert!(group_match(&tu.group, &result.group));
            assert!(tags_match(&tu.tags, &result.tags))
        }

        println!("List groups...");
        let groups = registry.list_groups().expect("Failed to list groups");
        assert!(groups.contains(&"default".to_string()));
        assert!(groups.contains(&"test".to_string()));

        // List all URLs
        println!("List groups...");
        let urls = registry.list_urls().expect("Failed to list urls");

        assert_urls_match(&all_urls, urls);

        println!("List URLs from specific group...");
        let group_to_filter= "test";
        let group_filter = GroupFilter::new(group_to_filter.clone());

        registry.set_filter(group_filter);

        let urls = registry
            .list_urls()
            .expect("Failed to list urls");
        assert_eq!(1, urls.len());

        let filtered_test_cases: Vec<&TestUrl> = test_urls
            .iter()
            .clone()
            .filter(|t| {
                if let Some(group) = &t.group {
                    return *group == group_to_filter.clone();
                }
                false
            })
            .collect();

        assert_urls_match(&filtered_test_cases, urls);

        println!("List tagged URLs...");
        let tags_to_filter = vec!["tagged", "tag2"];
        let tags_filter = TagsFilter::new(tags_to_filter.clone());

        registry.set_filter(tags_filter);

        let urls = registry
            .list_urls()
            .expect("Failed to list urls");
        assert_eq!(2, urls.len());

        let filtered_test_cases: Vec<&TestUrl> = vec![&test_urls[1], &test_urls[2]];
        assert_urls_match(&filtered_test_cases, urls);

        println!("Delete existing URL...");
        registry.set_filter(NoopFilter::new());
        let url_0_id = calculate_hash("test1", "default");

        let deleted = registry
            .delete_by_id(&url_0_id)
            .expect("Failed to delete URL");
        assert!(deleted);
        let urls = registry.list_urls().expect("Failed to list urls");
        assert_eq!(2, urls.len());

        println!("Not delete if URL does not exist...");
        let deleted = registry
            .delete_by_id(&url_0_id)
            .expect("Failed to delete URL");
        assert!(!deleted);
        let urls = registry.list_urls().expect("Failed to list urls");
        assert_eq!(2, urls.len());

        println!("Get url by ID...");
        let id = calculate_hash("test_tagged", "default");
        let url_record = registry.get_url(id).expect("Failed to get URL");

        assert_eq!(url_record.expect("URL is None").id, urls[0].id);

        println!("Cleanup...");
        fs::remove_file(file_path).expect("Failed to remove file");
    }

    fn assert_urls_match(test_urls: &Vec<&TestUrl>, actual: Vec<URLRecord>) {
        for tu in test_urls {
            let exists = actual.iter().any(|rec| {
                rec.name == tu.name.clone()
                    && rec.url == tu.url.clone()
                    && group_match(&tu.group, &rec.group)
                    && tags_match(&tu.tags, &rec.tags)
            });
            assert!(exists)
        }
    }

    fn group_match(input: &Option<&str>, actual: &String) -> bool {
        if let Some(g) = input {
            return g == actual;
        } else {
            "default" == actual
        }
    }

    fn tags_match(expected: &Vec<&str>, actual: &HashMap<String, bool>) -> bool {
        for t in expected {
            let tag = actual.get(*t).expect("Tag not present");
            if !tag {
                return false;
            }
        }

        true
    }
}
