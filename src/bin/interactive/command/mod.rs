use bookmark_lib::Registry;
use crate::interactive::table::StatefulTable;
use crate::interactive::url_table_item::URLItem;
use tui::backend::Backend;
use crate::interactive::modules::{HandleInput, Draw};

pub mod tag;

pub trait Command<R: Registry, B: Backend>: HandleInput<R> + Draw<B> + Execute<R> {}

pub trait Execute<R: Registry> {
    fn execute(&self, registry: &R, table: &mut StatefulTable<URLItem>) -> Result<(), Box<dyn std::error::Error>>;
}
