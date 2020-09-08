use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
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
    pub id: String,
    pub url: String,
    pub name: String,
    pub group: String,
    pub tags: BTreeMap<String, bool>,
}

impl URLRecord {
    pub fn new(url: &str, name: &str, group: &str, tags_vec: Vec<&str>) -> URLRecord {
        let mut tags: BTreeMap<String, bool> = BTreeMap::new();
        for t in tags_vec {
            tags.insert(t.to_string(), true);
        }

        let random_bytes = rand::thread_rng().gen::<[u8; 8]>();

        URLRecord {
            id: hex::encode(random_bytes),
            url: url.to_string(),
            name: name.to_string(),
            group: group.to_string(),
            tags,
        }
    }

    pub fn tags_as_string(&self) -> String {
        let tags: Vec<&str> = self.tags.keys().map(|k| k.as_str()).collect();
        tags.join(", ")
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
