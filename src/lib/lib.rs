use crate::filters::Filter;
use crate::types::URLRecord;

pub mod filters;
pub mod registry;
pub mod storage;
pub mod types;

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
