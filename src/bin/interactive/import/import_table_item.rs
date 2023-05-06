use crate::interactive::table::TableItem;
use bookmark_lib::types::URLRecord;

use bookmark_lib::import::{ImportItem, ImportURLItem, ImportFolderItem};

// pub const DEFAULT_URL_COLS: [&str; 4] = ["  Name", "URL", "Group", "Tags", "Imported"];

// pub type Columns = Vec<String>;

// pub fn default_columns() -> Columns {
//     (&DEFAULT_URL_COLS)
//         .iter()
//         .map(|s| s.to_string())
//         .collect::<Columns>()
// }

#[derive(Clone, Debug)]
pub enum ImportTableItem {
    URL(ImportURLTableItem),
    Folder(ImportFolderTableItem)
}

impl From<ImportItem> for ImportTableItem {
    fn from(item: ImportItem) -> Self {
        match item {
            ImportItem::URL(url) => Self::URL(url.into()),
            ImportItem::Folder(folder) => Self::Folder(folder.into()),
        }
    }
}

impl ImportTableItem {
    pub fn id(&self) -> String {
        match self {
            Self::URL(url) => url.id(),
            Self::Folder(folder) => folder.id(),
        }
    }

    pub fn select(&mut self, selected: bool) {
        match self {
            Self::URL(url) => url.select(selected),
            Self::Folder(folder) => folder.select(selected),
        }
    }

    pub fn inner(&self) -> ImportItem {
        match self {
            Self::URL(url) => ImportItem::URL(url.inner.clone()),
            Self::Folder(folder) => ImportItem::Folder(folder.inner.clone()),
        }
    }
    
    // pub fn inner_mut(&mut self) -> &mut ImportItem {
    //     match self {
    //         Self::URL(url) => ImportItem::URL(url.inner.clone()),
    //         Self::Folder(folder) => ImportItem::Folder(folder.inner.clone()),
    //     }
    // }
}

#[derive(Clone, Debug)]
pub struct ImportURLTableItem {
    pub inner: ImportURLItem,
    row: Vec<String>,
    selected: bool,
}

impl From<ImportURLItem> for ImportURLTableItem {
    fn from(item: ImportURLItem) -> Self {
        Self {
            inner: item.clone(),
            row: vec!["URL".to_string(), item.name, item.url, "[ ]".to_string()],
            selected: false,
        }
    }
}

impl ImportURLTableItem {
    fn id(&self) -> String {
        self.inner.id.clone()
    }

    fn row(&self) -> &Vec<String> {
        &self.row
    }

    pub fn select(&mut self, selected: bool) {
        self.selected = selected;
        self.row[3] = if self.selected { "[x]".to_string() } else { "[ ]".to_string() };
    }

    pub fn refresh(&mut self) {
        self.row[1] = self.inner.name.clone();
        self.row[2] = self.inner.url.clone();
    }
}

#[derive(Clone, Debug)]
pub struct ImportFolderTableItem {
    pub inner: ImportFolderItem,

    row: Vec<String>,
    selected: bool,
}

impl From<ImportFolderItem> for ImportFolderTableItem{
    fn from(item: ImportFolderItem) -> Self {
        Self {
            inner: item.clone(),
            row: vec!["Folder".to_string(), item.name, "-".to_string(), "[ ]".to_string()],
            selected: false,
        }
    }
}

impl ImportFolderTableItem {
    fn id(&self) -> String {
        self.inner.id.clone()
    }

    fn row(&self) -> &Vec<String> {
        &self.row
    }

    pub fn select(&mut self, selected: bool) {
        self.selected = selected;
        self.row[3] = if self.selected { "[x]".to_string() } else { "[ ]".to_string() };
    }
}

impl TableItem for ImportTableItem {
    fn row(&self) -> &Vec<String> {
        match self {
            ImportTableItem::URL(url) => url.row(),
            ImportTableItem::Folder(folder) => folder.row(),
        }
    }

    fn id(&self) -> String {
        self.id()
    }
}



// #[cfg(test)]
// mod test {
//     use crate::interactive::helpers::to_string;
//     use crate::interactive::table::TableItem;
//     use crate::interactive::url_table_item::{default_columns, Columns, URLItem};
//     use bookmark_lib::types::URLRecord;

//     struct TestCase<'a> {
//         url_record: URLRecord,
//         columns: Option<&'a Columns>,
//         expected_row: Vec<String>,
//     }

//     #[test]
//     fn test_url_item() {
//         let record = URLRecord::new("url1", "name1", "group1", vec!["tag1", "tag1.2"]);
//         let cols = to_string(vec!["  ID", "Name", "  Tags   "]);

//         let items = vec![
//             TestCase {
//                 url_record: record.clone(),
//                 expected_row: to_string(vec!["name1", "url1", "group1", "tag1, tag1.2"]),
//                 columns: None,
//             },
//             TestCase {
//                 url_record: record.clone(),
//                 expected_row: to_string(vec![&record.id, "name1", "tag1, tag1.2"]),
//                 columns: Some(&cols),
//             },
//             TestCase {
//                 url_record: URLRecord::new("url2", "name2", "group2", vec!["tag2", "tag2.2"]),
//                 expected_row: to_string(vec!["name2", "url2", "group2", "tag2, tag2.2"]),
//                 columns: None,
//             },
//             TestCase {
//                 url_record: URLRecord::new("url3", "name3", "group3", vec!["tag3", "tag3.2"]),
//                 expected_row: to_string(vec!["name3", "url3", "group3", "tag3, tag3.2"]),
//                 columns: None,
//             },
//             TestCase {
//                 url_record: URLRecord::new("url4", "name4", "group4", Vec::<String>::new()),
//                 expected_row: to_string(vec!["name4", "url4", "group4", ""]),
//                 columns: None,
//             },
//             TestCase {
//                 url_record: URLRecord::new("url5", "name5", "group5", vec!["tag", "with space"]),
//                 expected_row: to_string(vec!["name5", "url5", "group5", "tag, \"with space\""]),
//                 columns: None,
//             },
//         ];

//         for item in items {
//             let table_item = URLItem::new(item.url_record, item.columns.clone());
//             let row = table_item.row();
//             assert_eq!(&item.expected_row, row);
//         }
//     }

//     #[test]
//     fn test_default_columns() {
//         let def_cols = default_columns();

//         let expected_cols = vec![
//             "  Name".to_string(),
//             "URL".to_string(),
//             "Group".to_string(),
//             "Tags".to_string(),
//         ];

//         assert_eq!(def_cols, expected_cols)
//     }
// }
