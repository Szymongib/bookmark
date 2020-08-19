use crate::interactive::command::{Command, Execute};
use bookmark_lib::Registry;
use tui::backend::Backend;
use crate::interactive::modules::{HandleInput, Draw};
use crate::interactive::interface::InputMode;
use crate::interactive::table::StatefulTable;
use crate::interactive::url_table_item::URLItem;
use std::error::Error;
use termion::event::Key;
use tui::Frame;
use crate::interactive::helpers::get_selected_item;

pub struct TagCmd {

}

// impl<R: Registry, B: Backend> Command<R,B> for TagCmd{ }
//
// impl<R: Registry> HandleInput<R> for TagCmd {
//     fn try_activate(&mut self, input: Key, registry: &R, table: &mut StatefulTable<URLItem>) -> Result<Option<InputMode>, Box<dyn Error>> {
//         unimplemented!()
//     }
//
//     fn handle_input(&mut self, input: Key, registry: &R, table: &mut StatefulTable<URLItem>) -> Result<Option<InputMode>, Box<dyn Error>> {
//         unimplemented!()
//     }
// }
//
// impl<B: Backend> Draw<B> for TagCmd {
//     fn draw(&self, mode: InputMode, f: &mut Frame<B>) {
//         unimplemented!()
//     }
// }

impl<R: Registry> Execute<R> for TagCmd {
    fn execute(&self, registry: &R, table: &mut StatefulTable<URLItem>) -> Result<bool, Box<dyn Error>> {
        let url_item = get_selected_item(registry, table)?;
        if url_item.is_none() {
            // TODO: what should be return if failed? Probably error
            return Ok(false)
        }

        registry.

        Ok(true)
    }
}