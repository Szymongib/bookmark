use crate::interactive::helpers::pad_layout;
use crate::interactive::import::import::ImportsTable;
use crate::interactive::import::import_table_item::{ImportURLTableItem, ImportTableItem};
use crate::interactive::interface::{InputMode, SuppressedAction};
use crate::interactive::modules::{Draw};
use crate::interactive::widgets::rect::centered_fixed_rect;
use std::error::Error;
use bookmark_lib::import::{ImportURLItem};
use termion::event::Key;
use tui::backend::Backend;
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, Clear, Paragraph};
use tui::Frame;

use super::{HandleImportsInput, ImportsModule};

pub(crate) struct EditModal {
    // TODO: make it generic later to also be able to edit bookmarks
    editing_import: Option<ImportURLItem>,

    edits: Vec<String>,
    active_edit: usize,
}
impl<B: Backend> ImportsModule<B> for EditModal {}

impl HandleImportsInput for EditModal {
    fn try_activate(
        &mut self,
        input: Key,
        table: &mut ImportsTable,
    ) -> Result<Option<InputMode>, Box<dyn Error>> {
        self.try_activate(input, table)
    }

    fn handle_input(
        &mut self,
        input: Key,
        table: &mut ImportsTable,
    ) -> Result<Option<InputMode>, Box<dyn Error>> {
        self.handle_input(input, table)
    }
}

impl<B: Backend> Draw<B> for EditModal {
    fn draw(&self, mode: InputMode, f: &mut Frame<B>) {
        if mode == InputMode::Suppressed(SuppressedAction::Edit) {
            self.show_edit_modal(f);
        }
    }
}

impl EditModal {
    pub fn new() -> EditModal {
        EditModal {
            editing_import: None,
            edits: vec!["".to_string(); 2],
            active_edit: 0,
        }
    }

    fn try_activate(
        &mut self,
        input: Key,
        table: &mut ImportsTable,
    ) -> Result<Option<InputMode>, Box<dyn Error>> {
        if input != Key::Char('e') {
            return Ok(None);
        }

        let import_url = match self.selected_import_url(table) {
            Some(url) => url.inner.clone(),
            None => return Ok(None),
        };

        // TODO: come up with better logging mechanism...
        eprintln!("Activating edit modal for import: {:?}", import_url);

        self.edits = vec![import_url.name.clone(), import_url.url.clone()];
        self.editing_import = Some(import_url);

        Ok(Some(InputMode::Suppressed(SuppressedAction::Edit)))
    }

    fn selected_import_url<'a>(&self, table: &'a mut ImportsTable) -> Option<&'a mut ImportURLTableItem> {
        let index = table.table().state.selected()?;
        let import = &mut table.table().items[index];

        match import {
            ImportTableItem::URL(url) => Some(url),
            ImportTableItem::Folder(_) => None,
        }
    }

    fn handle_input(
        &mut self,
        input: Key,
        table: &mut ImportsTable,
    ) -> Result<Option<InputMode>, Box<dyn Error>> {
        match input {
            Key::Char('\t') => {
                self.active_edit = (self.active_edit + 1) % self.edits.len();
            }
            Key::Char('\n') => {
                let url = self.selected_import_url(table)
                    .expect("failed to find editing import!"); // This should not happen hence panic

                url.inner.name = self.edits[0].clone();
                url.inner.url = self.edits[1].clone();
                url.refresh();

                return Ok(Some(InputMode::Normal));
            }
            Key::Char(c) => {
                self.edits[self.active_edit].push(c);
            }
            Key::Backspace => {
                self.edits[self.active_edit].pop();
            }
            Key::Esc => {
                return Ok(Some(InputMode::Normal));
            }
            _ => {}
        }

        Ok(None)
    }

    fn show_edit_modal<B: Backend>(&self, f: &mut Frame<B>) {
        if self.editing_import.is_none() {
            return;
        }

        let import = self.editing_import.as_ref().unwrap();

        // We strech it wide in case of large URLs or long names
        let width = (f.size().width - 4).min(120);
        let heihgt = 13;
        
        // Create centered rectangle
        let area = centered_fixed_rect(width, heihgt, f.size());

        let block = Block::default()
            .borders(Borders::ALL)
            .style(Style::default().bg(Color::Black).fg(Color::LightBlue))
            .title(Span::styled(
                "ENTER to save ESC to close TAB to change field".to_string(),
                Style::default().add_modifier(Modifier::BOLD),
            ));
        
        f.render_widget(Clear, area);
        f.render_widget(block, area);

        let inner_layout = pad_layout(area, [1,1,1,1]);
    
        // Split it into 2 even chunks + padd in between
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Length(5),
                    Constraint::Length(1),
                    Constraint::Length(5),
                ]
                .as_ref(),
            )
            .split(inner_layout);

        self.render_input_fields(
            &format!("Name:  {}", import.name),
            0,
            chunks[0], 
            f,
        );
        self.render_input_fields(
            &format!("URL:  {}", import.url),
            1,
            chunks[2], 
            f,
        )
    }

    fn render_input_fields<B: Backend>(
        &self,
        input_header: &str,
        edit_index: usize,
        area: Rect,
        f: &mut Frame<B>,
    ) {
        let name_input_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Length(1),
                    Constraint::Length(1),
                    Constraint::Length(3),
                ]
                .as_ref(),
            )
            .split(area);


        let input_block = Block::default()
            .borders(Borders::ALL)
            .style(Style::default().bg(Color::Black).fg(Color::LightBlue));

        let name_paragraph = Paragraph::new(Spans::from(input_header))
            .style(Style::default().bg(Color::Black).fg(Color::White))
            .alignment(Alignment::Left);
        let new_name_paragraph = Paragraph::new(Spans::from("Enter new value:"))
            .style(Style::default().bg(Color::Black).fg(Color::White))
            .alignment(Alignment::Left);
        let name_input = Paragraph::new(Spans::from(self.edits[edit_index].clone()))
            .style(
                self.get_input_style(edit_index)
            )
            .block(input_block)
            .alignment(Alignment::Left);
        
        f.render_widget(name_paragraph, name_input_layout[0]);
        f.render_widget(new_name_paragraph, name_input_layout[1]);
        f.render_widget(name_input, name_input_layout[2]);
    }

    fn get_input_style(&self, input_index: usize) -> Style {
        if input_index == self.active_edit {
            // TODO: use better colors
            Style::default().bg(Color::LightYellow).fg(Color::LightBlue)
        } else {
            Style::default().bg(Color::Black).fg(Color::White)
        }
    }
}
