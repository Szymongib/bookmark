extern crate clap;
use clap::{App, Arg, ArgMatches, SubCommand};

use crate::interactive::interactive_mode::enter_interactive_mode;
use bookmark_lib::registry::URLRegistry;
use bookmark_lib::storage::FileStorage;
use bookmark_lib::Registry;

use bookmark_lib::filters::{Filter, GroupFilter, NoopFilter, TagsFilter};

mod display;
mod interactive;

const GROUP_SUB_CMD: &str = "group";
const GROUP_LIST_CMD: &str = "list";

const ADD_SUB_CMD: &str = "add";
const LIST_SUB_CMD: &str = "list";
const DELETE_SUB_CMD: &str = "delete";
const TAG_SUB_CMD: &str = "tag";
const UNTAG_SUB_CMD: &str = "untag";
const IMPORT_SUB_CMD: &str = "import";

const URLS_V0_0_X_DEFAULT_FILE_PATH: &str = ".bookmark-cli/urls.json";

const URLS_DEFAULT_FILE_PATH: &str = ".bookmark-cli/urls_v0.1.json";

const VERSION_V0_0_X: &str = " v0.0.x";

fn main() {
    let urls_v0_0_x_default_full_path = &path_with_homedir(URLS_V0_0_X_DEFAULT_FILE_PATH)
        .expect("Faield to get default v0_0_x path");

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
        .subcommand(SubCommand::with_name(TAG_SUB_CMD)
            .about("Add tag to bookmark")
            .usage("bookmark tag [ID] [TAG]")
            .arg(Arg::with_name("id") // If not specified use default or global
                .help("Bookmark id to tag")
                .required(true)
                .index(1))
            .arg(Arg::with_name("tag")
                .help("Tag to add")
                .required(true)
                .index(2)
            )
        )
        .subcommand(SubCommand::with_name(UNTAG_SUB_CMD)
            .about("Remove tag from bookmark")
            .usage("bookmark untag [ID] [TAG]")
            .arg(Arg::with_name("id") // If not specified use default or global
                .help("Bookmark id to untag")
                .required(true)
                .index(1))
            .arg(Arg::with_name("tag")
                .help("Tag to remove")
                .required(true)
                .index(2)
            )
        )
        .subcommand(SubCommand::with_name(IMPORT_SUB_CMD)
            .about("Imports bookmarks from the previous versions")
            .arg(Arg::with_name("version")
                .help(format!("Version from which URLs should be imported. One of: {}", VERSION_V0_0_X).as_str())
                .required(false)
                .takes_value(true)
                .short("v")
                .long("version")
                .default_value(VERSION_V0_0_X))
            .arg(Arg::with_name("old-file")
                .help("Path to the file storing URLs from previous version")
                .required(false)
                .takes_value(true)
                .long("old-file")
                .default_value(urls_v0_0_x_default_full_path)
            )
        )
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
        (IMPORT_SUB_CMD, Some(import_matches)) => {
            application.import_sub_cmd(import_matches);
        }
        (TAG_SUB_CMD, Some(tag_matches)) => {
            application.tag_sub_cmd(tag_matches);
        }
        (UNTAG_SUB_CMD, Some(untag_matches)) => {
            application.untag_sub_cmd(untag_matches);
        }
        ("", None) => {
            match enter_interactive_mode(application.registry) {
                Err(err) => println!(
                    "Error: failed to enter interactive mode: {}",
                    err.to_string()
                ),
                _ => {}
            };
        }
        _ => println!("Error: subcommand not found"),
    }
}

fn get_default_registry_file_path() -> Option<String> {
    path_with_homedir(URLS_DEFAULT_FILE_PATH)
}

fn path_with_homedir(path: &str) -> Option<String> {
    match dirs::home_dir() {
        Some(home_dir) => home_dir.join(path).to_str().map(|s: &str| s.to_string()),
        None => None,
    }
}

struct Application<T: Registry> {
    registry: T,
}

