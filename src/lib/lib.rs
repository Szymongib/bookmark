use crate::filters::Filter;
use crate::import::v0_0_x;
use crate::types::URLRecord;

pub mod filters;
pub mod registry;
pub mod storage;
pub mod types;

pub mod import;

mod util;

pub trait Registry: RegistryReader + Importer {
    fn new(
        &self,
        name: &str,
        url: &str,
        group: Option<&str>,
        tags: Vec<&str>,
    ) -> Result<URLRecord, Box<dyn std::error::Error>>;

    fn add_url(&self, record: URLRecord) -> Result<URLRecord, Box<dyn std::error::Error>>;

    fn delete_by_id(&self, id: &str) -> Result<bool, Box<dyn std::error::Error>>;

    fn list_groups(&self) -> Result<Vec<String>, Box<dyn std::error::Error>>;

    fn tag_url(&self, id: &str, tag: &str)
        -> Result<Option<URLRecord>, Box<dyn std::error::Error>>;
}

pub trait RegistryReader {
    fn list_urls(
        &self,
        filter: Option<&Box<dyn Filter>>,
    ) -> Result<Vec<URLRecord>, Box<dyn std::error::Error>>;

    fn get_url(&self, id: &str) -> Result<Option<URLRecord>, Box<dyn std::error::Error>>;
}

pub trait Repository: RepositoryOld {
    fn add(&self, record: URLRecord) -> Result<URLRecord, Box<dyn std::error::Error>>;
    fn add_batch(
        &self,
        record: Vec<URLRecord>,
    ) -> Result<Vec<URLRecord>, Box<dyn std::error::Error>>;
    fn delete_by_id(&self, id: &str) -> Result<bool, Box<dyn std::error::Error>>;
    fn list(&self) -> Result<Vec<URLRecord>, Box<dyn std::error::Error>>;
    fn get(&self, id: &str) -> Result<Option<URLRecord>, Box<dyn std::error::Error>>;
    fn list_groups(&self) -> Result<Vec<String>, Box<dyn std::error::Error>>;
    fn update(
        &self,
        id: &str,
        record: URLRecord,
    ) -> Result<Option<URLRecord>, Box<dyn std::error::Error>>;
}

pub trait RepositoryOld {
    fn list_v_0_0_x(
        &self,
        path: &str,
    ) -> Result<Vec<v0_0_x::URLRecord>, Box<dyn std::error::Error>>;
}

pub trait Importer {
    fn import_from_v_0_0_x(&self, path: &str)
        -> Result<Vec<URLRecord>, Box<dyn std::error::Error>>;
}
