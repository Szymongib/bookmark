use crate::types::URLRecord;
use crate::filters::Filter;

pub mod filters;
pub mod registry;
pub mod storage;
pub mod types;

mod util;

pub trait Registry: RegistryReader {
    fn new(
        &self,
        name: &str,
        url: &str,
        group: Option<&str>,
        tags: Vec<&str>,
    ) -> Result<URLRecord, Box<dyn std::error::Error>>;

    fn add_url(
        &self,
        record: URLRecord,
    ) -> Result<URLRecord, Box<dyn std::error::Error>>;

    fn delete_by_id(
        &self,
        id: &str
    ) -> Result<bool, Box<dyn std::error::Error>>;

    fn list_groups(&self) -> Result<Vec<String>, Box<dyn std::error::Error>>;


    fn tag_url(&self, id: &str, tag: &str) -> Result<Option<URLRecord>, Box<dyn std::error::Error>>;
}

pub trait RegistryReader {
    fn list_urls(&self, filter: Option<&Box<dyn Filter>>) -> Result<Vec<URLRecord>, Box<dyn std::error::Error>>;

    fn get_url(&self, id: &str) -> Result<Option<URLRecord>, Box<dyn std::error::Error>>;
}

pub trait Repository {
    fn add(&self, record: URLRecord) -> Result<URLRecord, Box<dyn std::error::Error>>;
    fn delete_by_id(&self, id: &str) -> Result<bool, Box<dyn std::error::Error>>;
    fn list(&self) -> Result<Vec<URLRecord>, Box<dyn std::error::Error>>;
    fn get(&self, id: &str) -> Result<Option<URLRecord>, Box<dyn std::error::Error>>;
    fn list_groups(&self) -> Result<Vec<String>, Box<dyn std::error::Error>>;
    fn update(&self, id: &str, record: URLRecord) -> Result<Option<URLRecord>, Box<dyn std::error::Error>>;
}
