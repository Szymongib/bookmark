use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use uuid::Uuid;

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

use sha1::{Sha1, Digest};
use std::str;
use std::hash::{Hash, Hasher};


impl URLRecord {
    pub fn new(url: &str, name: &str, group: &str, tags_vec: Vec<&str>) -> URLRecord {
        let mut tags: HashMap<String, bool> = HashMap::new();
        for t in tags_vec {
            tags.insert(t.to_string(), true);
        }
        // https://github.com/RustCrypto/hashes/blob/master/sha1/examples/sha1sum.rs


        // use sha1::{Sha1, Digest};
        //
        // let mut hasher = Sha1::new();
        // hasher.update(format!("{}/{}", group.clone(), name.clone()));
        //
        // let result = hasher.finalize();
        //
        // let result = str::from_utf8(result.as_slice())
        //     .expect("Failed to map hash to String")// TODO: handle error
        //     .to_string();

        URLRecord {
            id: result,
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
