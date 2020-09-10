use crate::interactive::table::TableItem;
use bookmark_lib::types::URLRecord;

pub const DEFAULT_URL_COLS: [&str; 4] = ["  Name", "URL", "Group", "Tags"];

pub type Columns = Vec<String>;

pub fn default_columns() -> Columns {
    (&DEFAULT_URL_COLS)
        .iter()
        .map(|s| s.to_string())
        .collect::<Columns>()
}

#[derive(Clone, Debug)]
pub struct URLItem {
    visible: bool,
    url: URLRecord,
    row: Vec<String>,
}

impl URLItem {
    pub fn new(record: URLRecord, cols: Option<&Columns>) -> URLItem {
        URLItem {
            url: record.clone(),
            visible: true,
            row: url_to_row(
                &record,
                cols.unwrap_or(
                    &DEFAULT_URL_COLS
                        .iter()
                        .map(|s| s.to_string())
                        .collect::<Columns>(),
                ),
            ),
        }
    }

    pub fn from_vec(records: Vec<URLRecord>, cols: Option<&Columns>) -> Vec<URLItem> {
        records
            .iter()
            .map(|u| URLItem::new(u.clone(), cols))
            .collect()
    }

    pub fn url(&self) -> String {
        self.url.url.clone()
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

fn url_to_row(record: &URLRecord, cols: &Columns) -> Vec<String> {
    let mut vals = vec![];

    for c in cols {
        let col_name = c.trim().to_lowercase();

        match col_name.as_str() {
            "id" => vals.push(record.id.clone()),
            "name" => vals.push(record.name.clone()),
            "url" => vals.push(record.url.clone()),
            "group" => vals.push(record.group.clone()),
            "tags" => vals.push(record.tags_as_string()),
            _ => {}
        }
    }

    vals
}

#[cfg(test)]
mod test {
    use crate::interactive::helpers::to_string;
    use crate::interactive::table::TableItem;
    use crate::interactive::url_table_item::{default_columns, Columns, URLItem};
    use bookmark_lib::types::URLRecord;

    struct TestCase<'a> {
        url_record: URLRecord,
        columns: Option<&'a Columns>,
        expected_row: Vec<String>,
    }

    #[test]
    fn test_url_item() {
        let record = URLRecord::new("url1", "name1", "group1", vec!["tag1", "tag1.2"]);
        let cols = to_string(vec!["  ID", "Name", "  Tags   "]);

        let items = vec![
            TestCase {
                url_record: record.clone(),
                expected_row: to_string(vec!["name1", "url1", "group1", "tag1, tag1.2"]),
                columns: None,
            },
            TestCase {
                url_record: record.clone(),
                expected_row: to_string(vec![&record.id, "name1", "tag1, tag1.2"]),
                columns: Some(&cols),
            },
            TestCase {
                url_record: URLRecord::new("url2", "name2", "group2", vec!["tag2", "tag2.2"]),
                expected_row: to_string(vec!["name2", "url2", "group2", "tag2, tag2.2"]),
                columns: None,
            },
            TestCase {
                url_record: URLRecord::new("url3", "name3", "group3", vec!["tag3", "tag3.2"]),
                expected_row: to_string(vec!["name3", "url3", "group3", "tag3, tag3.2"]),
                columns: None,
            },
            TestCase {
                url_record: URLRecord::new("url4", "name4", "group4", vec![]),
                expected_row: to_string(vec!["name4", "url4", "group4", ""]),
                columns: None,
            },
            TestCase {
                url_record: URLRecord::new("url5", "name5", "group5", vec!["tag", "with space"]),
                expected_row: to_string(vec!["name5", "url5", "group5", "tag, \"with space\""]),
                columns: None,
            },
        ];

        for item in items {
            let table_item = URLItem::new(item.url_record, item.columns.clone());
            let row = table_item.row();
            assert_eq!(&item.expected_row, row);
        }
    }

    #[test]
    fn test_default_columns() {
        let def_cols = default_columns();

        let expected_cols = vec![
            "  Name".to_string(),
            "URL".to_string(),
            "Group".to_string(),
            "Tags".to_string(),
        ];

        assert_eq!(def_cols, expected_cols)
    }
}
