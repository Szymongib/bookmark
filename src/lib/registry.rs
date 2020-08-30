use crate::filters::{Filter, NoopFilter};
use crate::storage::FileStorage;
use crate::types::URLRecord;
use crate::util::create_temp_file;
use crate::{Importer, Registry, RegistryReader, Repository};
use std::error::Error;
use std::path::PathBuf;

// TODO: introduce custom errors

// TODO: pass filter to list function?

pub struct URLRegistry<T: Repository> {
    storage: T,
}

impl URLRegistry<FileStorage> {
    pub fn new_file_based(file_path: String) -> URLRegistry<FileStorage> {
        let storage = FileStorage::new_urls_repository(file_path);

        URLRegistry { storage }
    }

    pub fn with_temp_file(
        suffix: &str,
    ) -> Result<(URLRegistry<FileStorage>, PathBuf), Box<dyn std::error::Error>> {
        let file_path = create_temp_file(suffix)?;

        return match file_path.to_str() {
            Some(path) => Ok((URLRegistry::new_file_based(path.to_string()), file_path)),
            None => Err(From::from(
                "failed to initialized registry with temp file, path is None",
            )),
        };
    }
}

impl<T: Repository> Registry for URLRegistry<T> {
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

    fn add(&self, record: URLRecord) -> Result<URLRecord, Box<dyn Error>> {
        self.storage.add(record)
    }

    fn delete(&self, id: &str) -> Result<bool, Box<dyn Error>> {
        self.storage.delete_by_id(id)
    }

    fn list_groups(&self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        self.storage.list_groups()
    }

    fn tag(&self, id: &str, tag: &str) -> Result<Option<URLRecord>, Box<dyn Error>> {
        if tag == "" {
            return Err(From::from("Tag cannot be an empty string"));
        }

        let record = self.storage.get(id)?; // TODO: what should be returned here

        record.map_or(Ok(None), |mut record| {
            record.tags.entry(tag.to_string()).or_insert(true);
            self.storage.update(id, record)
        })
    }

    fn untag(&self, id: &str, tag: &str) -> Result<Option<URLRecord>, Box<dyn Error>> {
        if tag == "" {
            return Err(From::from("Tag cannot be an empty string"));
        }
        let record = self.storage.get(id)?;

        record.map_or(Ok(None), |mut record| {
            record.tags.remove(tag);
            self.storage.update(id, record)
        })
    }

    fn change_group(&self, id: &str, group: &str) -> Result<Option<URLRecord>, Box<dyn Error>> {
        if group == "" {
            return Err(From::from("Group cannot be an empty string"));
        }

        let record = self.storage.get(id)?;

        record.map_or(Ok(None), |mut record| {
            record.group = group.to_string();
            self.storage.update(id, record)
        })
    }
}

impl<T: Repository> RegistryReader for URLRegistry<T> {
    fn list_urls(
        &self,
        filter: Option<&Box<dyn Filter>>,
    ) -> Result<Vec<URLRecord>, Box<dyn std::error::Error>> {
        let urls = self.storage.list()?;
        let noop: Box<dyn Filter> = Box::new(NoopFilter::new()); // TODO: move as struct member

        let filter = filter.unwrap_or(&noop);

        Ok(urls
            .iter()
            .filter(|url| {
                filter.matches(*url)
            })
            .map(|u| u.to_owned())
            .collect())
    }

    fn get_url(&self, id: &str) -> Result<Option<URLRecord>, Box<dyn Error>> {
        self.storage.get(id)
    }
}

impl<T: Repository> Importer for URLRegistry<T> {
    // TODO: opts for overriding dups, opt for migrating only unique
    fn import_from_v_0_0_x(&self, path: &str) -> Result<Vec<URLRecord>, Box<dyn Error>> {
        let old_urls = self.storage.list_v_0_0_x(path)?;
        let urls: Vec<URLRecord> = old_urls
            .iter()
            .map(|u| {
                let tags = u.tags.iter().map(|(t, _)| t.as_str()).collect();
                URLRecord::new(&u.url, &u.name, &u.group, tags)
            })
            .collect();

        // If at least one items fails, nothing will be saved
        self.storage.add_batch(urls)
    }
}

#[cfg(test)]
mod test {
    use crate::filters::Filter;
    use crate::filters::{GroupFilter, TagsFilter};
    use crate::registry::URLRegistry;
    use crate::storage::FileStorage;
    use crate::types::URLRecord;
    use crate::util::create_temp_file;
    use crate::{Importer, Registry, RegistryReader};
    use std::collections::HashMap;
    use std::fs;
    use std::fs::OpenOptions;
    use std::io::{Seek, SeekFrom, Write};
    use std::path::PathBuf;

    struct TestUrl {
        name: &'static str,
        url: &'static str,
        group: Option<&'static str>,
        tags: Vec<&'static str>,
    }

