use crate::types::URLRecord;

pub mod v0_0_x;

pub mod brave;

// TODO: Optionally I could sort by date?

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ImportItem {
    URL(ImportURLItem),
    Folder(ImportFolderItem)
    // visible: bool,
    // url: URLRecord,
    // row: Vec<String>,
}

impl Ord for ImportItem {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Sort Folders before URLs
        match (self, other) {
            (Self::URL(a), Self::URL(b)) => a.cmp(b),
            (Self::Folder(a), Self::Folder(b)) => a.cmp(b),
            (Self::URL(_), Self::Folder(_)) => std::cmp::Ordering::Greater,
            (Self::Folder(_), Self::URL(_)) => std::cmp::Ordering::Less,
        }
    }
}

impl PartialOrd for ImportItem {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    } 
}

impl ImportItem {
    pub fn new_url(id: String, url: String, name: String, parent_folder: String) -> Self {
        Self::URL(ImportURLItem::new(id, url, name, parent_folder))
    }

    pub fn new_folder(id: String, name: String, children: Vec<ImportItem>) -> Self {
        Self::Folder(ImportFolderItem::new(id, name, children))
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ImportURLItem {
    pub id: String,
    pub url: String,
    pub name: String,
    pub parent_folder: String,
}

impl Ord for ImportURLItem {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.name.cmp(&other.name)
    }
}

impl PartialOrd for ImportURLItem {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    } 
}

impl ImportURLItem {
    pub fn new(id: String, url: String, name: String, parent_folder: String) -> Self {
        Self {
            id,
            url,
            name,
            parent_folder,
        }
    }
}

impl From<ImportURLItem> for URLRecord {
    fn from(value: ImportURLItem) -> Self {
        // TODO: Do not generate UUID but take gid instead
        URLRecord::new(
            &value.url,
            &value.name,
            &value.parent_folder, // TODO: take from parent folder
            Vec::<String>::new(),
        ).set_id(&value.id)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ImportFolderItem {
    pub id: String,
    pub name: String,
    pub children: Vec<ImportItem>,
}

impl Ord for ImportFolderItem {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.name.cmp(&other.name)
    }
}

impl PartialOrd for ImportFolderItem {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    } 
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