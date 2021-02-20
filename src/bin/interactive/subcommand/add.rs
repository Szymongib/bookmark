use crate::interactive::subcommand::{ask_for_string, require_string};
use std::ops::Add;
use std::io::{BufRead, Write};

#[derive(Debug)]
pub struct AddData {
    pub name: String,
    pub url: String,
    pub group: String,
    pub tags: Vec<String>,
}

impl AddData {
    pub fn new(name: &str, url: &str, group: &str, tags: &[String]) -> AddData {
        AddData {
            name: name.to_string(),
            url: url.to_string(),
            group: group.to_string(),
            tags: tags
                .to_vec()
                .into_iter()
                .filter(|t| !t.is_empty())
                .collect(),
        }
    }

    pub fn construct(name: Option<&str>, url: Option<&str>, group: &str, tags: &[&str]) -> AddData {
        let tags: Vec<String> = tags.iter().map(|t| t.to_string()).collect();

        AddData::new(
            name.unwrap_or_default(),
            url.unwrap_or_default(),
            group,
            &tags,
        )
    }
}

pub fn interactive_add(add_data: AddData) -> Result<AddData, Box<dyn std::error::Error>> {
    let name = if add_data.name.is_empty() {
        require_string("Bookmark name", "Bookmark name is required!")?
    } else {
        ask_for_string("Bookmark name", &add_data.name)?
    };

    let url = if add_data.url.is_empty() {
        require_string("Bookmark URL", "Bookmark URL is required!")?
    } else {
        ask_for_string("Bookmark URL", &add_data.url)?
    };

    let group = ask_for_string("Bookmark group", &add_data.group)?;
    let tags_raw = ask_for_string("Tags", &add_data.tags.join(", "))?;

    // TODO: handle whitespaces better here
    let tags: Vec<String> = tags_raw
        .split(", ")
        .into_iter()
        .map(|f| f.to_string())
        .collect();

    // TODO: after adding validation, add it here too

    Ok(AddData::new(&name, &url, &group, &tags))
}
