use crate::interactive::bookmarks_table::BookmarksTable;
use crate::interactive::interface::{InputMode, SuppressedAction};
use crate::interactive::modules::{Draw, HandleInput, Module};
use std::error::Error;
use termion::event::Key;
use tui::backend::Backend;
use tui::layout::Alignment;
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, Clear, Paragraph};
use tui::Frame;
use crate::interactive::widgets::rect::centered_fixed_rect;

pub(crate) struct HelpPanel {}

impl<B: Backend> Module<B> for HelpPanel {}

impl HandleInput for HelpPanel {
    fn try_activate(
        &mut self,
        input: Key,
        _table: &mut BookmarksTable,
    ) -> Result<Option<InputMode>, Box<dyn Error>> {
        if input != Key::Char('h') {
            return Ok(None);
        }

        return Ok(Some(InputMode::Suppressed(SuppressedAction::ShowHelp)));
    }

    fn handle_input(
        &mut self,
        input: Key,
        _table: &mut BookmarksTable,
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

        return Ok(None);
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
    pub fn new() -> HelpPanel {
        return HelpPanel {};
    }

    fn show_help_popup<'a, B: Backend>(&self, f: &mut Frame<B>) {
        let text = vec![
            "'ENTER'            - open bookmarked URL",
            "'/' or 'CTRL + F'  - search for URLs",
            "'d'                - delete URL",
            "':'                - go to command mode",
            "",
            "'Commands",
            "':tag [TAG_NAME]'  - add tag to selected bookmark",
            "':q'               - quit",
        ];
        let max_width = text.iter().map(|t| t.len()).max().unwrap_or_default() as u16;
        let spans: Vec<Spans> = text.iter().map(|t| {
            Spans::from(t.to_owned())
        }).collect();

        let block = Block::default()
            .borders(Borders::ALL)
            .style(Style::default().bg(Color::Black).fg(Color::LightBlue))
            .title(Span::styled(
                "Help - press ESC to close".to_string(),
                Style::default().add_modifier(Modifier::BOLD),
            ));

        let area = centered_fixed_rect(max_width+4, text.len() as u16 + 2, f.size());
        let paragraph = Paragraph::new(spans)
            .style(Style::default().bg(Color::Black).fg(Color::White))
            .block(block)
            .alignment(Alignment::Left);

        f.render_widget(Clear, area);
        f.render_widget(paragraph, area);
    }
}
