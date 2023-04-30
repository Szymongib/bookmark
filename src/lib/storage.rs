use super::types::{URLRecord, URLRegistry};
use crate::import::v0_0_x;
use crate::types::URLs;
use crate::{Repository, RepositoryOld};
use std::collections::HashMap;
use std::convert::TryInto;
use std::error::Error;
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::Path;

#[derive(Debug, Clone)]
pub struct FileStorage {
    file_path: String,
}

impl FileStorage {
    pub fn new_urls_repository(file_path: String) -> FileStorage {
        FileStorage { file_path }
    }

    fn delete_url<F>(&self, match_first: F) -> Result<bool, Box<dyn std::error::Error>>
    where
        F: Fn(&URLRecord) -> bool,
    {
        let mut file = open_urls_file(self.file_path.as_str())?;
        let mut registry = read_urls(&mut file)?;

        for (index, u) in registry.urls.items.iter().enumerate() {
            if match_first(u) {
                registry.urls.items.remove(index);
                write_urls(&mut file, registry)?;
                return Ok(true);
            }
        }

        Ok(false)
    }
}

impl Repository for FileStorage {
    fn add(&self, record: URLRecord) -> Result<URLRecord, Box<dyn Error>> {
        let mut file = open_urls_file(self.file_path.as_str())?;
        let mut registry = read_urls(&mut file)?;

        if !is_unique(&registry.urls.items, &record) {
            return Err(not_unique_error(&record));
        }

        registry.urls.items.push(record.clone());

        write_urls(&mut file, registry)?;

        Ok(record)
    }

    /// Adds all records to the registry as long as all of them are unique
    /// If at least one name-group pair is not unique, none of the URLs is saved
    fn add_batch(&self, records: Vec<URLRecord>) -> Result<Vec<URLRecord>, Box<dyn Error>> {
        let mut file = open_urls_file(self.file_path.as_str())?;
        let mut registry = read_urls(&mut file)?;

        for r in &records {
            if !is_unique(&registry.urls.items, &r) {
                return Err(not_unique_error(&r));
            }
            registry.urls.items.push(r.clone());
        }

        let registry = write_urls(&mut file, registry)?;

        Ok(registry.urls.items)
    }

    fn delete_by_id(&self, id: &str) -> Result<bool, Box<dyn Error>> {
        self.delete_url(|u| u.id == id)
    }

    fn list(&self) -> Result<Vec<URLRecord>, Box<dyn std::error::Error>> {
        let mut file = open_urls_file(self.file_path.as_str())?;
        let registry = read_urls(&mut file)?;
        Ok(registry.urls.items)
    }

    fn get(&self, id: &str) -> Result<Option<URLRecord>, Box<dyn Error>> {
        let mut file = open_urls_file(self.file_path.as_str())?;
        let registry = read_urls(&mut file)?;

        for url in &registry.urls.items {
            if url.id == id {
                return Ok(Some(url.clone()));
            }
        }

        Ok(None)
    }

    fn list_groups(&self) -> Result<Vec<String>, Box<dyn Error>> {
        let mut file = open_urls_file(self.file_path.as_str())?;
        let registry = read_urls(&mut file)?;

        let groups: Vec<&str> = registry
            .urls
            .items
            .iter()
            .map(|e: &URLRecord| e.group.as_str())
            .collect();

        let mut distinct: HashMap<&str, bool> = HashMap::new();

        for g in groups {
            distinct.insert(g, false);
        }

        Ok(distinct.keys().map(|k| k.to_string()).collect())
    }

    fn update(&self, id: &str, record: URLRecord) -> Result<Option<URLRecord>, Box<dyn Error>> {
        let mut file = open_urls_file(self.file_path.as_str())?;
        let mut registry = read_urls(&mut file)?;

        let mut found = false;
        for i in 0..registry.urls.items.len() {
            if is_same(&registry.urls.items[i], &record) {
                return Err(not_unique_error(&record));
            }

            if registry.urls.items[i].id.clone() == id {
                registry.urls.items[i] = record.clone();
                found = true
            }
        }
        if !found {
            return Ok(None);
        }

        write_urls(&mut file, registry)?;

        Ok(Some(record))
    }
}

impl RepositoryOld for FileStorage {
    fn list_v_0_0_x(&self, path: &str) -> Result<Vec<v0_0_x::URLRecord>, Box<dyn Error>> {
        let mut file = open_urls_file(path)?;
        let content: String = read_file(&mut file)?;

        let urls: v0_0_x::URLRegistry = if content != "" {
            serde_json::from_str(content.as_str())?
        } else {
            v0_0_x::URLRegistry {
                urls: v0_0_x::URLs { items: vec![] },
            }
        };

        Ok(urls.urls.items)
    }
}

fn is_unique(urls: &[URLRecord], record: &URLRecord) -> bool {
    for u in urls {
        if is_same(u, record) {
            return false;
        }
    }

    true
}

fn is_same(a: &URLRecord, b: &URLRecord) -> bool {
    a.name == b.name && a.group == b.group && a.id != b.id
}

fn open_urls_file(path: &str) -> Result<File, Box<dyn std::error::Error>> {
    let path = Path::new(path);

    if !path.exists() {
        if let Some(dir_path) = path.parent() {
            if !dir_path.exists() {
                fs::create_dir_all(dir_path)?;
            }
        };
    }

    match OpenOptions::new()
        .read(true)
        .create(true)
        .append(false)
        .write(true)
        .open(path)
    {
        Err(why) => Err(From::from(format!(
            "could not read URLs, failed to open file: {}",
            why
        ))),
        Ok(file) => Ok(file),
    }
}

fn read_file(file: &mut File) -> Result<String, Box<dyn std::error::Error>> {
    let mut content: String = String::new();

    match file.read_to_string(&mut content) {
        Err(why) => Err(From::from(why)),
        _ => Ok(content),
    }
}

fn read_urls(file: &mut File) -> Result<URLRegistry, Box<dyn std::error::Error>> {
    let content: String = read_file(file)?;

    let urls: URLRegistry = if content != "" {
        serde_json::from_str(content.as_str())?
    } else {
        URLRegistry {
            urls: URLs { items: vec![] },
        }
    };

    Ok(urls)
}

fn write_urls(
    file: &mut File,
    urls: URLRegistry,
) -> Result<URLRegistry, Box<dyn std::error::Error>> {
    let urls_json = serde_json::to_string(&urls)?;

    file.seek(SeekFrom::Start(0))?;
    file.write_all(urls_json.as_bytes())?;

    let desired_length: u64 = urls_json.len().try_into()?;
    file.set_len(desired_length)?;

    Ok(urls)
}

fn not_unique_error(record: &URLRecord) -> Box<dyn std::error::Error> {
    From::from(format!(
        "URL with name '{}' already exists in '{}' group",
        record.name.clone(),
        record.group.clone()
    ))
}
