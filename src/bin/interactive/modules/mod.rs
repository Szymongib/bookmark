use crate::interactive::bookmarks_table::BookmarksTable;
use crate::interactive::interface::InputMode;
use ratatui::Frame;
use termion::event::Key;

pub mod command;
pub mod delete;
pub mod help;
pub mod search;

pub trait Module: HandleInput + Draw {}

pub trait HandleInput {
    /// Activates Module
    fn try_activate(
        &mut self,
        input: Key,
        table: &mut BookmarksTable,
    ) -> Result<Option<InputMode>, Box<dyn std::error::Error>>;
    /// Handles input key when Module already active
    fn handle_input(
        &mut self,
        input: Key,
        table: &mut BookmarksTable,
    ) -> Result<Option<InputMode>, Box<dyn std::error::Error>>;
}

pub trait Draw {
    fn draw(&self, mode: InputMode, f: &mut Frame);
}
