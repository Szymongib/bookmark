
use std::collections::HashMap;
use std::convert::{TryFrom, TryInto};
use serde::{Serialize, Deserialize};

use super::BraveImportError;

const BOOKMARK_TYPE_URL: &str = "url";
const BOOKMARK_TYPE_FOLDER: &str = "folder";

const BOOKMARK_BAR_ROOT: &str = "bookmark_bar";

/// Since roots are not really a folder (they lack all metadata) we flatten
/// all of them to a single Vec.
pub type Bookmarks = Vec<BookmarkEntry>;
pub type ParsedBookmarks = Vec<Result<BookmarkEntry, BraveImportError>>;

// fn bookmarks_to_imports(bookmarks: Bookmarks) -> Vec<ImportTree> {
//     let imports = bookmarks.iter().map(|(folder, entry)| {
//         match entry {
//             BookmarkEntry::URL(url) => {
//                 ImportTree::URL(url.url.clone(), url.name.clone(), folder.clone())
//             }
//             BookmarkEntry::Folder(folder) => {
//                 let children = bookmarks_to_imports(folder.children.clone());
//                 ImportTree::Folder(folder.name.clone(), children)
//             }
//         }
//     }).collect();

//     imports
// }

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BookmarksData {
    pub checksum: String,
    pub roots: HashMap<String, RawEntry>,
}

impl BookmarksData {
    pub fn bookmarks(self) -> ParsedBookmarks {
        self.roots.into_iter().map(|(k, v)| {
            BookmarkEntry::try_from(v)
        }).collect()
    }
}

