use crate::interactive::bookmarks_table::BookmarksTable;
use crate::interactive::interface::{InputMode, SuppressedAction};
use crate::interactive::modules::{Draw, HandleInput, Module};
use crate::interactive::widgets::rect::centered_fixed_rect;
use bookmark_lib::types::URLRecord;
use ratatui::backend::Backend;
use ratatui::layout::Alignment;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Span, Text};
use ratatui::widgets::{Block, Borders, Clear, Paragraph};
use ratatui::Frame;
use std::error::Error;
use termion::event::Key;

// TODO: consider some generic mechanism for actions that require confirmation

pub(crate) struct Delete {
    record: Option<URLRecord>,
}

impl Module for Delete {}

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

impl Draw for Delete {
    fn draw(&self, mode: InputMode, f: &mut Frame) {
        if let InputMode::Suppressed(SuppressedAction::Delete) = mode {
            self.confirm_delete_popup(f)
        }
    }
}

impl Delete {
    pub fn new() -> Delete {
        Delete { record: None }
    }

    fn confirm_delete_popup(&self, f: &mut Frame) {
        let area = centered_fixed_rect(50, 10, f.size());

        let record = self
            .record
            .clone()
            .expect("Error displaying delete confirmation");

        let mut text = Text::raw("");
        text.push_line(format!(
            "Delete '{}' from '{}' group?",
            record.name, record.group
        ));
        text.push_line("");
        text.push_line("Yes (Enter)   ---   No (ESC)"); // TODO: consider y and n as confirmation

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
