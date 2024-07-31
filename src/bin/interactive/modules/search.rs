use crate::interactive::bookmarks_table::BookmarksTable;
use crate::interactive::helpers::{horizontal_layout, vertical_layout};
use crate::interactive::interface::InputMode;
use crate::interactive::modules::{Draw, HandleInput, Module};
use ratatui::backend::Backend;
use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::text::Text;
use ratatui::widgets::{Block, Borders, Clear, Paragraph};
use ratatui::Frame;
use std::error::Error;
use termion::event::Key;

pub(crate) struct Search {
    search_phrase: String,
}

impl Module for Search {}

impl HandleInput for Search {
    fn try_activate(
        &mut self,
        input: Key,
        table: &mut BookmarksTable,
    ) -> Result<Option<InputMode>, Box<dyn Error>> {
        if input != Key::Char('/') && input != Key::Ctrl('f') {
            return Ok(None);
        }

        table.unselect();
        Ok(Some(InputMode::Search))
    }

    fn handle_input(
        &mut self,
        input: Key,
        table: &mut BookmarksTable,
    ) -> Result<Option<InputMode>, Box<dyn std::error::Error>> {
        match input {
            Key::Esc | Key::Up | Key::Down | Key::Char('\n') => {
                table.unselect();
                return Ok(Some(InputMode::Normal));
            }
            Key::Char(c) => {
                self.search_phrase.push(c);
            }
            Key::Backspace => {
                self.search_phrase.pop();
            }
            _ => {}
        }

        table.search(&self.search_phrase)?;
        Ok(None)
    }
}

impl Draw for Search {
    fn draw(&self, mode: InputMode, f: &mut Frame) {
        match mode {
            InputMode::Search => {
                self.render_search_input(f);
                // Make the cursor visible and ask ratatui-rs to put it at the specified coordinates after rendering
                f.set_cursor(
                    // Put cursor past the end of the input text
                    self.search_phrase.len() as u16 + 1, // TODO: consider using crate UnicodeWidth
                    // Move two line up from the bottom - search input
                    f.size().height - 2,
                )
            }
            _ => {
                // if search phrase is not empty - keep displaying search box
                if self.search_phrase != "" {
                    self.render_search_input(f);
                }
            }
        }
    }
}

impl Search {
    pub fn new() -> Search {
        Search {
            search_phrase: "".to_string(),
        }
    }

    pub fn render_search_input(&self, f: &mut Frame) {
        let input_widget = Paragraph::new(Text::raw(&self.search_phrase))
            .style(Style::default())
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Press '/' or 'CTRL + f' to search for URLs"),
            );
        let r = f.size();
        let search_block = Rect::new(0, r.height - 3, r.width, 3);

        f.render_widget(Clear, search_block);
        f.render_widget(input_widget, search_block); // TODO: render stateful widget?
    }
}

#[cfg(test)]
mod test {
    use crate::interactive::bookmarks_table::BookmarksTable;
    use crate::interactive::event::Events;
    use crate::interactive::helpers::to_keys;
    use crate::interactive::modules::search::Search;
    use crate::interactive::modules::HandleInput;
    use bookmark_lib::registry::URLRegistry;
    use termion::event::Key;

    #[test]
    fn test_handle_input_search_phrase() {
        let mut search_module = Search::new();
        let (dummy_registry, _) = URLRegistry::with_temp_file("search_test1.json")
            .expect("Failed to initialize Registry");
        let events = Events::new();

        let mut bookmarks_table = BookmarksTable::new(events.tx.clone(), Box::new(dummy_registry))
            .expect("Failed to initialized Bookmarks table");

        println!("Should input search phrase...");
        let key_events = to_keys("test 1");

        for key in key_events {
            let mode = search_module
                .handle_input(key, &mut bookmarks_table)
                .expect("Failed to handle event");
            assert!(mode == None);
        }
        assert_eq!("test 1".to_string(), search_module.search_phrase);

        let key_events = vec![
            Key::Backspace,
            Key::Backspace,
            Key::Char('-'),
            Key::Char('2'),
        ];

        for key in key_events {
            let mode = search_module
                .handle_input(key, &mut bookmarks_table)
                .expect("Failed to handle event");
            assert!(mode == None);
        }
        assert_eq!("test-2".to_string(), search_module.search_phrase);
    }
}
