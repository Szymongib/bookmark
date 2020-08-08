use std::{error::Error, io};
use termion::{input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};
use tui::{backend::TermionBackend, Terminal};

use super::event::Events;

use crate::ui::interface::Interface;
use bookmark_lib::types::URLRecord;

pub fn display_urls(urls: Vec<URLRecord>) -> Result<(), Box<dyn Error>> {
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    let events = Events::new();

    let mut user_interface = Interface::new(urls);

    loop {
        terminal.draw(|f| user_interface.draw(f))?;

        let quit = user_interface.handle_input(events.next()?)?;
        if quit {
            return Ok(());
        }
    }
}
