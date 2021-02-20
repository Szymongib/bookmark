use crate::types::URLRecord;
use std::cmp::Ordering;
use std::str::FromStr;

// TODO: Consider adding some option to sort without applying `to_lowercase`?
#[derive(Copy, Clone)]
pub struct SortConfig {
    sort_by: SortBy,
    order: SortOrder,
}

impl SortConfig {
    pub fn new(sort_by: SortBy, order: SortOrder) -> SortConfig {
        SortConfig { sort_by, order }
    }

    pub fn new_by(sort_by: SortBy) -> SortConfig {
        SortConfig {
            sort_by,
            order: SortOrder::Ascending,
        }
    }
}

#[derive(Copy, Clone)]
pub enum SortBy {
    Name,
    URL,
    Group,
}

impl FromStr for SortBy {
    type Err = Box<dyn std::error::Error>;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.to_lowercase().as_str() {
            "name" => Ok(SortBy::Name),
            "url" => Ok(SortBy::URL),
            "group" => Ok(SortBy::Group),
            _ => Err(From::from(
                "invalid sort column, must be one of: [name, url, group]",
            )),
        }
    }
}

#[derive(Copy, Clone)]
pub enum SortOrder {
    Ascending,
    Descending,
}

pub(crate) fn sort_urls(mut urls: Vec<URLRecord>, config: &SortConfig) -> Vec<URLRecord> {
    let cmp_func = match config.sort_by {
        SortBy::Name => sort_by_name,
        SortBy::URL => sort_by_url,
        SortBy::Group => sort_by_group,
    };

    urls.sort_by(cmp_func);

    match config.order {
        SortOrder::Ascending => {}
        SortOrder::Descending => urls.reverse(),
    };

    urls
}

fn sort_by_name(a: &URLRecord, b: &URLRecord) -> Ordering {
    a.name.to_lowercase().cmp(&b.name.to_lowercase())
}

fn sort_by_group(a: &URLRecord, b: &URLRecord) -> Ordering {
    a.group.to_lowercase().cmp(&b.group.to_lowercase())
}

fn sort_by_url(a: &URLRecord, b: &URLRecord) -> Ordering {
    strip_protocol(&a.url.to_lowercase()).cmp(&strip_protocol(&b.url.to_lowercase()))
}

fn strip_protocol(url: &str) -> String {
    let possible_prefix = &["https://www.", "http://www.", "https://", "http://"];
    for prefix in possible_prefix {
        if let Some(u) = url.strip_prefix(prefix) {
            return u.to_string();
        }
    }

    url.to_string()
}

#[cfg(test)]
mod test {
    use crate::sort::{sort_urls, SortBy, SortConfig, SortOrder};
    use crate::types::URLRecord;

    fn fix_url_records() -> Vec<URLRecord> {
        vec![
            URLRecord::new("http://abcd", "one", "one", vec!["tag", "with space"]),
            URLRecord::new("https://aaaa", "Two", "two", Vec::<String>::new()),
            URLRecord::new("http://www.ccc", "three", "one", Vec::<String>::new()),
            URLRecord::new("https://www.cbbc", "FOUR", "abcd", vec!["tag"]),
            URLRecord::new("baobab", "five", "GROUP", Vec::<String>::new()),
            URLRecord::new("http://xyz", "six", "one", Vec::<String>::new()),
            URLRecord::new(
                "http://YellowSubmarine",
                "seven",
                "GROUP",
                Vec::<String>::new(),
            ),
        ]
    }

    #[test]
    fn test_sort_by_url() {
        let mut records = fix_url_records();

        let sort_cfg = SortConfig::new(SortBy::URL, SortOrder::Ascending);
        records = sort_urls(records, &sort_cfg);

        let expected_order = &mut [
            "https://aaaa",
            "http://abcd",
            "baobab",
            "https://www.cbbc",
            "http://www.ccc",
            "http://xyz",
            "http://YellowSubmarine",
        ];

        for (i, r) in records.iter().enumerate() {
            assert_eq!(r.url, expected_order[i])
        }

        // Descending
        let sort_cfg = SortConfig::new(SortBy::URL, SortOrder::Descending);
        records = sort_urls(records, &sort_cfg);

        expected_order.reverse();
        for (i, r) in records.iter().enumerate() {
            assert_eq!(r.url, expected_order[i])
        }
    }

    #[test]
    fn test_sort_by_name() {
        let mut records = fix_url_records();

        let sort_cfg = SortConfig::new(SortBy::Name, SortOrder::Ascending);
        records = sort_urls(records, &sort_cfg);

        let expected_order = &mut ["five", "FOUR", "one", "seven", "six", "three", "Two"];

        for (i, r) in records.iter().enumerate() {
            assert_eq!(r.name, expected_order[i])
        }

        // Descending
        let sort_cfg = SortConfig::new(SortBy::Name, SortOrder::Descending);
        records = sort_urls(records, &sort_cfg);

        expected_order.reverse();
        for (i, r) in records.iter().enumerate() {
            assert_eq!(r.name, expected_order[i])
        }
    }

    #[test]
    fn test_sort_by_group() {
        let mut records = fix_url_records();

        let sort_cfg = SortConfig::new(SortBy::Group, SortOrder::Ascending);
        records = sort_urls(records, &sort_cfg);

        let expected_order = &mut ["abcd", "GROUP", "GROUP", "one", "one", "one", "two"];

        for (i, r) in records.iter().enumerate() {
            assert_eq!(r.group, expected_order[i])
        }

        // Descending
        let sort_cfg = SortConfig::new(SortBy::Group, SortOrder::Descending);
        records = sort_urls(records, &sort_cfg);

        expected_order.reverse();
        for (i, r) in records.iter().enumerate() {
            assert_eq!(r.group, expected_order[i])
        }
    }
}
