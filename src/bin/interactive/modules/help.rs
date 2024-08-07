use crate::interactive::bookmarks_table::BookmarksTable;
use crate::interactive::interface::{InputMode, SuppressedAction};
use crate::interactive::modules::{Draw, HandleInput, Module};
use crate::interactive::widgets::rect::centered_fixed_rect;
use ratatui::layout::Alignment;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Clear, Paragraph};
use ratatui::Frame;
use std::error::Error;
use termion::event::Key;

pub(crate) struct HelpPanel {}

impl Module for HelpPanel {}

impl HandleInput for HelpPanel {
    fn try_activate(
        &mut self,
        input: Key,
        _table: &mut BookmarksTable,
    ) -> Result<Option<InputMode>, Box<dyn Error>> {
        if input != Key::Char('h') {
            return Ok(None);
        }

        Ok(Some(InputMode::Suppressed(SuppressedAction::ShowHelp)))
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

        Ok(None)
    }
}

impl Draw for HelpPanel {
    fn draw(&self, mode: InputMode, f: &mut Frame) {
        if mode == InputMode::Suppressed(SuppressedAction::ShowHelp) {
            self.show_help_popup(f);
        }
    }
}

impl HelpPanel {
    pub fn new() -> HelpPanel {
        HelpPanel {}
    }

    // TODO: consider using consts from cmd - or embedding docs?
    fn show_help_popup(&self, f: &mut Frame) {
        let text = vec![
            "Action               Description",
            "'ENTER'            | open bookmarked URL",
            "'/' or 'CTRL + F'  | search for URLs",
            "'d'                | delete URL",
            "'i'                | show/hide ids",
            "'q'                | exit interactive mode",
            "':'                | go to command mode",
            "",
            "",
            "Command                Alias     Description",
            "':tag <TAG_NAME>'    |         | add tag <TAG_NAME> to selected bookmark",
            "':untag <TAG_NAME>'  |         | remove tag <TAG_NAME> from selected bookmark",
            "':chgroup <GROUP>'   | chg     | change group to <GROUP> for selected bookmark",
            "':chname <NAME>'     | chn     | change name to <NAME> for selected bookmark",
            "':churl <URL>'       | chu     | change url to <URL> for selected bookmark",
            "':sort [SORT_BY]'    |         | sort bookmarks by one of: [name, url, group]",
            "':q'                 | quit    | exit interactive mode",
            "",
        ];
        let max_width = text.iter().map(|t| t.len()).max().unwrap_or_default() as u16;
        let lines: Vec<Line> = text.iter().map(|t| Line::from(t.to_owned())).collect();

        let block = Block::default()
            .borders(Borders::ALL)
            .style(Style::default().bg(Color::Black).fg(Color::LightBlue))
            .title(Span::styled(
                "Help - press ESC to close".to_string(),
                Style::default().add_modifier(Modifier::BOLD),
            ));

        let area = centered_fixed_rect(max_width + 4, text.len() as u16 + 2, f.size());
        let paragraph = Paragraph::new(lines)
            .style(Style::default().bg(Color::Black).fg(Color::White))
            .block(block)
            .alignment(Alignment::Left);

        f.render_widget(Clear, area);
        f.render_widget(paragraph, area);
    }
}
