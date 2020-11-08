extern crate clap;
use clap::{App, Arg, ArgMatches, SubCommand};

use crate::interactive::interactive_mode::enter_interactive_mode;
use bookmark_lib::registry::URLRegistry;
use bookmark_lib::storage::FileStorage;
use bookmark_lib::Registry;

use bookmark_lib::filters::{Filter, GroupFilter, NoopFilter, TagsFilter};
use bookmark_lib::sort::{SortBy, SortConfig};

mod cmd;
mod display;
mod interactive;

const URLS_V0_0_X_DEFAULT_FILE_PATH: &str = ".bookmark-cli/urls.json";

const URLS_DEFAULT_FILE_PATH: &str = ".bookmark/urls_v0.1.json";

const VERSION_V0_0_X: &str = " v0.0.x";

fn main() {
    let urls_v0_0_x_default_full_path = &path_with_homedir(URLS_V0_0_X_DEFAULT_FILE_PATH)
        .expect("Failed to get default v0_0_x path");

    let matches = App::new("Bookmark")
        .version("0.0.1")
        .author("Szymon Gibała <szumongib@gmail.com>")
        .about("Group, tag and quickly access your URLs from terminal")
        .arg(Arg::with_name("file")
            .short("f")
            .long("file")
            .value_name("FILE")
            .required(false)
            .help("Path to file storing the URLs")
            .takes_value(true)
        )
        .subcommand(SubCommand::with_name(cmd::GROUP_SUB_CMD)
            .about("Manage URL groups")
            .subcommand(SubCommand::with_name(cmd::GROUP_LIST_CMD)
                .about("List groups")
            )
        )
        .subcommand(SubCommand::with_name(cmd::ADD_SUB_CMD)
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
        .subcommand(SubCommand::with_name(cmd::LIST_SUB_CMD)
            .alias("ls")
            .about("List bookmarks ")
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
            .arg(Arg::with_name("sort")
                .help("Specifies to sort bookmarks by one of the columns: [name, url, group]")
                .required(false)
                .long("sort")
                .takes_value(true)
                .number_of_values(1))
        )
        .subcommand(SubCommand::with_name(cmd::DELETE_SUB_CMD)
            .about("Delete bookmark")
            .arg(Arg::with_name("id")
                .help("Bookmark id to delete")
                .required(true)
                .index(1)
            )
        )
        .subcommand(SubCommand::with_name(cmd::TAG_SUB_CMD)
            .about("Add tag to bookmark")
            .usage("bookmark tag [ID] [TAG]")
            .arg(Arg::with_name("id")
                .help("Bookmark id to tag")
                .required(true)
                .index(1))
            .arg(Arg::with_name("tag")
                .help("Tag to add")
                .required(true)
                .index(2)
            )
        )
        .subcommand(SubCommand::with_name(cmd::UNTAG_SUB_CMD)
            .about("Remove tag from bookmark")
            .usage("bookmark untag [ID] [TAG]")
            .arg(Arg::with_name("id")
                .help("Bookmark id to untag")
                .required(true)
                .index(1))
            .arg(Arg::with_name("tag")
                .help("Tag to remove")
                .required(true)
                .index(2)
            )
        )
        .subcommand(SubCommand::with_name(cmd::CHANGE_GROUP_SUB_CMD)
            .about("Change group of the bookmark")
            .usage("bookmark chg [ID] [GROUP]")
            .alias(cmd::CHANGE_GROUP_SUB_CMD_ALIAS)
            .arg(Arg::with_name("id")
                .help("Bookmark id to change the group")
                .required(true)
                .index(1))
            .arg(Arg::with_name("group")
                .help("New group")
                .required(true)
                .index(2)
            )
        )
        .subcommand(SubCommand::with_name(cmd::CHANGE_NAME_SUB_CMD)
            .about("Change name of the bookmark")
            .usage("bookmark chn [ID] [NAME]")
            .alias(cmd::CHANGE_NAME_SUB_CMD_ALIAS)
            .arg(Arg::with_name("id")
                .help("Bookmark id to change the name")
                .required(true)
                .index(1))
            .arg(Arg::with_name("name")
                .help("New name")
                .required(true)
                .index(2)
            )
        )
        .subcommand(SubCommand::with_name(cmd::CHANGE_URL_SUB_CMD)
            .about("Change URL of the bookmark")
            .usage("bookmark chu [ID] [URL]")
            .alias(cmd::CHANGE_URL_SUB_CMD_ALIAS)
            .arg(Arg::with_name("id")
                .help("Bookmark id to change the URL")
                .required(true)
                .index(1))
            .arg(Arg::with_name("url")
                .help("New URL")
                .required(true)
                .index(2)
            )
        )
        .subcommand(SubCommand::with_name(cmd::IMPORT_SUB_CMD)
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
            Some(path) => path,
            None => panic!("Failed to get default file path"),
        },
    };

    let application = Application::new_file_based_registry(file_path);

    match matches.subcommand() {
        (cmd::GROUP_SUB_CMD, Some(group_matches)) => {
            application.group_sub_cmd(group_matches);
        }
        (cmd::ADD_SUB_CMD, Some(add_matches)) => {
            application.add_sub_cmd(add_matches);
        }
        (cmd::LIST_SUB_CMD, Some(list_matches)) => {
            application.list_sub_cmd(list_matches);
        }
        (cmd::DELETE_SUB_CMD, Some(delete_matches)) => {
            application.delete_sub_cmd(delete_matches);
        }
        (cmd::IMPORT_SUB_CMD, Some(import_matches)) => {
            application.import_sub_cmd(import_matches);
        }
        (cmd::TAG_SUB_CMD, Some(tag_matches)) => {
            application.tag_sub_cmd(tag_matches);
        }
        (cmd::UNTAG_SUB_CMD, Some(untag_matches)) => {
            application.untag_sub_cmd(untag_matches);
        }
        (cmd::CHANGE_GROUP_SUB_CMD, Some(chg_matches)) => {
            application.change_group_sub_cmd(chg_matches);
        }
        (cmd::CHANGE_NAME_SUB_CMD, Some(chn_matches)) => {
            application.change_name_sub_cmd(chn_matches);
        }
        (cmd::CHANGE_URL_SUB_CMD, Some(chu_matches)) => {
            application.change_url_sub_cmd(chu_matches);
        }
        ("", None) => {
            if let Err(err) = enter_interactive_mode(application.registry) {
                println!(
                    "Error: failed to enter interactive mode: {}",
                    err.to_string()
                )
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

        let tags = get_multiple_values(matches, "tag").unwrap_or_default();

        match self.registry.create(url_name, url, group, tags) {
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
        let noop_filter: Box<dyn Filter> = Box::new(NoopFilter::default());

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

        let sort_cfg = matches.value_of("sort").map(|val| {
            let sort_by = SortBy::from_str(val).expect("Invalid sort column");
            SortConfig::new_by(sort_by)
        });

        // TODO: support output as json?
        match self.registry.list_urls(Some(tags_filter.as_ref()), sort_cfg) {
            Ok(urls) => {
                display::display_urls(urls);
            }
            Err(why) => {
                println!("Error getting URLs: {}", why);
            }
        }
    }

    pub fn delete_sub_cmd(&self, matches: &ArgMatches) {
        let id = matches.value_of("id").expect("Error: id not provided");

        match self.registry.delete(id) {
            Ok(deleted) => {
                if deleted {
                    println!("URL '{}' removed", id)
                } else {
                    println!("URL '{}' not found", id)
                }
            }
            Err(why) => println!("Error deleting '{}' URL: {}", id, why),
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

        match self.registry.tag(id, tag) {
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

        match self.registry.untag(id, tag) {
            Ok(record) => match record {
                Some(r) => println!("Tag '{}' removed from bookmark '{}'", tag, r.id),
                None => println!("Error: bookmark with id '{}' not found", id),
            },
            Err(why) => println!("Error: failed to untag bookmark '{}': {} ", id, why),
        }
    }

    pub fn change_group_sub_cmd(&self, matches: &ArgMatches) {
        let id = matches
            .value_of("id")
            .expect("Error: bookmark id not provided");
        let group = matches
            .value_of("group")
            .expect("Error: group not provided");

        match self.registry.change_group(id, group) {
            Ok(record) => match record {
                Some(r) => println!("Bookmark '{}' group change to '{}'", r.id, group),
                None => println!("Error: bookmark with id '{}' not found", id),
            },
            Err(why) => println!(
                "Error: failed to change group of bookmark '{}': {} ",
                id, why
            ),
        }
    }

    pub fn change_name_sub_cmd(&self, matches: &ArgMatches) {
        let id = matches
            .value_of("id")
            .expect("Error: bookmark id not provided");
        let name = matches.value_of("name").expect("Error: name not provided");

        match self.registry.change_name(id, name) {
            Ok(record) => match record {
                Some(r) => println!("Bookmark '{}' name change to '{}'", r.id, name),
                None => println!("Error: bookmark with id '{}' not found", id),
            },
            Err(why) => println!(
                "Error: failed to change name of bookmark '{}': {} ",
                id, why
            ),
        }
    }

    pub fn change_url_sub_cmd(&self, matches: &ArgMatches) {
        let id = matches
            .value_of("id")
            .expect("Error: bookmark id not provided");
        let url = matches.value_of("url").expect("Error: url not provided");

        match self.registry.change_url(id, url) {
            Ok(record) => match record {
                Some(r) => println!("Bookmark '{}' url change to '{}'", r.id, url),
                None => println!("Error: bookmark with id '{}' not found", id),
            },
            Err(why) => println!("Error: failed to change url of bookmark '{}': {} ", id, why),
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
