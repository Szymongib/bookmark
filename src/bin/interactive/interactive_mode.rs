use std::{error::Error, io};
use termion::{input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};
use tui::{backend::TermionBackend, Terminal};

use super::event::Events;

use crate::interactive::interface::Interface;
use bookmark_lib::Registry;
use crate::interactive::bookmarks_table::BookmarksTable;
use crate::interactive::url_table_item::URLItem;
use crate::interactive::table::StatefulTable;


pub fn enter_interactive_mode<T: Registry + 'static>(registry: T) -> Result<(), Box<dyn Error>> {
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    let events = Events::new();

    let items: Vec<URLItem> = URLItem::from_vec(registry.list_urls(None)?);
    let table = StatefulTable::with_items(items);
    let bookmarks_table = BookmarksTable::new(&events.tx, Box::new(registry), table);

    let mut user_interface = Interface::new(bookmarks_table)?;

    loop {
        terminal.draw(|f| user_interface.draw(f))?;

        let quit = user_interface.handle_input(events.next()?)?;
        if quit {
            return Ok(());
        }
    }
}
