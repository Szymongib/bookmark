use crate::interactive::app_event::{self, AppEvent};
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

pub(crate) struct HelpPanel {
    max_width: u16,
    help_text_spans: Vec<Spans<'static>>,
}

impl<B: Backend> BookmarksModule<B> for HelpPanel {}

impl HandleBookmarksInput for HelpPanel {
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

impl<B: Backend> ImportsModule<B> for HelpPanel {}

impl HandleImportsInput for HelpPanel {
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

impl<B: Backend> Draw<B> for HelpPanel {
    fn draw(&self, mode: InputMode, f: &mut Frame<B>) {
        if mode == InputMode::Suppressed(SuppressedAction::ShowHelp) {
            self.show_help_popup(f);
        }
    }
}

impl HelpPanel {
    pub fn new(help_text: &'static str) -> HelpPanel {
        let max_width = help_text.lines().map(|l| l.len()).max().unwrap_or_default() as u16;
        HelpPanel {
            max_width,
            help_text_spans: help_text.lines().map(|l| Spans::from(l.to_owned())).collect(),
        }
    }

    fn try_activate(
        &mut self,
        app_event: &AppEvent,
    ) -> Result<Option<InputMode>, Box<dyn Error>> {
        if app_event != &AppEvent::ShowHelpPopup {
            return Ok(None);
        }

        Ok(Some(InputMode::Suppressed(SuppressedAction::ShowHelp)))
    }

    fn handle_input(
        &mut self,
        input: Key,
    ) -> Result<Option<InputMode>, Box<dyn Error>> {
        match input {
            Key::Esc | Key::Char('\n') | Key::Char('h') => {
                return Ok(Some(InputMode::Normal));
            }
            Key::Char('q') => {
                return Ok(Some(InputMode::Normal));
            }
            _ => {}
        }

        Ok(None)
    }

    fn show_help_popup<B: Backend>(&self, f: &mut Frame<B>) {
        let block = Block::default()
            .borders(Borders::ALL)
            .style(Style::default().bg(Color::Black).fg(Color::LightBlue))
            .title(Span::styled(
                "Help - press ESC to close".to_string(),
                Style::default().add_modifier(Modifier::BOLD),
            ));

        let area = centered_fixed_rect(self.max_width + 4, self.help_text_spans.len() as u16 + 2, f.size());
        let paragraph = Paragraph::new(self.help_text_spans.clone())
            .style(Style::default().bg(Color::Black).fg(Color::White))
            .block(block)
            .alignment(Alignment::Left);

        f.render_widget(Clear, area);
        f.render_widget(paragraph, area);
    }
}
