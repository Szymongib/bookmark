use std::{path::PathBuf, collections::HashMap, io::Write};

use crate::import::{brave::bookmarks::BookmarkEntry, v0_0_x::URLRecord};

use super::{bookmarks::{BookmarksData, filter_bookmarks, Bookmarks}, BraveImportError};

pub fn import_from_bookmarks(bookmarks_path: &PathBuf) -> Result<Vec<URLRecord>, BraveImportError> {
    let bookmarks = read_bookmarks(bookmarks_path)?;

    // TODO: the issue with that approach is if some validation during inserting
    // fails, you loose all the work you have done...
    let mut urls_to_import = vec![];

    for (folder, entry) in bookmarks {
        urls_to_import.extend(import_from_folder(&folder, &vec![entry]));
    }

    Ok(urls_to_import)
}

fn import_from_folder(folder: &str, entries: &[BookmarkEntry]) -> Vec<URLRecord> {
    let mut urls_to_import = vec![];
    
    print!("Do you want to import bookmarks from folder '{}'? (y/n): " , folder);
    std::io::stdout().flush().unwrap();
    let import_folder = read_confirmation();
    if !import_folder {
        return urls_to_import;
    }

    for entry in entries {
        urls_to_import.extend(import_from_entry(entry, folder));
    }

    urls_to_import
}

fn import_from_entry(entry: &BookmarkEntry, parent_folder: &str) -> Vec<URLRecord> {
    let mut urls_to_import = vec![];
    
    match entry {
        BookmarkEntry::URL(url) => {
            print!("Do you want to import bookmark '{}' ({})? (y/n): " , url.name, url.url);
            std::io::stdout().flush().unwrap();
            let import_url = read_confirmation();

            // TODO: import URL
            if import_url {
                // TODO: some better import func
                urls_to_import.push(URLRecord { url: url.url.clone(), name: url.name.clone(), group: parent_folder.to_string(), tags: HashMap::new() });
            }
        }
        BookmarkEntry::Folder(folder) => {
            urls_to_import.extend(import_from_folder(&folder.name, &folder.children));
        }
    }

    urls_to_import
}

fn read_confirmation() -> bool {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("failed to read confirmation from stdin");

    match input.trim().to_lowercase().as_str() {
        "y" | "yes" => true,
        "n" | "no" => false,
        _ => {
            println!("Please answer with 'y' or 'n'");
            false
        }
    }
}


fn read_bookmarks(bookmarks_path: &PathBuf) -> Result<Bookmarks, BraveImportError> {
    let bookmarks_path = bookmarks_path.canonicalize()
        .map_err(|e| BraveImportError::BookmarksPathError(e))?;

    let bookmarks_raw = std::fs::read_to_string(bookmarks_path)
        .map_err(|e| BraveImportError::ReadBookmarksError(e))?;

    let data = serde_json::de::from_str::<BookmarksData>(&bookmarks_raw)
        .map_err(|e| BraveImportError::DeserializationError("bookmarks data", e))?;

    // for (k, v) in data.roots {
    //     println!("{}", k);
    //     for child in v.children.unwrap_or_default() {
    //         println!("  {}", child.name)
    //     }
    // }

    let parsed = data.bookmarks();

    let (bookmarks, _errs) = filter_bookmarks(parsed);
    // TODO: for now ignore errors, handle them later

    // for (k, v) in bookmarks {
    //     println!("{}", k);
    //     match v {
    //         BookmarkEntry::URL(url) => {
    //             println!("  {}", url.url);
    //         }
    //         BookmarkEntry::Folder(folder) => {
    //             println!("  {}", folder.name)
    //             // for child in folder.children {
    //                 // println!("  {}", child);
    //             // }
    //         }
    //     }
        
    // }

    Ok(bookmarks)

    // TODO: I envision import this way:
    // CLI asks:
    // 1. Do you want to import bookmarks from folder [TOP FOLDER]
        // y  ->  recurse: if folder ask again if we are entering, if url ask if we want to import
        // n  ->  skip

    // 2. Do you want to import bookamrk: [BOOKMARK NAME] ([URL])?
        // y  ->  import
        // n  ->  skip

    // 3. Importing bookamrk: [BOOKMARK NAME] ([URL])
    // > Name ([BOOKMARK NAME]): [NEW NAME]
    // > URL ([URL]): [NEW URL]
    // > Group ([CURRENT_FOLDER]): [NEW FOLDER]
    // > Tags: [TAGS]
    // > Save in bookmarks
}


// #[cfg(test)]
// mod tests {
//     use crate::import::brave::default_brookmarks_file_path;

//     use super::*;

//     #[test]
//     fn test_import_bookmarks() {
//         let path =default_brookmarks_file_path();
//         read_bookmarks(&path).unwrap();
//     }
// }