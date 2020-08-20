use crate::interactive::command::{Command, Execute};
use crate::interactive::command;
use bookmark_lib::Registry;
use tui::backend::Backend;
use crate::interactive::modules::{HandleInput, Draw};
use crate::interactive::interface::InputMode;
use crate::interactive::table::StatefulTable;
use crate::interactive::url_table_item::URLItem;
use termion::event::Key;
use tui::Frame;
use crate::interactive::helpers::get_selected_item;

pub struct TagAction {}

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

// TODO: custom errors - command input error
impl<R: Registry> Execute<R> for TagAction {
    fn execute(&self, registry: &R, table: &mut StatefulTable<URLItem>, args: Vec<&str>) -> Result<bool, command::Error> {
        let url_item = get_selected_item(registry, table)?;
        if url_item.is_none() {
            // TODO: what should be return if failed? Probably error
            return Err(command::Error::new_input("URL record not selected"))
        }

        if args.len() < 1 {
            return Err(command::Error::new_input("Error: need at least one argument [TAG_NAME]"))
        }

        let tag_name = args[0].clone();
        if tag_name == "" {
            return Err(command::Error::new_input("Error: need at least one argument [TAG_NAME]"))
        }

        // TODO: validate tag

        let record = registry.tag_url(url_item.unwrap().id, tag_name.to_string())
            .map_err(|err| {command::Error::new_internal(err.to_string())})?;

        return Ok(record.map_or(false, |_| true));
    }
}

impl TagAction {

    pub fn new() -> TagAction {
        TagAction{}
    }

}