use bookmark_lib::types::URLRecord;
use crate::interactive::table::{TableItem};

#[derive(Clone, Debug)]
pub struct URLItem {
    visible: bool,
    url: URLRecord,
    row: Vec<String>,
}

impl URLItem {
    pub fn new(record: URLRecord) -> URLItem {
        URLItem {
            url: record.clone(),
            visible: true,
            row: url_to_row(&record),
        }
    }

    pub fn from_vec(records: Vec<URLRecord>) -> Vec<URLItem> {
        records.iter().map(|u| URLItem::new(u.clone())).collect()
    }

    pub fn url(&self) -> String {
        return self.url.url.clone();
    }
}

impl TableItem for URLItem {
    fn row(&self) -> &Vec<String> {
        &self.row
    }

    fn id(&self) -> String {
        self.url.id.clone()
    }
}

fn url_to_row(record: &URLRecord) -> Vec<String> {
    vec![
        record.name.clone(),
        record.url.clone(),
        record.group.clone(),
        record.tags_as_string(),
    ]
}

#[cfg(test)]
mod test {
    use crate::interactive::table::TableItem;
    use crate::interactive::url_table_item::URLItem;
    use bookmark_lib::filters::Filter;
    use bookmark_lib::types::URLRecord;

    struct TestCase {
        url_record: URLRecord,
        expected_row: Vec<String>,
    }

    struct FixedFilter {
        matches: bool,
    }

    impl FixedFilter {
        fn new(matches: bool) -> FixedFilter {
            FixedFilter { matches }
        }
    }

    impl Filter for FixedFilter {
        fn matches(&self, _: &URLRecord) -> bool {
            return self.matches;
        }
    }

    #[test]
    fn test_url_item() {
        let match_filter = FixedFilter::new(true);
        let do_not_match_filter = FixedFilter::new(false);

        let items = vec![
            TestCase {
                url_record: URLRecord::new("url1", "name1", "group1", vec!["tag1, tag1.2"]),
                expected_row: vec![
                    "name1".to_string(),
                    "url1".to_string(),
                    "group1".to_string(),
                    "tag1, tag1.2".to_string(),
                ],
            },
            TestCase {
                url_record: URLRecord::new("url2", "name2", "group2", vec!["tag2, tag2.2"]),
                expected_row: vec![
                    "name2".to_string(),
                    "url2".to_string(),
                    "group2".to_string(),
                    "tag2, tag2.2".to_string(),
                ],
            },
            TestCase {
                url_record: URLRecord::new("url3", "name3", "group3", vec!["tag3, tag3.2"]),
                expected_row: vec![
                    "name3".to_string(),
                    "url3".to_string(),
                    "group3".to_string(),
                    "tag3, tag3.2".to_string(),
                ],
            },
            TestCase {
                url_record: URLRecord::new("url4", "name4", "group4", vec![]),
                expected_row: vec![
                    "name4".to_string(),
                    "url4".to_string(),
                    "group4".to_string(),
                    "".to_string(),
                ],
            },
        ];

        for item in items {
            let mut table_item = URLItem::new(item.url_record);
            let row = table_item.row();
            assert_eq!(&item.expected_row, row);
        }
    }
}
