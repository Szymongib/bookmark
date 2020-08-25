use bookmark_lib::types::URLRecord;
use bookmark_lib::filters::Filter;
use crate::interactive::table::{StatefulTable, TableItem};
use crate::interactive::url_table_item::URLItem;
use bookmark_lib::filters::FilterSet;
use bookmark_lib::Registry;
use std::sync::mpsc;
use crate::interactive::event::Event;
use termion::event::Key;
use crate::interactive::event::Signal;

pub struct BookmarksTable<'a> {
    signal_sender: &'a mpsc::Sender<Event<Key>>,
    registry: Box<dyn Registry>,
    table: StatefulTable<URLItem>,
    filter: Option<Box<dyn Filter>>,
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

    pub fn get_selected(&self) -> Result<Option<URLRecord>, Box<dyn std::error::Error>> {
        let selected_id = self.get_selected_id();
        if selected_id.is_none() {
            return Ok(None);
        }

        let url_record = self.registry.get_url(&selected_id.unwrap())?;

        return Ok(url_record)
    }

    pub fn open(&self) -> Result<(), Box<dyn std::error::Error>> {
        match self.table.state.selected() {
            Some(id) => {
                match open::that(self.table.items[id].url().as_str()) {
                    Ok(_) => Ok(()),
                    Err(err) => Err(From::from(format!(
                        "failed to open URL in the browser: {}", err.to_string()
                    )))
                }
            },
            None => Ok(())
        }
    }

    pub fn search(&mut self, phrase: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.filter = Some(Box::new(FilterSet::new_combined_for_phrase(phrase)));
        self.refresh_items()
    }

    // TODO: consider returning some command result
    pub fn exec(&mut self, command: &str, args: Vec<&str>) -> Result<(), Box<dyn std::error::Error>> {
        let id = self.get_selected_id();

        match command {
            "tag" =>  self.tag(id, args)?,
            "q" => self.signal_sender.send(Event::Signal(Signal::Quit))?,
            _ => return Err(From::from(format!("error: command {} not found", command)))
        };

        self.refresh_items()?;

        Ok(())
    }

    // TODO: some command wrapper? The same functions will be used in CLI version
    pub fn tag(&mut self, id: Option<String>, args: Vec<&str>) -> Result<(), Box<dyn std::error::Error>>  {
        let id = unwrap_id(id)?;

        if args.len() < 1 {
            return Err(From::from("Tag requires exactly one argument. Usage: tag [TAG_1]")) // TODO: support multiple tags at once
        }

        self.registry.tag_url(&id, args[0])?;
        Ok(())
    }

    // TODO: quit command

    pub fn delete(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        match self.get_selected_id() {
            Some(id) => {
                if self.registry.delete_by_id(&id)? {
                    self.refresh_items()?;
                    return Ok(true)
                }
                Ok(false)
            },
            None => Ok(false)
        }
    }

    fn refresh_items(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let urls = match &self.filter {
            Some(f) => self.registry.list_urls(Some(&f))?,
            None => self.registry.list_urls(None)?
        };

        self.table.override_items(URLItem::from_vec(urls));
        Ok(())
    }

    fn get_selected_id(&self) -> Option<String> {
        self.table.state.selected().map(|index| {
            self.table.items[index].id()
        })
    }

}

impl<'a> BookmarksTable<'a> {

    pub fn new(sender:  &'a mpsc::Sender<Event<Key>>, registry: Box<dyn Registry>, table: StatefulTable<URLItem>) -> BookmarksTable {
        BookmarksTable{
            signal_sender: sender,
            registry,
            table,
            filter: None,
        }
    }

}

fn unwrap_id(id: Option<String>) -> Result<String, Box<dyn std::error::Error>> {
    match id {
        Some(id) => Ok(id),
        None => Err(From::from(format!("error: item not selected")))
    }
}

