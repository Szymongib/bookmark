use bookmark_lib::types::URLRecord;
use bookmark_lib::filters::Filter;
use crate::interactive::table::{StatefulTable, TableItem};
use crate::interactive::url_table_item::URLItem;
use bookmark_lib::filters::FilterSet;
use bookmark_lib::Registry;

pub struct BookmarksTable {
    registry: Box<dyn Registry>,// TODO: or just Box<dyn Registry>?
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

        let url_record = self.registry.get_url(selected_id.unwrap())?;

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

    pub fn tag(&mut self, tag: String) {
        // TODO: Tag selected URL
    }

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

impl BookmarksTable {

    pub fn new(registry: Box<dyn Registry>, table: StatefulTable<URLItem>) -> BookmarksTable {
        BookmarksTable{
            registry,
            table,
            filter: None,
        }
    }

}
