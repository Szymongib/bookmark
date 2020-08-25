use termion::event::Key;
use tui::backend::Backend;
use tui::Frame;
use crate::interactive::table::StatefulTable;
use crate::interactive::url_table_item::{URLItem};
use crate::interactive::interface::InputMode;
use bookmark_lib::Registry;
use bookmark_lib::filters::FilterSet;
use tui::widgets::{Paragraph, Block, Borders, Clear};
use tui::style::{Style, Color};
use tui::layout::{Rect, Layout, Direction, Constraint};
use crate::interactive::modules::{HandleInput, Draw, Module};
use std::error::Error;
use std::collections::HashMap;
use crate::interactive::helpers;
use crate::interactive::bookmarks_table::BookmarksTable;


pub(crate) struct Command {
    command_input: String,
    command_display: String,
    result_display: Option<String>,
}

impl<B: Backend> Module<B> for Command {}

// TODO: display error message in different line?
impl HandleInput for Command {
    fn try_activate(&mut self, input: Key, table: &mut BookmarksTable) -> Result<Option<InputMode>, Box<dyn Error>> {
        if input != Key::Char(':') {
            return Ok(None)
        }
        self.command_display = ":".to_string();

        return Ok(Some(InputMode::Command))
    }

    fn handle_input(&mut self, input: Key, table: &mut BookmarksTable) -> Result<Option<InputMode>, Box<dyn std::error::Error>> {
        match input {
            Key::Esc => {
                self.command_input = "".to_string();
                return Ok(Some(InputMode::Normal))
            }
            Key::Char('\n') => {
                if self.command_input == "" {
                    return Ok(None)
                }

                let action_index = self.command_input.find(' ').unwrap_or(self.command_input.len());

                let action = &self.command_input.as_str()[0..action_index];
                let args: Vec<&str> = (self.command_input.as_str())[action_index..].split(' ')
                    .filter(|s| { *s != "" })
                    .collect();

                return match table.exec(action, args) { // TODO: here I want error, command error and msg
                    Ok(msg) => {
                        self.command_input = "".to_string();
                        Ok(Some(InputMode::Normal))
                    },
                    Err(err) => {
                        self.command_display = err.to_string();
                        Ok(None)
                    }
                }
            }
            Key::Char(c) => {
                self.input_push(c);
            }
            Key::Backspace => {
                self.input_pop()
            }
            _ => {
                self.update_display()
            }
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

        // let tag_action: Box<dyn Execute<R>> = Box::new(TagAction::new());

        Command{
            command_input: "".to_string(),
            command_display: "".to_string(),
            result_display: None,
            // actions: hashmap![
            //     "tag".to_string() => tag_action
            // ],
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

    fn update_display(&mut self) {
        self.command_display = format!(":{}", self.command_input)
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

