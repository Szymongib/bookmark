use crate::interactive::table::StatefulTable;
use crate::interactive::url_table_item::URLItem;
use bookmark_lib::Registry;

pub struct BookmarksTable<R: Registry> {
    registry: R,// TODO: or just Box<dyn Registry>?
    table: StatefulTable<URLItem>
}

impl<R: Registry> BookmarksTable<R> {
    pub fn next(&mut self) {
        self.table.next()
    }

    pub fn previous(&mut self) {
        self.table.previous()
    }

    pub fn unselect(&mut self) {
        self.table.unselect()
    }

    pub fn open(&self) {
        // TODO: open selected URL
    }

    pub fn search(&mut self, phrase: String) {
        // TODO: filter and refresh
    }

    pub fn tag(&mut self, tag: String) {
        // TODO: Tag selected URL
    }

    pub fn delete(&mut self) {
        // TODO: Delete selected URL
    }



}