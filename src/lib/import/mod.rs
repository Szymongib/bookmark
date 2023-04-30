use crate::types::URLRecord;

pub mod v0_0_x;

pub mod brave;


#[derive(Clone, Debug)]
pub enum ImportItem {
    URL(ImportURLItem),
    Folder(ImportFolderItem)
    // visible: bool,
    // url: URLRecord,
    // row: Vec<String>,
}

impl ImportItem {
    pub fn new_url(id: String, url: String, name: String) -> Self {
        Self::URL(ImportURLItem::new(id, url, name))
    }

    pub fn new_folder(id: String, name: String, children: Vec<ImportItem>) -> Self {
        Self::Folder(ImportFolderItem::new(id, name, children))
    }
}

#[derive(Clone, Debug)]
pub struct ImportURLItem {
    pub id: String,
    pub url: String,
    pub name: String,
}

impl ImportURLItem {
    pub fn new(id: String, url: String, name: String) -> Self {
        Self {
            id,
            url,
            name,
        }
    }
}

#[derive(Clone, Debug)]
pub struct ImportFolderItem {
    pub id: String,
    pub name: String,
    pub children: Vec<ImportItem>,
}

impl ImportFolderItem {
    pub fn new(id: String, name: String, children: Vec<ImportItem>) -> Self {
        Self {
            id,
            name,
            children,
        }
    }
}