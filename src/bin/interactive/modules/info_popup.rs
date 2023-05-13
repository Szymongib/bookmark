use crate::interactive::app_event::AppEvent;
use crate::interactive::bookmarks_table::BookmarksTable;
use crate::interactive::import::import::ImportsTable;
use crate::interactive::interface::{InputMode, SuppressedAction};
use crate::interactive::modules::{Draw, HandleBookmarksInput, BookmarksModule};
use crate::interactive::widgets::rect::centered_fixed_rect;
use std::error::Error;
use termion::event::Key;
use tui::backend::Backend;
use tui::layout::Alignment;
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, Clear, Paragraph};
use tui::Frame;

use super::{HandleImportsInput, ImportsModule};

const MIN_WIDTH: u16 = 30;
const MIN_HEIGHT: u16 = 4;

pub(crate) struct InfoPopup {
    message: Option<String>,
}

impl<B: Backend> BookmarksModule<B> for InfoPopup {}

impl HandleBookmarksInput for InfoPopup {
    fn try_activate(
        &mut self,
        app_event: &AppEvent,
        _table: &mut BookmarksTable,
    ) -> Result<Option<InputMode>, Box<dyn Error>> {
        self.try_activate(app_event)
    }

    fn handle_input(
        &mut self,
        input: Key,
        _table: &mut BookmarksTable,
    ) -> Result<Option<InputMode>, Box<dyn Error>> {
        self.handle_input(input)
    }
}

impl<B: Backend> ImportsModule<B> for InfoPopup {}

impl HandleImportsInput for InfoPopup {
    fn try_activate(
        &mut self,
        app_event: &AppEvent,
        _table: &mut ImportsTable,
    ) -> Result<Option<InputMode>, Box<dyn Error>> {
        self.try_activate(app_event)
    }

    fn handle_input(
        &mut self,
        input: Key,
        _table: &mut ImportsTable,
    ) -> Result<Option<InputMode>, Box<dyn Error>> {
        self.handle_input(input)
    }
}

impl<B: Backend> Draw<B> for InfoPopup {
    fn draw(&self, mode: InputMode, f: &mut Frame<B>) {
        if InputMode::InfoPopup == mode {
            self.show_info_popup(f);
        }
    }
}

impl InfoPopup {
    pub fn new() -> InfoPopup {
        InfoPopup {
            message: None,
        }
    }

    fn try_activate(
        &mut self,
        app_event: &AppEvent,
    ) -> Result<Option<InputMode>, Box<dyn Error>> {
        match app_event {
            AppEvent::ShowInfoPopup(msg) => {
                self.message = Some(msg.clone());
                return Ok(Some(InputMode::InfoPopup));
            }
            _ => {}
        }
        Ok(None)
    }

    fn handle_input(
        &mut self,
        input: Key,
    ) -> Result<Option<InputMode>, Box<dyn Error>> {
        match input {
            Key::Esc | Key::Char('\n') | Key::Char('q') => {
                return Ok(Some(InputMode::Normal));
            }
            _ => {}
        }

        Ok(None)
    }

    fn show_info_popup<B: Backend>(&self, f: &mut Frame<B>) {
        let block = Block::default()
            .borders(Borders::ALL)
            .style(Style::default().bg(Color::Black).fg(Color::LightBlue))
            .title(Span::styled(
                "Info - ENTER / ESC to continue".to_string(),
                Style::default().add_modifier(Modifier::BOLD),
            ));
        
        let message = self.message.as_ref()
            .map(|l| l.as_str()).unwrap_or_default();
        let lines = message.lines();
        let height = (lines.clone().count() as u16).max(MIN_HEIGHT);
        let width = (lines.map(|l| l.len()).max().unwrap_or(0) as u16).max(MIN_WIDTH);
        let spans = Spans::from(message);

        let area = centered_fixed_rect(width + 4, height + 2, f.size());
        let paragraph = Paragraph::new(spans)
            .style(Style::default().bg(Color::Black).fg(Color::White))
            .block(block)
            .alignment(Alignment::Center);

        f.render_widget(Clear, area);
        f.render_widget(paragraph, area);
    }
}