    #[test]
    fn registry_test() {
        let (registry, file_path) =
            URLRegistry::<FileStorage>::with_temp_file("registry_tests.json")
                .expect("Failed to initialize registry");

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
        let urls = registry.list_urls(None).expect("Failed to list urls");

        assert_urls_match(&all_urls, &urls);

        println!("List URLs from specific group...");
        let group_to_filter = "test";
        let group_filter: Box<dyn Filter> = Box::new(GroupFilter::new(group_to_filter.clone()));

        let urls = registry
            .list_urls(Some(&group_filter))
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

        assert_urls_match(&filtered_test_cases, &urls);

        println!("List tagged URLs...");
        let tags_to_filter = vec!["tagged", "tag2"];
        let tags_filter: Box<dyn Filter> = Box::new(TagsFilter::new(tags_to_filter.clone()));

        let urls = registry
            .list_urls(Some(&tags_filter))
            .expect("Failed to list urls");
        assert_eq!(2, urls.len());

        let filtered_test_cases: Vec<&TestUrl> = vec![&test_urls[1], &test_urls[2]];
        assert_urls_match(&filtered_test_cases, &urls);

        println!("Delete existing URL...");
        let url_0_id = urls[0].id.clone();

        let deleted = registry.delete(&url_0_id).expect("Failed to delete URL");
        assert!(deleted);
        let urls = registry.list_urls(None).expect("Failed to list urls");
        assert_eq!(2, urls.len());

        println!("Not delete if URL does not exist...");
        let deleted = registry.delete(&url_0_id).expect("Failed to delete URL");
        assert!(!deleted);
        let urls = registry.list_urls(None).expect("Failed to list urls");
        assert_eq!(2, urls.len());

        println!("Tag URL...");
        let id = urls[0].id.clone();

        let url_record = registry
            .tag(&id, "some-awesome-tag")
            .expect("Failed to tag URL")
            .expect("URL record is None");
        assert!(url_record.tags.contains_key("some-awesome-tag"));

        println!("Get url by ID...");
        let url_record = registry
            .get_url(&id)
            .expect("Failed to get URL")
            .expect("URL record is None");
        assert_eq!(url_record.id, urls[0].id);
        assert!(url_record.tags.contains_key("some-awesome-tag"));

        println!("Untag URL...");
        let url_record = registry
            .untag(&id, "some-awesome-tag")
            .expect("Failed to tag URL")
            .expect("URL record is None");
        assert!(!url_record.tags.contains_key("some-awesome-tag"));

        println!("Change group...");
        let id = urls[0].id.clone();

        let url_record = registry
            .change_group(&id, "different-group")
            .expect("Failed to tag URL")
            .expect("URL record is None");
        assert_eq!(url_record.group, "different-group");
        let record = registry.get_url(&id)
            .expect("Failed to get URL")
            .expect("URL record is None");
        assert_eq!(record.group, "different-group");

        println!("Cleanup...");
        fs::remove_file(file_path).expect("Failed to remove file");
    }

    fn assert_urls_match(test_urls: &Vec<&TestUrl>, actual: &Vec<URLRecord>) {
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

    #[test]
    fn import_from_v0_0_x_test() {
        let (registry, file_path) =
            URLRegistry::<FileStorage>::with_temp_file("registry_tests2.json")
                .expect("Failed to initialize registry");

        let expected_urls = vec![
            URLRecord::new(
                "https://github.com/Szymongib/bookmark-cli",
                "Bookmark-CLI",
                "projects",
                vec!["rust", "repo"],
            ),
            URLRecord::new("https://github.com", "GitHub.com", "websites", vec![]),
            URLRecord::new(
                "https://youtube.com",
                "YouTube",
                "entertainment",
                vec!["video"],
            ),
            URLRecord::new(
                "https://stackoverflow.com",
                "Stack Overflow",
                "dev",
                vec!["help", "dev"],
            ),
            URLRecord::new("https://reddit.com", "Reddit", "entertainment", vec![]),
        ];

        let old_path = setup_old_urls_file();

        println!("Should import URLs...");
        let imported = registry
            .import_from_v_0_0_x(old_path.as_os_str().to_str().expect("Failed to get path"))
            .expect("Failed to import bookmarks");

        assert_eq!(imported.len(), 5);
        for i in 0..imported.len() {
            assert_eq!(imported[i].id.len(), 36);
            assert_eq!(imported[i].name, expected_urls[i].name);
            assert_eq!(imported[i].url, expected_urls[i].url);
            assert_eq!(imported[i].group, expected_urls[i].group);
            assert_eq!(imported[i].tags, expected_urls[i].tags);
        }

        println!("Should fail if URLs not unique...");
        let imported = registry
            .import_from_v_0_0_x(old_path.as_os_str().to_str().expect("Failed to get path"));
        assert!(imported.is_err());

        println!("Cleanup...");
        fs::remove_file(file_path).expect("Failed to remove file");
        fs::remove_file(old_path).expect("Failed to remove file");
    }

    fn setup_old_urls_file() -> PathBuf {
        let old_file_content = OLD_BOOKMARKS_FILE_CONTENT;
        let path =
            create_temp_file("registry_tests_old_file.json").expect("Failed to create temp file");

        let mut file = OpenOptions::new()
            .read(true)
            .create(true)
            .append(false)
            .write(true)
            .open(path.clone())
            .expect("Failed to open old URLs file");

        file.seek(SeekFrom::Start(0))
            .expect("Failed to seek to file start");
        file.write_all(old_file_content.as_bytes())
            .expect("Failed to write od URLs");

        return path;
    }

    const OLD_BOOKMARKS_FILE_CONTENT: &str = r###"
    {
  "urls": {
    "items": [
      {
        "url": "https://github.com/Szymongib/bookmark-cli",
        "name": "Bookmark-CLI",
        "group": "projects",
        "tags": {
          "rust": true,
          "repo": true
        }
      },
      {
        "url": "https://github.com",
        "name": "GitHub.com",
        "group": "websites",
        "tags": {}
      },
      {
        "url": "https://youtube.com",
        "name": "YouTube",
        "group": "entertainment",
        "tags": {
          "video": true
        }
      },
      {
        "url": "https://stackoverflow.com",
        "name": "Stack Overflow",
        "group": "dev",
        "tags": {
          "help": true,
          "dev": true
        }
      },
      {
        "url": "https://reddit.com",
        "name": "Reddit",
        "group": "entertainment",
        "tags": {}
      }
    ]
  }
}
"###;
}
