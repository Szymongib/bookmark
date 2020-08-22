use crate::interactive::bookmarks_table::BookmarksTable;
use crate::interactive::modules::{Module, HandleInput, Draw};
use tui::backend::Backend;
use crate::interactive::interface::{InputMode, SuppressedAction};
use std::error::Error;
use termion::event::Key;
use tui::Frame;
use crate::interactive::widgets::rect::centered_rect;
use tui::widgets::{Paragraph, Clear, Block, Borders};
use tui::style::{Style, Color, Modifier};
use tui::layout::Alignment;
use tui::text::{Span, Spans};

pub(crate) struct HelpPanel {}

impl<B: Backend> Module<B> for HelpPanel {}

impl HandleInput for HelpPanel {
    fn try_activate(&mut self, input: Key, _table: &mut BookmarksTable) -> Result<Option<InputMode>, Box<dyn Error>> {
        if input != Key::Char('h') {
            return Ok(None)
        }

        return Ok(Some(InputMode::Suppressed(SuppressedAction::ShowHelp)))
    }

    fn handle_input(&mut self, input: Key, _table: &mut BookmarksTable) -> Result<Option<InputMode>, Box<dyn Error>> {
        match input {
            Key::Esc | Key::Char('\n') | Key::Char('h') => {
                return Ok(Some(InputMode::Normal));
            }
            Key::Char('q') => {
                return Ok(Some(InputMode::Normal));
            }
            _ => {}
        }

        return Ok(None)
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
        return HelpPanel{}
    }

    fn show_help_popup<'a, B: Backend>(&self, f: &mut Frame<B>) {
        let text = vec![
            Spans::from("'ENTER'            - open bookmarked URL"),
            Spans::from("'/' or 'CTRL + F'  - search for URLs"),
            Spans::from("'d'                - delete URL"),
        ];

        let block = Block::default()
            .borders(Borders::ALL)
            .style(Style::default().bg(Color::Black).fg(Color::LightBlue))
            .title(Span::styled(
                "Help - press ESC to close".to_string(),
                Style::default().add_modifier(Modifier::BOLD),
            ));

        let area = centered_rect(30, 30, f.size());
        let paragraph = Paragraph::new(text)
            .style(Style::default().bg(Color::Black).fg(Color::White))
            .block(block)
            .alignment(Alignment::Left);

        f.render_widget(Clear, area);
        f.render_widget(paragraph, area);
    }

}
