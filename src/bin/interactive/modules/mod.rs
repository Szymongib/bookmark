use crate::interactive::bookmarks_table::BookmarksTable;
use crate::interactive::interface::InputMode;
use termion::event::Key;
use tui::backend::Backend;
use tui::Frame;

use super::app_event::AppEvent;
use super::import::import::ImportsTable;

pub mod command;
pub mod delete;
pub mod help;
pub mod search;
pub mod edit_modal;
pub mod info_popup;

pub trait BookmarksModule<B: Backend>: HandleBookmarksInput + Draw<B> {}

pub trait HandleBookmarksInput {
    /// Activates Module
    fn try_activate(
        &mut self,
        app_event: &AppEvent,
        table: &mut BookmarksTable,
    ) -> Result<Option<InputMode>, Box<dyn std::error::Error>>;
    /// Handles input key when Module already active
    fn handle_input(
        &mut self,
        input: Key,
        table: &mut BookmarksTable,
    ) -> Result<Option<InputMode>, Box<dyn std::error::Error>>;
}

pub trait Draw<B: Backend> {
    fn draw(&self, mode: InputMode, f: &mut Frame<B>);
}

pub trait ImportsModule<B: Backend>: HandleImportsInput + Draw<B> {}

pub trait HandleImportsInput {
    /// Activates Module
    fn try_activate(
        &mut self,
        app_event: &AppEvent,
        table: &mut ImportsTable,
    ) -> Result<Option<InputMode>, Box<dyn std::error::Error>>;
    /// Handles input key when Module already active
    fn handle_input(
        &mut self,
        input: Key,
        table: &mut ImportsTable,
    ) -> Result<Option<InputMode>, Box<dyn std::error::Error>>;
}