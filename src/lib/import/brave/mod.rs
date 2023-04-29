use std::path::PathBuf;


pub mod import;
mod bookmarks;

pub const DEFAULT_BOOKMARKS_PATH_MAC: &str = "Library/Application Support/BraveSoftware/Brave-Browser/Default/Bookmarks";


#[derive(thiserror::Error, Debug)]
pub enum BraveImportError {
    #[error("unexpected bookmark entry type: {0}")]
    UnexpectedEntryType(String),
    #[error("failed to deserialize object {0}: {1}")]
    DeserializationError(&'static str, serde_json::Error),
    #[error("failed to read bookmraks file: {0}")]
    ReadBookmarksError(std::io::Error),
    #[error("bookmarks file path error: {0}")]
    BookmarksPathError(std::io::Error),
}

pub fn default_brookmarks_file_path() -> String {

    // TODO: different logic for different OSes

    let home_dir =  home::home_dir()
        .expect("failed to determine home dir for default bookmarks file path");

    let path = home_dir.join(DEFAULT_BOOKMARKS_PATH_MAC);

    path.to_str()
        .expect("failed to convert default bookmarks file path to string")
        .to_string()
}