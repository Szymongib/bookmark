use bookmark_lib::Registry;
use tui::backend::Backend;
use crate::interactive::interface::InputMode;
use termion::event::Key;
use tui::Frame;
use crate::interactive::table::StatefulTable;
use crate::interactive::url_table_item::URLItem;

pub mod search;
pub mod help;

pub trait Module<R: Registry, B: Backend>: HandleInput<R> + Draw<B> {}

pub trait HandleInput<R: Registry> {
    fn handle_input(&mut self, input: Key, registry: &R, table: &mut StatefulTable<URLItem>) -> Result<InputMode, Box<dyn std::error::Error>>;
}

pub trait Draw<B: Backend> {
    fn draw(&self, mode: InputMode, f: &mut Frame<B>);
}