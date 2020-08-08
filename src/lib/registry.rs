use crate::filters::{Filter, ListFilter, NoopFilter};
use crate::storage::FileStorage;
use crate::types::URLRecord;
use crate::Repository;
use std::ops::Deref;

pub struct Registry<T: Repository> {
    storage: T,
}

impl<T: Repository> Registry<T> {
    pub fn new_file_based(file_path: String) -> Registry<FileStorage> {
        let storage = FileStorage::new_urls_repository(file_path);

        Registry { storage }
    }

    pub fn add_url(
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

    pub fn delete(
        &self,
        name: &str,
        group: Option<&str>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let group = group.unwrap_or("default");

        self.storage.delete(name, group)
    }

    pub fn list_groups(&self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        self.storage.list_groups()
    }

    pub fn list_urls(
        &self,
        group: Option<&str>,
        tags: Option<Vec<&str>>,
    ) -> Result<Vec<URLRecord>, Box<dyn std::error::Error>> {
        let filter: Box<dyn Filter> = if let Some(t) = tags {
            Box::new(ListFilter::new_tags_filter(t))
        } else {
            Box::new(NoopFilter::new())
        };

        self.storage.list(group, filter.deref())
    }
}

#[cfg(test)]
mod test {
    use crate::registry::Registry;
    use crate::storage::FileStorage;
    use crate::types::URLRecord;
    use std::collections::HashMap;
    use std::fs::File;
    use std::path::PathBuf;
    use std::{env, fs};

    struct TestUrl {
        name: &'static str,
        url: &'static str,
        group: Option<&'static str>,
        tags: Vec<&'static str>,
    }

    #[test]
    fn registry_test() {
        let file_path = create_temp_file("test_1.txt");

        let registry =
            Registry::<FileStorage>::new_file_based(file_path.to_str().unwrap().to_string());

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
                tags: vec!["tagged"],
            },
        ];

        let all_urls: Vec<&TestUrl> = test_urls.iter().collect();

        // Add URLs
        for tu in &all_urls {
            let result = registry
                .add_url(
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

        // List groups
        let groups = registry.list_groups().expect("Failed to list groups");
        assert!(groups.contains(&"default".to_string()));
        assert!(groups.contains(&"test".to_string()));

        // List all URLs
        let urls = registry.list_urls(None, None).expect("Failed to list urls");

        assert_urls_match(&all_urls, urls);

        // List URLs from specific group
        let group_filter = "test";

        let urls = registry
            .list_urls(Some(group_filter.clone()), None)
            .expect("Failed to list urls");
        assert_eq!(1, urls.len());

        let filtered_test_cases: Vec<&TestUrl> = test_urls
            .iter()
            .clone()
            .filter(|t| {
                if let Some(group) = &t.group {
                    return *group == group_filter;
                }
                false
            })
            .collect();

        assert_urls_match(&filtered_test_cases, urls);

        // List tagged URLs
        let tags_to_filter = vec!["tagged"];

        let urls = registry
            .list_urls(None, Some(tags_to_filter))
            .expect("Failed to list urls");
        assert_eq!(2, urls.len());

        let filtered_test_cases: Vec<&TestUrl> = vec![&test_urls[1], &test_urls[2]];
        assert_urls_match(&filtered_test_cases, urls);

        // Delete existing URL
        let deleted = registry
            .delete("test1", None)
            .expect("Failed to delete URL");
        assert!(deleted);
        let urls = registry.list_urls(None, None).expect("Failed to list urls");
        assert_eq!(2, urls.len());

        // Not delete if URL does not exist
        let deleted = registry
            .delete("test1", None)
            .expect("Failed to delete URL");
        assert!(!deleted);
        let urls = registry.list_urls(None, None).expect("Failed to list urls");
        assert_eq!(2, urls.len());

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

    fn create_temp_file(file: &str) -> PathBuf {
        let mut temp_path = env::temp_dir();
        temp_path.push(file);

        File::create(temp_path.clone()).expect("Failed to create temp file");

        return temp_path;
    }
}
