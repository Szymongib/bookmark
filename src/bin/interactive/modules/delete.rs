use crate::interactive::bookmarks_table::BookmarksTable;
use tui::backend::Backend;
use crate::interactive::modules::{Module, HandleInput, Draw};
use termion::event::Key;
use crate::interactive::interface::{InputMode, SuppressedAction};
use std::error::Error;
use tui::Frame;
use crate::interactive::widgets::rect::centered_fixed_rect;
use tui::widgets::{Paragraph, Clear, Block, Borders};
use tui::style::{Style, Color, Modifier};
use tui::layout::Alignment;
use bookmark_lib::types::URLRecord;
use tui::text::{Span, Spans};


// TODO: consider some generic mechanism for actions that require confirmation

pub(crate) struct Delete {
    record: Option<URLRecord>,
}

impl<B: Backend> Module<B> for Delete {}

impl HandleInput for Delete {
    fn try_activate(&mut self, input: Key, table: &mut BookmarksTable) -> Result<Option<InputMode>, Box<dyn Error>> {
        if input != Key::Char('d') {
            return Ok(None)
        }

        self.record  = table.get_selected()?;
        if self.record .is_none() {
            return Ok(Some(InputMode::Normal))
        }

        return Ok(Some(InputMode::Suppressed(SuppressedAction::Delete)))
    }

    fn handle_input(&mut self, input: Key, table: &mut BookmarksTable) -> Result<Option<InputMode>, Box<dyn Error>> {
        match input {
            Key::Char('\n') => {
                table.delete();
                return Ok(Some(InputMode::Normal));
            }
            Key::Char('q') | Key::Esc => {
                return Ok(Some(InputMode::Normal));
            }
            _ => {}
        }

        return Ok(None)
    }
}

impl<B: Backend> Draw<B> for Delete {
    fn draw(&self, mode: InputMode, f: &mut Frame<B>) {
        match mode {
            InputMode::Suppressed(SuppressedAction::Delete) => {
                self.confirm_delete_popup(f)
            }
            _ => {}
        }
    }
}

impl Delete {

    pub fn new() -> Delete {
        return Delete{record: None}
    }

    fn confirm_delete_popup<B: Backend>(&self, f: &mut Frame<B>) {
        let area = centered_fixed_rect(50, 10, f.size());

        let record = self.record.clone().expect("Error displaying delete confirmation").clone();

        let text = vec![
            Spans::from(""),
            Spans::from(format!("Delete '{}' from '{}' group?", record.name, record.group)),
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
