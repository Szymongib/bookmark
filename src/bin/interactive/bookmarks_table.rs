use crate::interactive::event::Event;
use crate::interactive::event::Signal;
use crate::interactive::table::{StatefulTable, TableItem};
use crate::interactive::url_table_item::{default_columns, Columns, URLItem};
use bookmark_lib::filters::{Filter, UnorderedWordSetFilter};
use bookmark_lib::types::URLRecord;
use bookmark_lib::Registry;
use std::sync::mpsc;
use termion::event::Key;

use crate::cmd;
use bookmark_lib::sort::{SortBy, SortConfig};

type CommandResult = Result<(), Box<dyn std::error::Error>>;

pub struct BookmarksTable {
    signal_sender: mpsc::Sender<Event<Key>>,
    registry: Box<dyn Registry>,
    table: StatefulTable<URLItem>,
    columns: Vec<String>,
    filter: Option<Box<dyn Filter>>,
    sort_cfg: Option<SortConfig>,
}

impl BookmarksTable {
    pub fn next(&mut self) {
        self.table.next()
    }

    pub fn previous(&mut self) {
        self.table.previous()
    }

    pub fn unselect(&mut self) {
        self.table.unselect()
    }

    pub fn table(&mut self) -> &mut StatefulTable<URLItem> {
        &mut self.table
    }

    pub fn columns(&self) -> &Columns {
        &self.columns
    }

    pub fn get_selected(&self) -> Result<Option<URLRecord>, Box<dyn std::error::Error>> {
        let selected_id = self.get_selected_id();
        if selected_id.is_none() {
            return Ok(None);
        }

        let url_record = self.registry.get_url(&selected_id.unwrap())?;

        Ok(url_record)
    }

    pub fn open(&self) -> Result<(), Box<dyn std::error::Error>> {
        match self.table.state.selected() {
            Some(id) => match open::that(self.table.items[id].url().as_str()) {
                Ok(_) => Ok(()),
                Err(err) => Err(From::from(format!(
                    "failed to open URL in the browser: {}",
                    err.to_string()
                ))),
            },
            None => Ok(()),
        }
    }

    pub fn search(&mut self, phrase: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.filter = Some(Box::new(UnorderedWordSetFilter::new(phrase)));
        self.refresh_items()
    }

    pub fn set_columns(&mut self, columns: Columns) -> Result<(), Box<dyn std::error::Error>> {
        self.columns = columns;
        self.refresh_items()
    }

    // TODO: consider returning some command result
    pub fn exec(
        &mut self,
        command: &str,
        args: Vec<&str>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let id = self.get_selected_id();

        match command {
            cmd::TAG_SUB_CMD => self.tag(id, args)?,
            cmd::UNTAG_SUB_CMD => self.untag(id, args)?,
            cmd::CHANGE_GROUP_SUB_CMD | cmd::CHANGE_GROUP_SUB_CMD_ALIAS => {
                self.change_group(id, args)?
            }
            cmd::CHANGE_NAME_SUB_CMD | cmd::CHANGE_NAME_SUB_CMD_ALIAS => {
                self.change_name(id, args)?
            }
            cmd::CHANGE_URL_SUB_CMD | cmd::CHANGE_URL_SUB_CMD_ALIAS => self.change_url(id, args)?,
            cmd::SORT_CMD => self.sort_urls(id, args)?,
            "q" | "quit" => self.signal_sender.send(Event::Signal(Signal::Quit))?,
            _ => return Err(From::from(format!("error: command {} not found", command))),
        };

        self.refresh_items()?;

        Ok(())
    }

    pub fn tag(&mut self, id: Option<String>, args: Vec<&str>) -> CommandResult {
        let id = unwrap_id(id)?;

        if args.is_empty() {
            return Err(From::from(
                "tag requires exactly one argument. Usage: tag [TAG_1]",
            )); // TODO: support multiple tags at once
        }

        self.registry.tag(&id, args[0])?;
        Ok(())
    }

    pub fn untag(&mut self, id: Option<String>, args: Vec<&str>) -> CommandResult {
        let id = unwrap_id(id)?;

        if args.is_empty() {
            return Err(From::from(
                "untag requires exactly one argument. Usage: untag [TAG_1]",
            )); // TODO: support multiple tags at once
        }

        self.registry.untag(&id, args[0])?;
        Ok(())
    }

    pub fn change_group(&mut self, id: Option<String>, args: Vec<&str>) -> CommandResult {
        let id = unwrap_id(id)?;

        if args.is_empty() {
            return Err(From::from(
                "change group requires exactly one argument. Usage: chg [GROUP]",
            ));
        }

        self.registry.change_group(&id, args[0])?;
        Ok(())
    }

    pub fn change_name(&mut self, id: Option<String>, args: Vec<&str>) -> CommandResult {
        let id = unwrap_id(id)?;

        if args.is_empty() {
            return Err(From::from(
                "change name requires exactly one argument. Usage: chn [NAME]",
            ));
        }

        self.registry.change_name(&id, args[0])?;
        Ok(())
    }

    pub fn change_url(&mut self, id: Option<String>, args: Vec<&str>) -> CommandResult {
        let id = unwrap_id(id)?;

        if args.is_empty() {
            return Err(From::from(
                "change url requires exactly one argument. Usage: chu [URL]",
            ));
        }

        self.registry.change_url(&id, args[0])?;
        Ok(())
    }

    pub fn sort_urls(&mut self, _: Option<String>, args: Vec<&str>) -> CommandResult {
        let sort_cfg = if args.is_empty() {
            SortConfig::new_by(SortBy::Name)
        } else {
            match args[0].to_lowercase().as_str() {
                "name" => SortConfig::new_by(SortBy::Name),
                "url" => SortConfig::new_by(SortBy::URL),
                "group" => SortConfig::new_by(SortBy::Group),
                _ => {
                    return Err(From::from(
                        "invalid sort column, must be one of: [name, url, group]",
                    ))
                }
            }
        };
        self.sort_cfg = Some(sort_cfg);

        self.refresh_items()
    }

    pub fn delete(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        match self.get_selected_id() {
            Some(id) => {
                if self.registry.delete(&id)? {
                    self.refresh_items()?;
                    return Ok(true);
                }
                Ok(false)
            }
            None => Ok(false),
        }
    }

    fn refresh_items(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let urls = match &self.filter {
            Some(f) => self.registry.list_urls(Some(f.as_ref()), self.sort_cfg)?,
            None => self.registry.list_urls(None, self.sort_cfg)?,
        };

        self.table
            .override_items(URLItem::from_vec(urls, Some(&self.columns)));
        Ok(())
    }

    fn get_selected_id(&self) -> Option<String> {
        self.table
            .state
            .selected()
            .map(|index| self.table.items[index].id())
    }
}

impl BookmarksTable {
    pub fn new(
        sender: mpsc::Sender<Event<Key>>,
        registry: Box<dyn Registry>,
    ) -> Result<BookmarksTable, Box<dyn std::error::Error>> {
        let default_columns = default_columns();

        let items: Vec<URLItem> =
            URLItem::from_vec(registry.list_urls(None, None)?, Some(&default_columns));
        let table = StatefulTable::with_items(items);

        Ok(BookmarksTable {
            signal_sender: sender,
            registry,
            table,
            filter: None,
            sort_cfg: None,
            columns: default_columns,
        })
    }
}

fn unwrap_id(id: Option<String>) -> Result<String, Box<dyn std::error::Error>> {
    match id {
        Some(id) => Ok(id),
        None => Err(From::from("error: item not selected".to_string())),
    }
}
