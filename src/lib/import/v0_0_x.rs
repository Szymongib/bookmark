use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

#[derive(Serialize, Deserialize)]
pub struct URLRegistry {
    pub urls: URLs,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct URLGroups {
    pub items: Vec<URLGroup>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct URLGroup {
    pub name: String,
}

impl URLGroup {
    pub fn new(name: String) -> URLGroup {
        URLGroup { name }
    }
}

#[derive(Serialize, Deserialize)]
pub struct URLs {
    pub items: Vec<URLRecord>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct URLRecord {
    pub url: String,
    pub name: String,
    pub group: String,
    pub tags: HashMap<String, bool>,
}

impl URLRecord {
    pub fn new(url: &str, name: &str, group: &str, tags_vec: Vec<&str>) -> URLRecord {
        let mut tags: HashMap<String, bool> = HashMap::new();
        for t in tags_vec {
            tags.insert(t.to_string(), true);
        }

        URLRecord {
            url: url.to_string(),
            name: name.to_string(),
            group: group.to_string(),
            tags,
        }
    }
}

impl fmt::Display for URLRecord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Name: {}, URL: {}, Group: {}, Tags: {:?}",
            self.name, self.url, self.group, self.tags
        )
    }
}
