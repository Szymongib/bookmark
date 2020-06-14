use crate::types::URLRecord;

// TODO: introduce combine method
pub trait Filter {
    fn apply(&self, urls: Vec<URLRecord>) -> Vec<URLRecord>;
}

pub struct ListFilter {
    tags: Vec<String>,
}

impl ListFilter {
    pub fn new_tags_filter(tags: Vec<&str>) -> ListFilter {
        ListFilter {
            tags: tags.iter().map(|t| t.to_string()).collect(),
        }
    }
}

impl Filter for ListFilter {
    fn apply(&self, urls: Vec<URLRecord>) -> Vec<URLRecord> {
        urls.iter()
            .filter(|url| {
                for t in &self.tags {
                    if url.tags.contains_key(t.as_str()) {
                        return true;
                    }
                }
                return false;
            })
            .map(|url| (*url).clone())
            .collect()
    }
}

pub struct NoopFilter {}

impl NoopFilter {
    pub fn new() -> NoopFilter {
        NoopFilter {}
    }
}

impl Filter for NoopFilter {
    fn apply(&self, urls: Vec<URLRecord>) -> Vec<URLRecord> {
        urls
    }
}
