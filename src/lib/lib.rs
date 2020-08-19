use crate::filters::Filter;
use crate::types::URLRecord;

pub mod filters;
pub mod record_filter;
pub mod registry;
pub mod storage;
pub mod types;

mod util;

pub trait Registry {
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

    fn delete_url(
        &self,
        name: &str,
        group: Option<&str>,
    ) -> Result<bool, Box<dyn std::error::Error>>;

    fn delete_by_id(
        &self,
        id: &str
    ) -> Result<bool, Box<dyn std::error::Error>>;

    fn list_groups(&self) -> Result<Vec<String>, Box<dyn std::error::Error>>;

    fn list_urls(
        &self,
        group: Option<&str>,
        tags: Option<Vec<&str>>,
    ) -> Result<Vec<URLRecord>, Box<dyn std::error::Error>>;

    fn get_url(&self, id: String) -> Result<Option<URLRecord>, Box<dyn std::error::Error>>;

    fn tag_url(&self, id: String, tag: String) -> Result<Option<URLRecord>, Box<dyn std::error::Error>>;
}

pub trait Repository {
    fn add(&self, record: URLRecord) -> Result<URLRecord, Box<dyn std::error::Error>>;
    fn delete(&self, name: &str, group: &str) -> Result<bool, Box<dyn std::error::Error>>;
    fn delete_by_id(&self, id: &str) -> Result<bool, Box<dyn std::error::Error>>;
    fn list(
        &self,
        group: Option<&str>,
        filter: &dyn Filter,
    ) -> Result<Vec<URLRecord>, Box<dyn std::error::Error>>; // TODO: consider extracting group as filter
    fn get(&self, id: String) -> Result<Option<URLRecord>, Box<dyn std::error::Error>>;
    fn list_groups(&self) -> Result<Vec<String>, Box<dyn std::error::Error>>;
    fn update(&self, id: String, record: URLRecord) -> Result<Option<URLRecord>, Box<dyn std::error::Error>>;
}
