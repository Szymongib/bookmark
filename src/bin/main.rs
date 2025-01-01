extern crate clap;
use clap::{Arg, ArgAction, ArgMatches, Command};

use crate::interactive::interactive_mode::enter_interactive_mode;
use crate::interactive::subcommand::add;

use bookmark_lib::registry::{URLRegistry, DEFAULT_GROUP};
use bookmark_lib::storage::FileStorage;
use bookmark_lib::Registry;

use bookmark_lib::filters::{Filter, GroupFilter, NoopFilter, TagsFilter};
use bookmark_lib::sort::{SortBy, SortConfig};
use std::str::FromStr;

mod cmd;
mod display;
mod interactive;

const URLS_V0_0_X_DEFAULT_FILE_PATH: &str = ".bookmark-cli/urls.json";

const URLS_DEFAULT_FILE_PATH: &str = ".bookmark/urls_v0.1.json";

const VERSION_V0_0_X: &str = " v0.0.x";

fn main() {
    let urls_v0_0_x_default_full_path = path_with_homedir(URLS_V0_0_X_DEFAULT_FILE_PATH)
        .expect("Failed to get default v0_0_x path");

    let cmd = Command::new("Bookmark")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Szymon Gibała <szumongib@gmail.com>")
        .about("Group, tag and quickly access your URLs from terminal")
        .arg(Arg::new("file")
            .short('f')
            .long("file")
            .value_name("FILE")
            .required(false)
            .help("Path to file storing the URLs")
            .action(ArgAction::Set)
        )
        .subcommand(Command::new(cmd::GROUP_SUB_CMD)
            .about("Manage URL groups")
            .subcommand(Command::new(cmd::GROUP_LIST_CMD)
                .about("List groups")
            )
        )
        .subcommand(Command::new(cmd::ADD_SUB_CMD)
            .about("Add bookmark URL")
            .arg(Arg::new("name")
                .help("Bookmark name")
                .index(1)
                )
            .arg(Arg::new("url")
                .help("URL address")
                .index(2)
            )
            .arg(Arg::new("tag")
                .help("URL tags. Accepts multiple values: url add [NAME] [URL] -t tag1 -t tag2")
                .required(false)
                .short('t')
                .long("tag")
                .action(ArgAction::Append)
                .number_of_values(1)
                // TODO: add validator to exclude forbidden chars (like ,)
            )
            .arg(Arg::new("group")
                .help("Group to which URL should be assigned")
                .required(false)
                .action(ArgAction::Set)
                .short('g')
                .long("group"))
        )
        .subcommand(Command::new(cmd::LIST_SUB_CMD)
            .alias("ls")
            .about("List bookmarks ")
            .arg(Arg::new("group") // If not specified use default or global
                .help("Group from which URLs should be listed")
                .required(false)
                .action(ArgAction::Set)
                .short('g')
                .long("group"))
            .arg(Arg::new("tag")
                .help("URL tags. Accepts multiple values: url add [NAME] [URL] -t tag1 -t tag2")
                .required(false)
                .short('t')
                .long("tag")
                .action(ArgAction::Append)
                .number_of_values(1))
            .arg(Arg::new("sort")
                .help("Specifies to sort bookmarks by one of the columns: [name, url, group]")
                .required(false)
                .long("sort")
                .action(ArgAction::Set)
                .number_of_values(1))
        )
        .subcommand(Command::new(cmd::DELETE_SUB_CMD)
            .about("Delete bookmark")
            .arg(Arg::new("id")
                .help("Bookmark id to delete")
                .required(true)
                .index(1)
            )
        )
        .subcommand(Command::new(cmd::TAG_SUB_CMD)
            .about("Add tag to bookmark")
            // .usage("bookmark tag [ID] [TAG]")
            // .override_usage(usage)
            .arg(Arg::new("id")
                .help("Bookmark id to tag")
                .required(true)
                .index(1))
            .arg(Arg::new("tag")
                .help("Tag to add")
                .required(true)
                .index(2)
            )
        )
        .subcommand(Command::new(cmd::UNTAG_SUB_CMD)
            .about("Remove tag from bookmark")
            // .usage("bookmark untag [ID] [TAG]")
            .arg(Arg::new("id")
                .help("Bookmark id to untag")
                .required(true)
                .index(1))
            .arg(Arg::new("tag")
                .help("Tag to remove")
                .required(true)
                .index(2)
            )
        )
        .subcommand(Command::new(cmd::CHANGE_GROUP_SUB_CMD)
            .about("Change group of the bookmark")
            // .usage("bookmark chg [ID] [GROUP]")
            .alias(cmd::CHANGE_GROUP_SUB_CMD_ALIAS)
            .arg(Arg::new("id")
                .help("Bookmark id to change the group")
                .required(true)
                .index(1))
            .arg(Arg::new("group")
                .help("New group")
                .required(true)
                .index(2)
            )
        )
        .subcommand(Command::new(cmd::CHANGE_NAME_SUB_CMD)
            .about("Change name of the bookmark")
            // .usage("bookmark chn [ID] [NAME]")
            .alias(cmd::CHANGE_NAME_SUB_CMD_ALIAS)
            .arg(Arg::new("id")
                .help("Bookmark id to change the name")
                .required(true)
                .index(1))
            .arg(Arg::new("name")
                .help("New name")
                .required(true)
                .index(2)
            )
        )
        .subcommand(Command::new(cmd::CHANGE_URL_SUB_CMD)
            .about("Change URL of the bookmark")
            // .usage("bookmark chu [ID] [URL]")
            .alias(cmd::CHANGE_URL_SUB_CMD_ALIAS)
            .arg(Arg::new("id")
                .help("Bookmark id to change the URL")
                .required(true)
                .index(1))
            .arg(Arg::new("url")
                .help("New URL")
                .required(true)
                .index(2)
            )
        )
        // TODO: I think I can drop it at this point
        .subcommand(Command::new(cmd::IMPORT_SUB_CMD)
            .about("Imports bookmarks from the previous versions")
            .arg(Arg::new("version")
                .help(format!("Version from which URLs should be imported. One of: {}", VERSION_V0_0_X))
                .required(false)
                .action(ArgAction::Set)
                .short('v')
                .long("version")
                .default_value(VERSION_V0_0_X))
            .arg(Arg::new("old-file")
                .help("Path to the file storing URLs from previous version")
                .required(false)
                .action(ArgAction::Set)
                .long("old-file")
                .default_value(urls_v0_0_x_default_full_path)
            )
        );

    let matches = cmd.get_matches();

    let file_path = match matches.get_one::<String>("file") {
        Some(t) => t.to_string(),
        None => match get_default_registry_file_path() {
            Some(path) => path,
            None => panic!("Failed to get default file path"),
        },
    };

    let application = Application::new_file_based_registry(file_path);

    match matches.subcommand() {
        Some((cmd::GROUP_SUB_CMD, group_matches)) => {
            application.group_sub_cmd(group_matches);
        }
        Some((cmd::ADD_SUB_CMD, add_matches)) => {
            application.add_sub_cmd(add_matches);
        }
        Some((cmd::LIST_SUB_CMD, list_matches)) => {
            application.list_sub_cmd(list_matches);
        }
        Some((cmd::DELETE_SUB_CMD, delete_matches)) => {
            application.delete_sub_cmd(delete_matches);
        }
        Some((cmd::IMPORT_SUB_CMD, import_matches)) => {
            application.import_sub_cmd(import_matches);
        }
        Some((cmd::TAG_SUB_CMD, tag_matches)) => {
            application.tag_sub_cmd(tag_matches);
        }
        Some((cmd::UNTAG_SUB_CMD, untag_matches)) => {
            application.untag_sub_cmd(untag_matches);
        }
        Some((cmd::CHANGE_GROUP_SUB_CMD, chg_matches)) => {
            application.change_group_sub_cmd(chg_matches);
        }
        Some((cmd::CHANGE_NAME_SUB_CMD, chn_matches)) => {
            application.change_name_sub_cmd(chn_matches);
        }
        Some((cmd::CHANGE_URL_SUB_CMD, chu_matches)) => {
            application.change_url_sub_cmd(chu_matches);
        }
        None => {
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
        let url_name = matches.get_one::<String>("name");
        let url = matches.get_one::<String>("url");
        let group = matches
            .get_one::<String>("group")
            .map(|g| g.to_string())
            .unwrap_or(DEFAULT_GROUP.to_string());

        let tags: Vec<String> = get_multiple_values(matches, "tag")
            .unwrap_or_default()
            .iter()
            .map(|s| s.to_string())
            .collect();

        let mut add_data = add::AddData::new(
            url_name.unwrap_or(&"".to_string()).as_str(),
            url.unwrap_or(&"".to_string()).as_str(),
            &group,
            &tags,
        );

        if url_name.is_none() || url.is_none() {
            add_data = add::interactive_add(add_data).expect("err");
        }

        match self.registry.create(
            &add_data.name,
            &add_data.url,
            Some(&add_data.group),
            add_data.tags,
        ) {
            Ok(url_record) => println!(
                "Added url '{}': '{}' to '{}' group",
                url_record.name, url_record.url, url_record.group
            ),
            Err(why) => println!(
                "Error adding url '{}' with name '{}': {}",
                add_data.url, add_data.name, why
            ),
        }
    }

    pub fn list_sub_cmd(&self, matches: &ArgMatches) {
        let noop_filter: Box<dyn Filter> = Box::new(NoopFilter::default());

        let group_filter: Box<dyn Filter> = matches
            .get_one::<String>("group")
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

        let sort_cfg = matches.get_one::<String>("sort").map(|val| {
            let sort_by = SortBy::from_str(val).expect("Invalid sort column");
            SortConfig::new_by(sort_by)
        });

        // TODO: support output as json?
        match self
            .registry
            .list_urls(Some(tags_filter.as_ref()), sort_cfg)
        {
            Ok(urls) => {
                display::display_urls(urls);
            }
            Err(why) => {
                println!("Error getting URLs: {}", why);
            }
        }
    }

    pub fn delete_sub_cmd(&self, matches: &ArgMatches) {
        let id = matches
            .get_one::<String>("id")
            .expect("Error: id not provided");

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
            .get_one::<String>("version")
            .expect("Version from which to import not provided");
        let old_file = matches
            .get_one::<String>("old-file")
            .expect("Old version file path not provided");

        match version.as_str() {
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
            .get_one::<String>("id")
            .expect("Error: bookmark id not provided");
        let tag = matches
            .get_one::<String>("tag")
            .expect("Error: tag not provided");

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
            .get_one::<String>("id")
            .expect("Error: bookmark id not provided");
        let tag = matches
            .get_one::<String>("tag")
            .expect("Error: tag not provided");

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
            .get_one::<String>("id")
            .expect("Error: bookmark id not provided");
        let group = matches
            .get_one::<String>("group")
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
            .get_one::<String>("id")
            .expect("Error: bookmark id not provided");
        let name = matches
            .get_one::<String>("name")
            .expect("Error: name not provided");

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
            .get_one::<String>("id")
            .expect("Error: bookmark id not provided");
        let url = matches
            .get_one::<String>("url")
            .expect("Error: url not provided");

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
    let values = matches.get_many::<String>(name);
    values.map(|vals| {
        vals.into_iter()
            .map(|s| s.as_str())
            // .filter_map(|s| s)
            .collect()
    })
}
