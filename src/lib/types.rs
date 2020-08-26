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

// TODO: consider refactoring to add some id or make name and group non whitespaced and treat group/name as uuid
// + add some description
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct URLRecord {
    pub id: String,
    pub url: String,
    pub name: String,
    pub group: String,
    pub tags: HashMap<String, bool>,
}

use hex;
use sha1::{Digest, Sha1};
use std::str;

impl URLRecord {
    pub fn new(url: &str, name: &str, group: &str, tags_vec: Vec<&str>) -> URLRecord {
        let mut tags: HashMap<String, bool> = HashMap::new();
        for t in tags_vec {
            tags.insert(t.to_string(), true);
        }

        URLRecord {
            id: calculate_hash(name.clone(), group.clone()),
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

pub(crate) fn calculate_hash(name: &str, group: &str) -> String {
    // TODO: consider using just uuid instead
    // TODO: decide if calculate hash earlier to compare earlier
    let mut hasher = Sha1::new();
    hasher.update(format!("{}/{}", group.clone(), name.clone()));
    return hex::encode(hasher.finalize());
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