impl Application<URLRegistry<FileStorage>> {
    pub fn new_file_based_registry(file_path: String) -> Application<URLRegistry<FileStorage>> {
        Application {
            registry: URLRegistry::new_file_based(file_path),
        }
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
            Err(why) => println!("Error: failed to list groups: {}", why),
        }
    }

    pub fn add_sub_cmd(&self, matches: &ArgMatches) {
        let url_name = matches.value_of("name").expect("Error getting URL name");
        let url = matches.value_of("url").expect("Error getting URL");
        let group: Option<&str> = matches.value_of("group");

        let tags = get_multiple_values(matches, "tag").unwrap_or(vec![]);

        match self.registry.new(url_name, url, group, tags) {
            Ok(url_record) => println!(
                "Added url '{}': '{}' to {}' group",
                url_record.name, url_record.url, url_record.group
            ),
            Err(why) => println!(
                "Error adding url '{}' with name '{}': {}",
                url, url_name, why
            ),
        }
    }

    pub fn list_sub_cmd(&self, matches: &ArgMatches) {
        let noop_filter: Box<dyn Filter> = Box::new(NoopFilter::new());

        let group_filter: Box<dyn Filter> = matches
            .value_of("group")
            .map(|g| {
                let f: Box<dyn Filter> = Box::new(GroupFilter::new(g));
                f
            })
            .unwrap_or(noop_filter);

        let tags_filter: Box<dyn Filter> = get_multiple_values(matches, "tag")
            .map(|t| {
                let f: Box<dyn Filter> = Box::new(TagsFilter::new(t));
                f
            })
            .unwrap_or(group_filter);

        // TODO: support output as json?
        return match self.registry.list_urls(Some(&tags_filter)) {
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

        match self.registry.delete("TODO: id") {
            Ok(deleted) => {
                if deleted {
                    println!("URL '{}' removed from '{}' group", url_name, group_name,)
                } else {
                    println!("URL '{}' not found in '{}' group", url_name, group_name,)
                }
            }
            Err(why) => println!(
                "Error deleting '{}' url from '{}' group: {}",
                url_name, group_name, why
            ),
        }
    }

    pub fn import_sub_cmd(&self, matches: &ArgMatches) {
        let version = matches
            .value_of("version")
            .expect("Version from which to import not provided");
        let old_file = matches
            .value_of("old-file")
            .expect("Old version file path not provided");

        match version {
            VERSION_V0_0_X => match self.registry.import_from_v_0_0_x(old_file) {
                Ok(_imported) => println!("Successfully imported bookmarks!"),
                Err(why) => println!(
                    "Error importing bookmarks from file '{}': {} ",
                    old_file, why
                ),
            },
            v => {
                println!("Error importing bookmarks, version '{}' not recognized. Version have to be one of '{}'", v, VERSION_V0_0_X);
            }
        }
    }

    pub fn tag_sub_cmd(&self, matches: &ArgMatches) {
        let id = matches
            .value_of("id")
            .expect("Error: bookmark id not provided");
        let tag = matches.value_of("tag").expect("Error: tag not provided");

        match self.registry.tag(id.clone(), tag.clone()) {
            Ok(record) => match record {
                Some(r) => println!("Bookmark '{}' tagged with '{}'", r.id, tag),
                None => println!("Error: bookmark with id '{}' not found", id),
            },
            Err(why) => println!("Error: failed to tag bookmark '{}': {} ", id, why),
        }
    }

    pub fn untag_sub_cmd(&self, matches: &ArgMatches) {
        let id = matches
            .value_of("id")
            .expect("Error: bookmark id not provided");
        let tag = matches.value_of("tag").expect("Error: tag not provided");

        match self.registry.untag(id.clone(), tag.clone()) {
            Ok(record) => match record {
                Some(r) => println!("Tag '{}' removed from bookmark '{}'", tag, r.id),
                None => println!("Error: bookmark with id '{}' not found", id),
            },
            Err(why) => println!("Error: failed to untag bookmark '{}': {} ", id, why),
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
