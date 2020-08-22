use bookmark_lib::filters::Filter;
use tui::widgets::TableState;
use crate::interactive::table::StatefulTable;
use crate::interactive::url_table_item::URLItem;
use bookmark_lib::filters::FilterSet;
use bookmark_lib::Registry;

pub struct BookmarksTable {
    registry: Box<dyn Registry>,// TODO: or just Box<dyn Registry>?
    table: StatefulTable<URLItem>
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

    // pub fn items(&self) -> &Vec<URLItem> {
    //     &self.table.items
    // }

    // pub fn state(&mut self) -> &mut TableState {
    //     &mut self.table.state
    // }

    pub fn open(&self) -> Result<(), Box<dyn std::error::Error>> {
        let selected_id = self.table.state.selected();
        if selected_id.is_none() {
            return Ok(());
        }
        let selected_id = selected_id.unwrap();

        let item = &self.table.items[selected_id];

        let res = open::that(item.url().as_str());
        match res {
            Ok(_) => Ok(()),
            Err(err) => {
                Err(From::from(format!(
                    "failed to open URL in the browser: {}", err.to_string()
                )))
            }
        }
    }

    pub fn search(&mut self, phrase: &str) -> Result<(), Box<dyn std::error::Error>> {
        // TODO: filter and refresh
        let filter: Box<dyn Filter> = Box::new(FilterSet::new_combined_for_phrase(phrase));
        let urls = self.registry.list_urls(Some(filter))?;

        self.table.override_items(URLItem::from_vec(urls));

        Ok(())
    }

    pub fn tag(&mut self, tag: String) {
        // TODO: Tag selected URL
    }

    pub fn delete(&mut self) {
        // TODO: Delete selected URL
    }



}

impl BookmarksTable {

    pub fn new(registry: Box<dyn Registry>, table: StatefulTable<URLItem>) -> BookmarksTable {
        BookmarksTable{
            registry,
            table,
        }
    }

}
