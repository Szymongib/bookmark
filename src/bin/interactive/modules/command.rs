use crate::interactive::bookmarks_table::BookmarksTable;
use crate::interactive::helpers::{horizontal_layout, vertical_layout};
use crate::interactive::interface::InputMode;
use crate::interactive::modules::{Draw, HandleInput, Module};
use std::error::Error;
use termion::event::Key;
use tui::backend::Backend;
use tui::layout::Rect;
use tui::style::Style;
use tui::widgets::{Block, Borders, Clear, Paragraph};
use tui::Frame;

const DEFAULT_INFO_MESSAGE: &str =
    "Press 'Enter' to execute command on selected Bookmark. Press 'Esc' to discard.";

pub(crate) struct Command {
    info_display: String,
    command_input: String,
    command_display: String,
}

impl<B: Backend> Module<B> for Command {}

impl HandleInput for Command {
    fn try_activate(
        &mut self,
        input: Key,
        _table: &mut BookmarksTable,
    ) -> Result<Option<InputMode>, Box<dyn Error>> {
        if input != Key::Char(':') {
            return Ok(None);
        }

        return Ok(Some(InputMode::Command));
    }

    fn handle_input(
        &mut self,
        input: Key,
        table: &mut BookmarksTable,
    ) -> Result<Option<InputMode>, Box<dyn std::error::Error>> {
        match input {
            Key::Esc => {
                self.reset_input();
                return Ok(Some(InputMode::Normal));
            }
            Key::Char('\n') => {
                if self.command_input == "" {
                    return Ok(None);
                }

                let action_index = self
                    .command_input
                    .find(' ')
                    .unwrap_or(self.command_input.len());

                let action = &self.command_input.as_str()[0..action_index];
                let args: Vec<&str> = (self.command_input.as_str())[action_index..]
                    .split(' ')
                    .filter(|s| *s != "")
                    .collect();

                return match table.exec(action, args) {
                    // TODO: here I want error, command error and msg
                    Ok(_) => {
                        self.reset_input();
                        Ok(Some(InputMode::Normal))
                    }
                    Err(err) => {
                        self.info_display = err.to_string();
                        Ok(None)
                    }
                };
            }
            Key::Char(c) => {
                self.input_push(c);
            }
            Key::Backspace => self.input_pop(),
            _ => self.update_display(),
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
        };
    }
}

impl Command {
    pub fn new() -> Command {
        Command {
            info_display: DEFAULT_INFO_MESSAGE.to_string(),
            command_input: "".to_string(),
            command_display: ":".to_string(),
        }
    }

    fn input_push(&mut self, ch: char) {
        self.command_input.push(ch);
        self.update_display();
    }
    fn input_pop(&mut self) {
        self.command_input.pop();
        self.update_display();
    }

    fn update_display(&mut self) {
        self.command_display = format!(":{}", self.command_input)
    }

    fn reset_input(&mut self) {
        self.info_display = DEFAULT_INFO_MESSAGE.to_string();
        self.command_input = "".to_string();
        self.update_display();
    }

    pub fn render_command_input<B: tui::backend::Backend>(&self, f: &mut Frame<B>) {
        let info_widget = Paragraph::new(self.info_display.as_ref())
            .style(Style::default())
            .block(Block::default().borders(Borders::TOP));

        let input_widget = Paragraph::new(self.command_display.as_ref())
            .style(Style::default())
            .block(Block::default().borders(Borders::BOTTOM));

        let (info_block, input_block) = self.centered_command_input(f.size());

        f.render_widget(Clear, info_block);
        f.render_widget(info_widget, info_block); // TODO: render stateful widget?
        f.render_widget(Clear, input_block);
        f.render_widget(input_widget, input_block); // TODO: render stateful widget?
    }

    fn centered_command_input(&self, r: Rect) -> (Rect, Rect) {
        let horizontal_layout = horizontal_layout(vec![1, r.width - 2, r.width - 1]);

        let split_info = vertical_layout(vec![r.height - 7, 2, r.height - 5]).split(r);
        let info = horizontal_layout.clone().split(split_info[1])[1];

        let split_input = vertical_layout(vec![r.height - 5, 2, r.height - 3]).split(r);
        let input = horizontal_layout.split(split_input[1])[1];

        (info, input)
    }
}

