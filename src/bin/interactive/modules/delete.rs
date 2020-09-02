use crate::interactive::bookmarks_table::BookmarksTable;
use crate::interactive::interface::{InputMode, SuppressedAction};
use crate::interactive::modules::{Draw, HandleInput, Module};
use crate::interactive::widgets::rect::centered_fixed_rect;
use bookmark_lib::types::URLRecord;
use std::error::Error;
use termion::event::Key;
use tui::backend::Backend;
use tui::layout::Alignment;
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, Clear, Paragraph};
use tui::Frame;

// TODO: consider some generic mechanism for actions that require confirmation

pub(crate) struct Delete {
    record: Option<URLRecord>,
}

impl<B: Backend> Module<B> for Delete {}

impl HandleInput for Delete {
    fn try_activate(
        &mut self,
        input: Key,
        table: &mut BookmarksTable,
    ) -> Result<Option<InputMode>, Box<dyn Error>> {
        if input != Key::Char('d') {
            return Ok(None);
        }

        self.record = table.get_selected()?;
        if self.record.is_none() {
            return Ok(Some(InputMode::Normal));
        }

        Ok(Some(InputMode::Suppressed(SuppressedAction::Delete)))
    }

    fn handle_input(
        &mut self,
        input: Key,
        table: &mut BookmarksTable,
    ) -> Result<Option<InputMode>, Box<dyn Error>> {
        match input {
            Key::Char('\n') => {
                table.delete()?;
                return Ok(Some(InputMode::Normal));
            }
            Key::Char('q') | Key::Esc => {
                return Ok(Some(InputMode::Normal));
            }
            _ => {}
        }

        Ok(None)
    }
}

impl<B: Backend> Draw<B> for Delete {
    fn draw(&self, mode: InputMode, f: &mut Frame<B>) {
        if let InputMode::Suppressed(SuppressedAction::Delete) = mode {
            self.confirm_delete_popup(f)
        }
    }
}

impl Delete {
    pub fn new() -> Delete {
        Delete { record: None }
    }

    fn confirm_delete_popup<B: Backend>(&self, f: &mut Frame<B>) {
        let area = centered_fixed_rect(50, 10, f.size());

        let record = self
            .record
            .clone()
            .expect("Error displaying delete confirmation");

        let text = vec![
            Spans::from(""),
            Spans::from(format!(
                "Delete '{}' from '{}' group?",
                record.name, record.group
            )),
            Spans::from(""),
            Spans::from("Yes (Enter)   ---   No (ESC)"), // TODO: consider y and n as confirmation
        ];

        // TODO: remove duplicated code
        let block = Block::default()
            .borders(Borders::ALL)
            .style(Style::default().bg(Color::Black).fg(Color::LightBlue))
            .title(Span::styled(
                "Confirm deletion".to_string(),
                Style::default().add_modifier(Modifier::BOLD),
            ));

        let paragraph = Paragraph::new(text)
            .style(Style::default().bg(Color::Black).fg(Color::White))
            .block(block)
            .alignment(Alignment::Center);

        f.render_widget(Clear, area);
        f.render_widget(paragraph, area);
    }
}