pub fn filter_bookmarks(parsed: ParsedBookmarks) -> (Bookmarks, Vec<BraveImportError>) {
    // Most bookmarks will be valid, therefore we assume such capacity.
    let mut valid_bookmarks = Vec::with_capacity(parsed.len());
    let mut bookmark_errs = vec![];

    for entry in parsed {
        match entry {
            Ok(entry) => {
                valid_bookmarks.push(entry);
            }
            Err(err) => {
                bookmark_errs.push(err)
            }
        }
    }

    (valid_bookmarks, bookmark_errs)
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BookmarkEntry {
    URL(BookmarkURL),
    Folder(BookmarkFolder),
}

impl TryFrom<RawEntry> for BookmarkEntry {
    type Error = BraveImportError;

    fn try_from(value: RawEntry) -> Result<Self, Self::Error> {
        match value.r_type.as_str() {
            BOOKMARK_TYPE_URL => Ok(BookmarkEntry::URL(value.into())),
            BOOKMARK_TYPE_FOLDER => Ok(BookmarkEntry::Folder(value.try_into()?)),
            t => Err(BraveImportError::UnexpectedEntryType(t.to_string()))
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RawEntry {
    pub date_added: String,
    pub date_modified: Option<String>,
    pub guid: String,
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub r_type: String,
    pub children: Option<Vec<RawEntry>>,
    pub url: Option<String>,
}


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BookmarkURL {
    pub date_added: String,
    pub guid: String,
    pub id: String,
    pub name: String,
    pub url: String,
}

impl From<RawEntry> for BookmarkURL {
    fn from(e: RawEntry) -> Self {
        Self{
            date_added: e.date_added,
            guid: e.guid,
            id: e.id,
            name: e.name,
            url: e.url.unwrap_or_default(),
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BookmarkFolder {
    pub date_added: String,
    pub date_modified: Option<String>,
    pub guid: String,
    pub id: String,
    pub name: String,
    pub children: Vec<BookmarkEntry>,
}

impl TryFrom<RawEntry> for BookmarkFolder {
    type Error = BraveImportError;

    fn try_from(e: RawEntry) -> Result<Self, Self::Error> {
        Ok(Self{
            date_added: e.date_added,
            date_modified: e.date_modified,
            guid: e.guid,
            id: e.id,
            name: e.name,
            children: e.children.map(|c| {
                c.into_iter().map(|e| e.try_into()).collect::<Result<Vec<BookmarkEntry>, BraveImportError>>()
            }).unwrap_or(Ok(vec![]))?,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::import::brave::bookmarks::BOOKMARK_BAR_ROOT;

    use super::{BookmarkEntry, BookmarkFolder, BookmarkURL, };

    

    const BOOKMARKS_DATA: &str = r#"{
  "checksum": "3d97c44a0cff7da12dca4f0f25d2510e",
  "roots": {
    "bookmark_bar": {
      "children": [
        {
          "date_added": "13280334723204976",
          "guid": "5cb6de11-58fe-4f8c-958d-988c4c542730",
          "id": "990",
          "name": "Bookmark In folder",
          "type": "url",
          "url": "https://bookmark.in.folder"
        },
        {
          "children": [
            {
              "date_added": "13280334723204303",
              "date_modified": "13280334723215380",
              "guid": "3cd87a84-8623-48c8-ab73-920276f631d5",
              "id": "985",
              "name": "Nested Folder",
              "type": "folder",
              "children": [
                {
                  "date_added": "13280334723204976",
                  "guid": "5cb6de11-58fe-4f8c-958d-988c4c542730",
                  "id": "991",
                  "name": "Bookmark In folder In folder",
                  "type": "url",
                  "url": "https://nested.com"
                },
                {
                  "date_added": "13280334723204976",
                  "guid": "5cb6de11-58fe-4f8c-958d-988c4c542731",
                  "id": "992",
                  "name": "Bookmark In folder In folder2",
                  "type": "url",
                  "url": "https://nested2.com"
                }
              ]
            }
          ],
          "date_added": "13292869307359560",
          "date_modified": "13292869307359560",
          "guid": "4a1fb38f-0e4a-4b6b-a718-5dbc5f3a2ea7",
          "id": "1502",
          "name": "Folder1",
          "type": "folder"
        }
      ],
        "date_added": "13292859307359560",
        "date_modified": "13292869307359560",
        "guid": "4a1fb38f-0e4a-4b6b-a718-4dbc5f3a2ea7",
        "id": "1",
        "name": "Bookmarks Bar",
        "type": "folder"
    },
    "other": {
      "date_added": "13292869307359561",
      "date_modified": "13292869307359563",
      "guid": "4a1fb38f-0e4a-4b6b-a718-5dbc5f3a2ea8",
      "id": "1505",
      "name": "Other folder1",
      "type": "folder",
      "children": [
        {
          "date_added": "13292869307359599",
          "date_modified": "13292869307359963",
          "guid": "4a1fb38f-0e4a-4b6b-a718-5dbc5f3a2ea9",
          "id": "1507",
          "name": "Empty folder",
          "type": "folder",
          "children": []
        },
        {
          "date_added": "13280334723204976",
          "guid": "5cb6de11-58fe-4f8c-958d-988c4c542760",
          "id": "690",
          "name": "Bookmark In Other",
          "type": "url",
          "url": "https://bookmark.other"
        }
      ]
    }
  },
  "version": 1
}"#;

    fn expect_folder(entry: &BookmarkEntry) -> BookmarkFolder {
        match entry {
            BookmarkEntry::URL(_) => panic!("expected entry of type folder got URL"),
            BookmarkEntry::Folder(f) => f.clone(),
        }
    }
    fn expect_url(entry: &BookmarkEntry) -> BookmarkURL {
        match entry {
            BookmarkEntry::URL(u) => u.clone(),
            BookmarkEntry::Folder(_) => panic!("expected entry of type URL got folder"),
        }
    }

    #[test]
    fn test_deserialize_bookmarks() {
        let data = serde_json::de::from_str::<super::BookmarksData>(BOOKMARKS_DATA).unwrap();

        let parsed = data.bookmarks();

        let bookmark_bar = parsed.iter().find(|entry| {
            match entry {
                Ok(entry) => {
                    match entry {
                        BookmarkEntry::URL(_) => false,
                        BookmarkEntry::Folder(f) => {
                            f.name == "Bookmarks Bar"
                        }
                    }
                },
                Err(_) => false,
            }
        }).unwrap();

        let bookmark_bar_folder = expect_folder(bookmark_bar.as_ref().unwrap());
        assert_eq!("Bookmarks Bar", bookmark_bar_folder.name);

        let urls = bookmark_bar_folder.children.iter().filter_map(|c| {
            match c {
                BookmarkEntry::URL(url) => Some(url.clone()),
                _ => None
            }
        }).collect::<Vec<BookmarkURL>>();
        assert_eq!(1, urls.len());
        assert_eq!("Bookmark In folder", &urls[0].name);
        assert_eq!("https://bookmark.in.folder", &urls[0].url);

        let folders = bookmark_bar_folder.children.iter().filter_map(|c| {
            match c {
                BookmarkEntry::Folder(folder) => Some(folder.clone()),
                _ => None
            }
        }).collect::<Vec<BookmarkFolder>>();
        assert_eq!(1, folders.len());
        assert_eq!("Folder1", &folders[0].name);
        assert_eq!(1, folders[0].children.len());
        let double_nested_folder = expect_folder(&folders[0].children[0]);
        assert_eq!("Nested Folder", double_nested_folder.name);
        assert_eq!("Bookmark In folder In folder", expect_url(&double_nested_folder.children[0]).name);
        assert_eq!("https://nested.com", expect_url(&double_nested_folder.children[0]).url);
        assert_eq!("Bookmark In folder In folder2", expect_url(&double_nested_folder.children[1]).name);
        assert_eq!("https://nested2.com", expect_url(&double_nested_folder.children[1]).url);
    }
}