#[cfg(test)]
mod test {
    use crate::interactive::bookmarks_table::BookmarksTable;
    use crate::interactive::event::Events;
    use crate::interactive::helpers::to_keys;
    use crate::interactive::interface::InputMode;
    use crate::interactive::modules::command::{Command, DEFAULT_INFO_MESSAGE};
    use crate::interactive::modules::HandleInput;
    use bookmark_lib::registry::URLRegistry;
    use bookmark_lib::Registry;
    use termion::event::Key;

    #[test]
    fn test_exec_command() {
        let mut command_module = Command::new();
        let (dummy_registry, _) = URLRegistry::with_temp_file("command_test1.json")
            .expect("Failed to initialize Registry");
        dummy_registry
            .new("abcd", "url", None, vec![])
            .expect("Failed to create Bookmark");
        let events = Events::new();

        let mut bookmarks_table = BookmarksTable::new(events.tx.clone(), Box::new(dummy_registry))
            .expect("Failed to initialized Bookmarks table");

        println!("Should input command phrase...");
        let key_events = to_keys("tag test");

        for key in key_events {
            let mode = command_module
                .handle_input(key, &mut bookmarks_table)
                .expect("Failed to handle event");
            assert!(mode == None);
        }

        println!("Should execute 'tag' command...");
        bookmarks_table.table().state.select(Some(0));
        let mode = command_module
            .handle_input(Key::Char('\n'), &mut bookmarks_table)
            .expect("Failed to handle event");
        assert!(mode == Some(InputMode::Normal));
        assert_eq!(command_module.info_display, DEFAULT_INFO_MESSAGE);
        assert_eq!(command_module.command_input, "");
        assert_eq!(command_module.command_display, ":");
    }

    #[test]
    fn test_exec_display_error_message_when_cmd_failed() {
        let mut command_module = Command::new();
        let (dummy_registry, _) = URLRegistry::with_temp_file("command_test2.json")
            .expect("Failed to initialize Registry");
        let events = Events::new();

        let mut bookmarks_table = BookmarksTable::new(events.tx.clone(), Box::new(dummy_registry))
            .expect("Failed to initialized Bookmarks table");

        println!("Should input command phrase...");
        let key_events = to_keys("tag test");

        for key in key_events {
            let mode = command_module
                .handle_input(key, &mut bookmarks_table)
                .expect("Failed to handle event");
            assert!(mode == None);
        }

        println!("Should fail to execute 'tag' command when no item selected...");
        let mode = command_module
            .handle_input(Key::Char('\n'), &mut bookmarks_table)
            .expect("Failed to handle event");
        assert!(mode == None);
        assert_eq!(command_module.info_display, "error: item not selected");
        assert_eq!(command_module.command_input, "tag test");
        assert_eq!(command_module.command_display, ":tag test");
    }

    #[test]
    fn test_do_nothing_when_input_empty() {
        let mut command_module = Command::new();
        let (dummy_registry, _) = URLRegistry::with_temp_file("command_test2.json")
            .expect("Failed to initialize Registry");
        let events = Events::new();

        let mut bookmarks_table = BookmarksTable::new(events.tx.clone(), Box::new(dummy_registry))
            .expect("Failed to initialized Bookmarks table");

        println!("Should do nothing when input is empty...");
        let mode = command_module
            .handle_input(Key::Char('\n'), &mut bookmarks_table)
            .expect("Failed to handle event");
        assert!(mode == None);
        assert_eq!(command_module.info_display, DEFAULT_INFO_MESSAGE);
        assert_eq!(command_module.command_input, "");
        assert_eq!(command_module.command_display, ":");
    }

    #[test]
    fn test_handle_input_write_command() {
        let mut command_module = Command::new();
        let (dummy_registry, _) = URLRegistry::with_temp_file("command_test3.json")
            .expect("Failed to initialize Registry");
        let events = Events::new();

        let mut bookmarks_table = BookmarksTable::new(events.tx.clone(), Box::new(dummy_registry))
            .expect("Failed to initialized Bookmarks table");

        println!("Should input command phrase...");
        let key_events = to_keys("tag test");

        for key in key_events {
            let mode = command_module
                .handle_input(key, &mut bookmarks_table)
                .expect("Failed to handle event");
            assert!(mode == None);
        }
        assert_eq!("tag test".to_string(), command_module.command_input);

        let key_events = vec![
            Key::Backspace,
            Key::Backspace,
            Key::Char('m'),
            Key::Char('p'),
        ];

        for key in key_events {
            let mode = command_module
                .handle_input(key, &mut bookmarks_table)
                .expect("Failed to handle event");
            assert!(mode == None);
        }
        assert_eq!("tag temp".to_string(), command_module.command_input);
    }
}
