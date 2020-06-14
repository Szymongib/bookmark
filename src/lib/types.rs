use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct URLRegistry {
    pub urls: URLs,
}

#[derive(Serialize, Deserialize)]
pub struct URLGroups {
    pub items: Vec<URLGroup>,
}

#[derive(Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize, Clone)]
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
