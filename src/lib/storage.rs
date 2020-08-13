use super::types::{URLRecord, URLRegistry};
use crate::filters::Filter;
use crate::types::URLs;
use crate::Repository;
use std::collections::HashMap;
use std::convert::TryInto;
use std::error::Error;
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::Path;

pub struct FileStorage {
    file_path: String,
}

impl FileStorage {
    pub fn new_urls_repository(file_path: String) -> FileStorage {
        FileStorage { file_path }
    }
}

impl Repository for FileStorage {
    fn add(&self, record: URLRecord) -> Result<URLRecord, Box<dyn Error>> {
        let mut file = open_urls_file(self.file_path.as_str())?;
        let mut registry = read_urls(&mut file)?;

        if !is_unique(&registry.urls.items, &record) {
            return Err(From::from(format!(
                "URL with name {} already exists in {} group",
                record.name.clone(),
                record.group.clone()
            )));
        }

        registry.urls.items.push(record.clone());

        write_urls(&mut file, registry)?;

        return Ok(record);
    }

    fn delete(&self, name: &str, group: &str) -> Result<bool, Box<dyn std::error::Error>> {
        let mut file = open_urls_file(self.file_path.as_str())?;
        let mut registry = read_urls(&mut file)?;

        let mut index: usize = 0;
        for u in &registry.urls.items {
            if u.group == group && u.name == name {
                registry.urls.items.remove(index);
                write_urls(&mut file, registry)?;
                return Ok(true);
            }
            index += 1;
        }

        return Ok(false);
    }

    fn list(
        &self,
        group: Option<&str>,
        filter: &dyn Filter,
    ) -> Result<Vec<URLRecord>, Box<dyn std::error::Error>> {
        let mut file = open_urls_file(self.file_path.as_str())?;
        let registry = read_urls(&mut file)?;

        if group.is_none() {
            return Ok(filter.apply(registry.urls.items));
        }

        let group = group.unwrap();

        let urls = registry
            .urls
            .items
            .iter()
            .filter(|rec| rec.group == group)
            .map(|rec| rec.clone())
            .collect();

        return Ok(filter.apply(urls));
    }

    fn get(&self, id: String) -> Result<Option<URLRecord>, Box<dyn Error>> {
        let mut file = open_urls_file(self.file_path.as_str())?;
        let registry = read_urls(&mut file)?;

        for url in &registry.urls.items {
            if url.id == id {
                return Ok(Some(url.clone()))
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

        for g in &groups {
            distinct.insert(g.clone(), false);
        }

        Ok(distinct.keys().map(|k| k.to_string()).collect())
    }
}

fn is_unique(urls: &Vec<URLRecord>, record: &URLRecord) -> bool {
    for u in urls {
        if u.name == record.name && u.group == record.group {
            return false;
        }
    }

    return true;
}

fn open_urls_file(path: &str) -> Result<File, Box<dyn std::error::Error>> {
    let path = Path::new(path);

    if !path.exists() {
        match path.parent() {
            Some(dir_path) => {
                if !dir_path.exists() {
                    fs::create_dir_all(dir_path)?;
                }
            }
            None => {}
        };
    }

    return match OpenOptions::new()
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
    };
}

fn read_urls(file: &mut File) -> Result<URLRegistry, Box<dyn std::error::Error>> {
    let mut content: String = String::new();

    match file.read_to_string(&mut content) {
        Err(why) => return Err(From::from(why)),
        _ => {}
    };

    let mut urls: URLRegistry = URLRegistry {
        urls: URLs { items: vec![] },
    };
    if content != "" {
        urls = serde_json::from_str(content.as_str())?;
    }

    return Ok(urls);
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

    return Ok(urls);
}
