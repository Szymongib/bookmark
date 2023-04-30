use std::io::Stdout;
use std::{error::Error, io};
use bookmark_lib::import::{ImportItem};
use bookmark_lib::import::brave::bookmarks::Bookmarks;
use termion::raw::RawTerminal;
use termion::{input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};
use tui::{backend::TermionBackend, Terminal};

use super::event::Events;
use super::import::import::ImportsTable;
use super::import::interface::ImportInterface;

use crate::interactive::bookmarks_table::BookmarksTable;
use crate::interactive::interface::Interface;
use bookmark_lib::Registry;

pub fn enter_interactive_mode<T: Registry + 'static>(registry: T) -> Result<(), Box<dyn Error>> {
    let mut terminal = prepare_interactive_terminal()?;

    let events = Events::new();

    let bookmarks_table = BookmarksTable::new(events.tx.clone(), Box::new(registry));

    let mut user_interface = Interface::new(bookmarks_table?)?;

    loop {
        terminal.draw(|f| user_interface.draw(f))?;

        let quit = user_interface.handle_input(events.next()?)?;
        if quit {
            return Ok(());
        }
    }
}

pub fn enter_interactive_import<T: Registry + 'static>(registry: T, imports: Vec<ImportItem>) -> Result<(), Box<dyn Error>> {
    let mut terminal = prepare_interactive_terminal()?;

    let events = Events::new();

    let imports_table = ImportsTable::new(events.tx.clone(), Box::new(registry), imports)?;

    let mut user_interface = ImportInterface::new(imports_table);

    loop {
        terminal.draw(|f| user_interface.draw(f))?;

        let quit = user_interface.handle_input(events.next()?)?;
        if quit {
            return Ok(());
        }
    }
}

fn prepare_interactive_terminal() -> Result<Terminal<TermionBackend<AlternateScreen<MouseTerminal<RawTerminal<Stdout>>>>>, Box<dyn Error>> {
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;
    Ok(terminal)
}
