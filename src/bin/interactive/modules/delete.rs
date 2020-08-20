use bookmark_lib::Registry;
use tui::backend::Backend;
use crate::interactive::modules::{Module, HandleInput, Draw};
use termion::event::Key;
use crate::interactive::table::{StatefulTable, TableItem};
use crate::interactive::url_table_item::{URLItem, URLItemSource};
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

impl<R: Registry, B: Backend> Module<R, B> for Delete {}

impl<R: Registry> HandleInput<R> for Delete {
    fn try_activate(&mut self, input: Key, registry: &R, table: &mut StatefulTable<URLItemSource<R>, URLItem>) -> Result<Option<InputMode>, Box<dyn Error>> {
        if input != Key::Char('d') {
            return Ok(None)
        }

        let item_id = self.get_selected_item_id(table);
        if item_id.is_none() {
            return Ok(Some(InputMode::Normal))
        }

        self.record  = registry.get_url(item_id.unwrap())?;
        if self.record .is_none() {
            return Ok(Some(InputMode::Normal))
        }

        return Ok(Some(InputMode::Suppressed(SuppressedAction::Delete)))
    }

    fn handle_input(&mut self, input: Key, registry: &R, table: &mut StatefulTable<URLItemSource<R>, URLItem>) -> Result<Option<InputMode>, Box<dyn Error>> {
        match input {
            Key::Char('\n') => {
                self.delete_url(registry, table)?;
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

    fn get_selected_item_id<R: Registry>(&self, table: &mut StatefulTable<URLItemSource<R>, URLItem>) -> Option<String> {
        let selected_id = table.state.selected();
        if selected_id.is_none() {
            return None;
        }

        let item_id = table.visible[selected_id.unwrap()].id();
        Some(item_id)
    }

    fn delete_url<R: Registry>(&self, registry: &R, table: &mut StatefulTable<URLItemSource<R>, URLItem>) -> Result<(), Box<dyn Error>> {
        let url_id = self.get_selected_item_id(table);
        if url_id.is_none() {
            return Ok(())
        }
        let url_id = url_id.unwrap();

        let deleted = registry.delete_by_id(&url_id)?;
        if deleted {
            Delete::remove_item(url_id.clone(), &mut table.items);
            Delete::remove_item(url_id, &mut table.visible);
        }

        Ok(())
    }

    fn remove_item(item_id: String, vec: &mut Vec<URLItem>) {
        let mut i: usize = 0;
        for item in vec.clone() {
            if item.id() == item_id {
                break
            }
            i+=1;
        }
        vec.remove(i);
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
