use termion::event::Key;
use tui::backend::Backend;
use tui::Frame;
use crate::interactive::table::StatefulTable;
use crate::interactive::url_table_item::{URLItem, URLItemSource};
use crate::interactive::interface::InputMode;
use bookmark_lib::Registry;
use bookmark_lib::record_filter::FilterSet;
use tui::widgets::{Paragraph, Block, Borders, Clear};
use tui::style::{Style, Color};
use tui::layout::{Rect, Layout, Direction, Constraint};
use crate::interactive::modules::{HandleInput, Draw, Module};
use std::error::Error;
use std::collections::HashMap;
use crate::interactive::command::Execute;
use crate::interactive::helpers;
use crate::interactive::command::tag::{TagAction};


pub(crate) struct Command<R: Registry> {
    command_input: String,
    command_display: String,

    actions: HashMap<String, Box<dyn Execute<R>>>
}

impl<R: Registry, B: Backend> Module<R, B> for Command<R> {}

impl<R: Registry> HandleInput<R> for Command<R> {
    fn try_activate(&mut self, input: Key, _registry: &R, _table: &mut StatefulTable<URLItemSource<R>, URLItem>) -> Result<Option<InputMode>, Box<dyn Error>> {
        if input != Key::Char(':') {
            return Ok(None)
        }
        self.command_display = ":".to_string();

        return Ok(Some(InputMode::Command))
    }

    fn handle_input(&mut self, input: Key, registry: &R, table: &mut StatefulTable<URLItemSource<R>, URLItem>) -> Result<Option<InputMode>, Box<dyn std::error::Error>> {
        match input {
            Key::Esc => {
                // TODO: discard command
                self.command_input = "".to_string();
                // table.unselect();
                return Ok(Some(InputMode::Normal))
            }
            Key::Char('\n') => {
                // TODO: run command
                if self.command_input == "" {
                    return Ok(None)
                }

                let action_index = self.command_input.find(' ').unwrap_or(self.command_input.len());

                let action = &self.command_input.as_str()[0..action_index];
                let args: Vec<&str> = (self.command_input.as_str())[action_index..].split(' ')
                    .filter(|s| { *s != "" })
                    .collect();

                let action = self.actions.get_mut(action);
                if action.is_none() {
                    self.command_display = "ERROR".to_string(); // TODO: indicate that error is displayed
                    return Ok(None);
                }

                return match action.unwrap().execute(registry, table, args) {
                    Ok(_ ) => {
                        self.command_display = format!("URL tagged"); // TODO: indicate that info is displayed
                        // TODO: refresh visible tags
                        Ok(None)
                    }
                    Err(cmd_err) => {
                        self.command_display = format!("ERROR: {}", cmd_err.to_string()); // TODO: indicate that error is displayed
                        Ok(None)
                    }
                }
            }
            Key::Char(c) => {
                // TODO: as function on mut self
                self.input_push(c);
            }
            Key::Backspace => {
                self.input_pop()
            }
            _ => {}
        }

        Ok(None)
    }

}

impl<R: Registry, B: Backend> Draw<B> for Command<R> {
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

impl<R: Registry> Command<R> {
    pub fn new() -> Command<R> {

        let tag_action: Box<dyn Execute<R>> = Box::new(TagAction::new());

        Command{
            command_input: "".to_string(),
            command_display: "".to_string(),
            actions: hashmap![
                "tag".to_string() => tag_action
            ],
        }
    }

    fn input_push(&mut self, ch: char) {
        self.command_input.push(ch);
        self.command_display.push(ch);
    }
    fn input_pop(&mut self) {
        self.command_input.pop();
        if self.command_display.len() > 1 {
            self.command_display.pop();
        }
    }

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

