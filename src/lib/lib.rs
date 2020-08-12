use crate::filters::Filter;
use crate::types::URLRecord;

pub mod filters;
pub mod record_filter;
pub mod registry;
pub mod storage;
pub mod types;

pub trait Registry {
    fn add_url(
        &self,
        name: &str,
        url: &str,
        group: Option<&str>,
        tags: Vec<&str>,
    ) -> Result<URLRecord, Box<dyn std::error::Error>>;

    fn delete(
        &self,
        name: &str,
        group: Option<&str>,
    ) -> Result<bool, Box<dyn std::error::Error>>;

    fn list_groups(&self) -> Result<Vec<String>, Box<dyn std::error::Error>>;

    fn list_urls(
        &self,
        group: Option<&str>,
        tags: Option<Vec<&str>>,
    ) -> Result<Vec<URLRecord>, Box<dyn std::error::Error>>;
}

pub trait Repository {
    fn add(&self, record: URLRecord) -> Result<URLRecord, Box<dyn std::error::Error>>;
    fn delete(&self, name: &str, group: &str) -> Result<bool, Box<dyn std::error::Error>>;
    fn list(
        &self,
        group: Option<&str>,
        filter: &dyn Filter,
    ) -> Result<Vec<URLRecord>, Box<dyn std::error::Error>>; // TODO: consider extracting group as filter
    fn list_groups(&self) -> Result<Vec<String>, Box<dyn std::error::Error>>;
}
