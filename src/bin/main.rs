extern crate clap;
use crate::interactive::interactive_mode;
use clap::{App, Arg, ArgMatches, SubCommand};

use bookmark_lib::registry::URLRegistry;
use bookmark_lib::storage::FileStorage;
use bookmark_lib::{Registry};
use crate::interactive::interactive_mode::enter_interactive_mode;
use bookmark_lib::types::URLRecord;
use std::collections::HashMap;

mod interactive;
mod display;

const GROUP_SUB_CMD: &str = "group";
const GROUP_LIST_CMD: &str = "list";

const ADD_SUB_CMD: &str = "add";
const LIST_SUB_CMD: &str = "list";
const DELETE_SUB_CMD: &str = "delete";

// const URLS_DEFAULT_FILE_PATH: &str = ".bookmark-cli/urls.json";

// TODO: change to that after modifying data model
const URLS_DEFAULT_FILE_PATH: &str = ".bookmark-cli/urls_v0.1.json";

fn main() {
    let matches = App::new("Bookmark CLI")
        .version("0.0.1")
        .author("Szymon Gibała <szumongib@gmail.com>")
        .about("Group and quickly access your URLs from terminal")
        .arg(Arg::with_name("file")
            .short("f")
            .long("file")
            .value_name("FILE")
            .required(false)
            .help("Path to file storing the URLs")
            .takes_value(true)
        )
        .subcommand(SubCommand::with_name(GROUP_SUB_CMD)
            .about("Manage URL groups")
            .subcommand(SubCommand::with_name(GROUP_LIST_CMD)
                .about("List groups")
            )
        )
        .subcommand(SubCommand::with_name(ADD_SUB_CMD)
            .about("Add bookmark URL")
            .arg(Arg::with_name("name")
                .help("Bookmark name")
                .required(true)
                .index(1)
                )
            .arg(Arg::with_name("url")
                .help("URL address")
                .required(true)
                .index(2)
            )
            .arg(Arg::with_name("tag")
                .help("URL tags. Accepts multiple values: url add [NAME] [URL] -t tag1 -t tag2")
                .required(false)
                .short("t")
                .long("tag")
                .takes_value(true)
                .multiple(true)
                .number_of_values(1)
            )
            .arg(Arg::with_name("group")
                .help("Group to which URL should be assigned")
                .required(false)
                .takes_value(true)
                .short("g")
                .long("group"))
        )
        .subcommand(SubCommand::with_name(LIST_SUB_CMD)
            .about("List bookmark URLs")
            .arg(Arg::with_name("group") // If not specified use default or global
                .help("Group from which URLs should be listed")
                .required(false)
                .takes_value(true)
                .short("g")
                .long("group"))
            .arg(Arg::with_name("tag")
                .help("URL tags. Accepts multiple values: url add [NAME] [URL] -t tag1 -t tag2")
                .required(false)
                .short("t")
                .long("tag")
                .takes_value(true)
                .multiple(true)
                .number_of_values(1))
        )
        .subcommand(SubCommand::with_name(DELETE_SUB_CMD)
            .about("List bookmark URLs")
            .arg(Arg::with_name("group") // If not specified use default or global
                .help("Group from which URL should be deleted")
                .required(false)
                .takes_value(true)
                .short("g")
                .long("group"))
            .arg(Arg::with_name("name")
                .help("Bookmark name")
                .required(true)
                .index(1)
            )
        )
        // TODO: Add import subcommand to import from v1
        .get_matches();

    let file_path = match matches.value_of("file") {
        Some(t) => t.to_string(),
        None => match get_default_registry_file_path() {
            Some(path) => path.to_string(),
            None => panic!("Failed to get default file path"),
        },
    };

    let application = Application::new_file_based_registry(file_path);

    match matches.subcommand() {
        (GROUP_SUB_CMD, Some(group_matches)) => {
            application.group_sub_cmd(group_matches);
        }
        (ADD_SUB_CMD, Some(add_matches)) => {
            application.add_sub_cmd(add_matches);
        }
        (LIST_SUB_CMD, Some(list_matches)) => {
            application.list_sub_cmd(list_matches);
        }
        (DELETE_SUB_CMD, Some(delete_matches)) => {
            application.delete_sub_cmd(delete_matches);
        }
        ("", None) => {
            enter_interactive_mode(application.registry);

        },
        _ => unreachable!(), // If all subcommands are defined above, anything else is unreachabe!()
    }
}

fn get_default_registry_file_path() -> Option<String> {
    match dirs::home_dir() {
        Some(home_dir) => home_dir
            .join(URLS_DEFAULT_FILE_PATH)
            .to_str()
            .map(|s: &str| s.to_string()),
        None => None,
    }
}

struct Application<T: Registry> {
    registry: T,
}

impl Application<URLRegistry<FileStorage>> {
    pub fn new_file_based_registry(file_path: String) -> Application<URLRegistry<FileStorage>> {
        Application { registry: URLRegistry::new_file_based(file_path) }
    }
}

impl<T: Registry> Application<T> {
    pub fn group_sub_cmd(&self, matches: &ArgMatches) {
        self.list_groups_cmd(matches)
    }

    fn list_groups_cmd(&self, _matches: &ArgMatches) {
        match self.registry.list_groups() {
            Ok(groups) => {
                for g in &groups {
                    println!("{}", g);
                }
            }
            Err(why) => println!("Failed to list groups: {}", why),
        }
    }

    pub fn add_sub_cmd(&self, matches: &ArgMatches) {
        let url_name = matches.value_of("name").expect("Error getting URL name");
        let url = matches.value_of("url").expect("Error getting URL");
        let group: Option<&str> = matches.value_of("group");

        let tags = get_multiple_values(matches, "tag").unwrap_or(vec![]);

        match self.registry.new(url_name, url, group, tags) {
            Ok(url_record) => println!(
                "Added url {}: {} to {} group",
                url_record.name, url_record.url, url_record.group
            ),
            Err(why) => println!("Error adding url {} with name {}: {}", url, url_name, why),
        }
    }

    pub fn list_sub_cmd(&self, matches: &ArgMatches) {
        let group = matches.value_of("group");
        let tags = get_multiple_values(matches, "tag");

        // TODO: support output as json?
        return match self.registry.list_urls(group, tags) {
            Ok(urls) => {
                display::display_urls(urls);
            }
            Err(why) => {
                println!("Error getting URLs: {}", why);
            }
        };
    }

    pub fn delete_sub_cmd(&self, matches: &ArgMatches) {
        let url_name = matches.value_of("name").expect("Error getting URL name");
        let group: Option<&str> = matches.value_of("group");

        let group_name = group.unwrap_or("default");

        match self.registry.delete_url(url_name, group) {
            Ok(deleted) => {
                if deleted {
                    println!("URL {} removed from {} group", url_name, group_name,)
                } else {
                    println!("URL {} not found in {} group", url_name, group_name,)
                }
            }
            Err(why) => println!(
                "Error deleting {} url from {} group: {}",
                url_name, group_name, why
            ),
        }
    }
}

fn get_multiple_values<'a>(matches: &'a ArgMatches, name: &str) -> Option<Vec<&'a str>> {
    let values = matches.args.get(name);
    values.map(|vals| {
        vals.vals
            .iter()
            .map(|s| s.to_str())
            .filter_map(|s| s)
            .collect()
    })
}
