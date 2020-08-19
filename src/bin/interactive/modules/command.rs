use termion::event::Key;
use tui::backend::Backend;
use tui::Frame;
use crate::interactive::table::StatefulTable;
use crate::interactive::url_table_item::URLItem;
use crate::interactive::interface::InputMode;
use bookmark_lib::Registry;
use bookmark_lib::record_filter::FilterSet;
use tui::widgets::{Paragraph, Block, Borders, Clear};
use tui::style::{Style, Color};
use tui::layout::{Rect, Layout, Direction, Constraint};
use crate::interactive::modules::{HandleInput, Draw, Module};
use std::error::Error;


pub(crate) struct Command {
    command_input: String,
    command_display: String,
}

impl<R: Registry, B: Backend> Module<R, B> for Command {}

impl<R: Registry> HandleInput<R> for Command {
    fn try_activate(&mut self, input: Key, _registry: &R, _table: &mut StatefulTable<URLItem>) -> Result<Option<InputMode>, Box<dyn Error>> {
        if input != Key::Char(':') {
            return Ok(None)
        }
        self.command_display = ":".to_string();

        return Ok(Some(InputMode::Command))
    }

    fn handle_input(&mut self, input: Key, _registry: &R, table: &mut StatefulTable<URLItem>) -> Result<Option<InputMode>, Box<dyn std::error::Error>> {
        match input {
            Key::Esc => {
                // TODO: discard command
                self.command_input = "".to_string();
                // table.unselect();
                return Ok(Some(InputMode::Normal))
            }
            Key::Char('\n') => {
                // TODO: run command
            }
            Key::Char(c) => {
                // TODO: as function on mut self
                self.command_input.push(c);
                self.command_display.push(c);
            }
            Key::Backspace => {
                self.command_input.pop();
                self.command_display.pop();
            }
            _ => {}
        }

        Ok(None)
    }

}

impl<B: Backend> Draw<B> for Command {
    fn draw(&self, mode: InputMode, f: &mut Frame<B>) {
        return match mode {
            InputMode::Command => {
                self.render_command_input(f);
                // Make the cursor visible and ask tui-rs to put it at the specified coordinates after rendering
                f.set_cursor(
                    // Put cursor past the end of the input text
                    self.command_display.len() as u16 + 1, // TODO: consider using crate UnicodeWidth
                    // Move two line up from the bottom - search input
                    f.size().height - 5,
                )
            }
            _ => {
                // if search phrase is not empty - keep displaying search box
                if self.command_input != "" {
                    self.render_command_input(f);
                }
            }
        }
    }
}

impl Command {
    pub fn new() -> Command {
        Command{ command_input: "".to_string(), command_display: "".to_string()}
    }

    // fn input_push(&mut self, )

    pub fn render_command_input<B: tui::backend::Backend>(&self, f: &mut Frame<B>) {
        let input_widget = Paragraph::new(self.command_display.as_ref())
            .style(
                Style::default()
            )
            .block(
                Block::default()
                    .borders(Borders::TOP)
            );

        let block = self.centered_command_input(f.size());

        f.render_widget(Clear, block);
        f.render_widget(input_widget, block);  // TODO: render stateful widget?
    }

    // TODO: Remove duplication
    fn centered_command_input(&self, r: Rect) -> Rect {
        let command_input = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Length(r.height - 6),
                    Constraint::Length(2),
                    Constraint::Length(r.height - 4),
                ]
                    .as_ref(),
            )
            .split(r);

        Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Length(1),
                    Constraint::Length(r.width-2),
                    Constraint::Length(r.width-1),
                ]
                    .as_ref(),
            )
            .split(command_input[1])[1]
    }
}

#[cfg(test)]
mod test {
    // use termion::event::Key;
    // use crate::interactive::modules::search::Command;
    // use crate::interactive::modules::HandleInput;
    // use bookmark_lib::registry::URLRegistry;
    // use crate::interactive::table::StatefulTable;
    // use crate::interactive::url_table_item::URLItem;
    //
    // #[test]
    // fn test_handle_input_search_phrase() {
    //     let mut search_module = Search::new();
    //     let (dummy_registry, _) = URLRegistry::with_temp_file("search_test1.json")
    //         .expect("Failed to initialize Registry");
    //     let mut dummy_table = StatefulTable::<URLItem>::with_items(&vec![]);
    //
    //
    //     println!("Should input search phrase...");
    //     let key_events = vec![
    //         Key::Char('t'),
    //         Key::Char('e'),
    //         Key::Char('s'),
    //         Key::Char('t'),
    //         Key::Char(' '),
    //         Key::Char('1'),
    //     ];
    //
    //     for key in key_events {
    //         let mode = search_module
    //             .handle_input(key, &dummy_registry, &mut dummy_table)
    //             .expect("Failed to handle event");
    //         assert!(mode == None);
    //     }
    //     assert_eq!("test 1".to_string(), search_module.search_phrase);
    //
    //     let key_events = vec![
    //         Key::Backspace,
    //         Key::Backspace,
    //         Key::Char('-'),
    //         Key::Char('2'),
    //     ];
    //
    //     for key in key_events {
    //         let mode = search_module
    //             .handle_input(key, &dummy_registry, &mut dummy_table)
    //             .expect("Failed to handle event");
    //         assert!(mode == None);
    //     }
    //     assert_eq!("test-2".to_string(), search_module.search_phrase);
    // }

}

