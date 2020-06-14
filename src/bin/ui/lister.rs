use std::{error::Error, io};
use termion::{event::Key, input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};
use tui::{
    backend::TermionBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, Text},
    Terminal,
};

use super::event::{Event, Events};

use crate::ui::list::StatefulList;
use bookmark_lib::types::URLRecord;

pub fn display_urls(urls: Vec<URLRecord>) -> Result<(), Box<dyn Error>> {
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    let events = Events::new();

    let mut list = StatefulList::with_items(urls);

    loop {
        terminal.draw(|mut f| {
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(100), Constraint::Percentage(100)].as_ref())
                .split(f.size());

            let style = Style::default().fg(Color::White).bg(Color::Black);

            let items = list
                .items
                .iter()
                .map(|i| Text::raw(format!("{}:  {}", i.name.clone(), i.url.clone()))); // TODO: Make name and URL as separate columns
                                                                                        // TODO: Display tags

            let items = List::new(items)
                .block(Block::default().borders(Borders::ALL).title("URLs"))
                .style(style)
                .highlight_style(style.fg(Color::LightGreen).modifier(Modifier::BOLD))
                .highlight_symbol(">");
            f.render_stateful_widget(items, chunks[0], &mut list.state);
        })?;

        match events.next()? {
            Event::Input(input) => match input {
                Key::Char('q') => {
                    return Ok(());
                }
                Key::Left => {
                    list.unselect();
                }
                Key::Down => {
                    list.next();
                }
                Key::Up => {
                    list.previous();
                }
                Key::Char('\n') => {
                    let selected_id = list.state.selected();
                    if selected_id.is_none() {
                        continue;
                    }
                    let selected_id = selected_id.unwrap();

                    let item = &list.items[selected_id];

                    let res = open::that(item.url.as_str());
                    if let Err(err) = res {
                        return Err(From::from(format!(
                            "failed to open URL in the browser: {}",
                            err.to_string()
                        )));
                    }
                }
                _ => {}
            },
        }
    }
}
