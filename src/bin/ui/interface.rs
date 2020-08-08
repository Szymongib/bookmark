use crate::ui::event::Event;
use crate::ui::table::{StatefulTable, TableItem};
use crate::ui::url_table_item::URLItem;
use bookmark_lib::record_filter::FilterSet;
use bookmark_lib::types::URLRecord;
use std::error::Error;
use termion::event::Key;
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, Paragraph, Row, Table};
use tui::Frame;

#[derive(PartialEq)]
pub enum InputMode {
    Normal,
    Edit(Action),
}

#[derive(PartialEq)]
pub enum Action {
    Search,
}

pub struct Interface {
    /// Current mode of input
    input_mode: InputMode,
    /// Current searched phrase
    search_phrase: String,

    /// Table with URLs
    table: StatefulTable<URLItem>,

    /// Styles used for displaying user interface
    styles: Styles,
}

struct Styles {
    normal: Style,
    selected: Style,
}

impl Interface {
    pub(crate) fn new(urls: Vec<URLRecord>) -> Interface {
        let items: Vec<URLItem> = urls.iter().map(|u| URLItem::new(u.clone())).collect();

        let table = StatefulTable::with_items(items.as_slice());

        Interface {
            input_mode: InputMode::Normal,
            search_phrase: "".to_string(),
            table,
            styles: Styles {
                selected: Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
                normal: Style::default().fg(Color::White),
            },
        }
    }

    /// updates URLs visibility inside the `table` according to the `search_phrase`
    fn apply_search(&mut self) {
        let filter = FilterSet::new_combined_filter(self.search_phrase.as_str());

        for item in &mut self.table.items {
            item.filter(&filter)
        }

        self.table.refresh_visible()
    }

    pub(crate) fn handle_input(&mut self, event: Event<Key>) -> Result<bool, Box<dyn Error>> {
        if let Event::Input(input) = event {
            match self.input_mode {
                InputMode::Normal => match input {
                    Key::Char('q') => {
                        return Ok(true);
                    }
                    Key::Left => {
                        self.table.unselect();
                    }
                    Key::Down => {
                        self.table.next();
                    }
                    Key::Up => {
                        self.table.previous();
                    }
                    Key::Char('/') | Key::Ctrl('f') => {
                        self.table.unselect();
                        self.input_mode = InputMode::Edit(Action::Search)
                    }
                    Key::Char('\n') => {
                        let selected_id = self.table.state.selected();
                        if selected_id.is_none() {
                            return Ok(false);
                        }
                        let selected_id = selected_id.unwrap();

                        let item = &self.table.visible[selected_id];

                        let res = open::that(item.url().as_str());
                        if let Err(err) = res {
                            return Err(From::from(format!(
                                "failed to open URL in the browser: {}",
                                err.to_string()
                            )));
                        }
                    }
                    _ => {}
                },
                InputMode::Edit(_) => match input {
                    Key::Esc | Key::Up | Key::Down | Key::Char('\n') => {
                        self.input_mode = InputMode::Normal;
                        self.table.unselect();
                    }
                    Key::Char(c) => {
                        self.search_phrase.push(c);
                        self.apply_search();
                    }
                    Key::Backspace => {
                        self.search_phrase.pop();
                        self.apply_search();
                    }
                    _ => {}
                },
            }
        }

        Ok(false)
    }

    pub(crate) fn draw<B: tui::backend::Backend>(&mut self, f: &mut Frame<B>) {
        let size = f.size();
        let normal_style = self.styles.normal.clone();

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Length(size.height - 3), // URLs display table
                    Constraint::Min(1),                  // Search input
                ]
                .as_ref(),
            )
            .split(f.size());

        let header = ["  Name", "URL", "Group", "Tags"];
        let rows = self.table.visible.iter().map(|i| {
            if i.visible() {
                Row::StyledData(i.row().iter(), normal_style)
            } else {
                Row::Data(i.row().iter())
            }
        });
        let t = Table::new(header.iter(), rows)
            .block(Block::default().borders(Borders::ALL).title("URLs"))
            .highlight_style(self.styles.selected)
            .highlight_symbol("> ")
            .widths(&[
                Constraint::Percentage(20),
                Constraint::Percentage(40),
                Constraint::Percentage(15),
                Constraint::Percentage(15),
            ]);

        f.render_stateful_widget(t, chunks[0], &mut self.table.state);

        let input_widget = Paragraph::new(self.search_phrase.as_ref())
            .style(Style::default())
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Press '/' or 'CTRL + f' to search for URLs"),
            );

        f.render_widget(input_widget, chunks[1]);

        match self.input_mode {
            InputMode::Normal =>
                // Hide the cursor. `Frame` does this by default, so we don't need to do anything here
                {}
            InputMode::Edit(_) => {
                // Make the cursor visible and ask tui-rs to put it at the specified coordinates after rendering
                f.set_cursor(
                    // Put cursor past the end of the input text
                    chunks[1].x + self.search_phrase.len() as u16 + 1, // TODO: consider using crate UnicodeWidth
                    // Move one line down, from the border to the input line
                    chunks[1].y + 1,
                )
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::ui::event::Event;
    use crate::ui::interface::{Action, InputMode, Interface};
    use bookmark_lib::types::URLRecord;
    use termion::event::Key;

    fn fix_url_records() -> Vec<URLRecord> {
        vec![
            URLRecord::new("one", "one", "one", vec!["tag"]),
            URLRecord::new("two", "two", "two", vec![]),
            URLRecord::new("three", "three", "three", vec![]),
            URLRecord::new("four", "four", "four", vec!["tag"]),
            URLRecord::new("five", "five", "five", vec![]),
        ]
    }

    #[test]
    fn test_handle_input_returns() {
        let mut interface = Interface::new(fix_url_records());

        // Should quit when input 'q'
        let event = Event::Input(Key::Char('q'));
        let quit = interface
            .handle_input(event)
            .expect("Failed to handle event");
        assert!(quit);

        // Should pass if key not handled
        let event = Event::Input(Key::Char('j'));
        let quit = interface
            .handle_input(event)
            .expect("Failed to handle event");
        assert!(!quit);

        // Should do nothing on enter, when no URL selected
        let event = Event::Input(Key::Char('\n'));
        let quit = interface
            .handle_input(event)
            .expect("Failed to handle event");
        assert!(!quit);
    }

    #[test]
    fn test_handle_input_input_modes() {
        let mut interface = Interface::new(fix_url_records());

        assert!(InputMode::Normal == interface.input_mode);

        println!("Should switch input modes...");
        let event = Event::Input(Key::Char('/'));
        let quit = interface
            .handle_input(event)
            .expect("Failed to handle event");
        assert!(!quit);
        assert!(InputMode::Edit(Action::Search) == interface.input_mode);

        let event = Event::Input(Key::Esc);
        let quit = interface
            .handle_input(event)
            .expect("Failed to handle event");
        assert!(!quit);
        assert!(InputMode::Normal == interface.input_mode);

        let event = Event::Input(Key::Ctrl('f'));
        let quit = interface
            .handle_input(event)
            .expect("Failed to handle event");
        assert!(!quit);
        assert!(InputMode::Edit(Action::Search) == interface.input_mode);

        let event = Event::Input(Key::Esc);
        let quit = interface
            .handle_input(event)
            .expect("Failed to handle event");
        assert!(!quit);
        assert!(InputMode::Normal == interface.input_mode);

        println!("Should go to normal mode...");
        let go_to_normal_events = vec![
            Event::Input(Key::Up),
            Event::Input(Key::Down),
            Event::Input(Key::Esc),
            Event::Input(Key::Char('\n')),
        ];

        for event in go_to_normal_events {
            interface.input_mode = InputMode::Edit(Action::Search);
            let quit = interface
                .handle_input(event)
                .expect("Failed to handle event");
            assert!(!quit);
            assert!(InputMode::Normal == interface.input_mode);
        }

        println!("Should go to edit mode...");
        let go_to_edit_events = vec![Event::Input(Key::Char('/')), Event::Input(Key::Ctrl('f'))];

        for event in go_to_edit_events {
            interface.input_mode = InputMode::Normal;
            let quit = interface
                .handle_input(event)
                .expect("Failed to handle event");
            assert!(!quit);
            assert!(InputMode::Edit(Action::Search) == interface.input_mode);
        }
    }

    #[test]
    fn test_handle_input_search_phrase() {
        let mut interface = Interface::new(fix_url_records());

        println!("Should input search phrase...");
        let event = Event::Input(Key::Char('/'));
        let quit = interface
            .handle_input(event)
            .expect("Failed to handle event");
        assert!(!quit);

        let events = vec![
            Event::Input(Key::Char('t')),
            Event::Input(Key::Char('e')),
            Event::Input(Key::Char('s')),
            Event::Input(Key::Char('t')),
            Event::Input(Key::Char(' ')),
            Event::Input(Key::Char('1')),
        ];

        for event in events {
            let quit = interface
                .handle_input(event)
                .expect("Failed to handle event");
            assert!(!quit);
        }
        assert_eq!("test 1".to_string(), interface.search_phrase);

        let events = vec![
            Event::Input(Key::Backspace),
            Event::Input(Key::Backspace),
            Event::Input(Key::Char('-')),
            Event::Input(Key::Char('2')),
        ];

        for event in events {
            let quit = interface
                .handle_input(event)
                .expect("Failed to handle event");
            assert!(!quit);
        }
        assert_eq!("test-2".to_string(), interface.search_phrase);

        println!("Should preserve search phrase when going to normal mode...");
        let event = Event::Input(Key::Esc);
        let quit = interface
            .handle_input(event)
            .expect("Failed to handle event");
        assert!(!quit);
        assert!(InputMode::Normal == interface.input_mode);

        assert_eq!("test-2".to_string(), interface.search_phrase);

        let event = Event::Input(Key::Char('/'));
        let quit = interface
            .handle_input(event)
            .expect("Failed to handle event");
        assert!(!quit);

        assert_eq!("test-2".to_string(), interface.search_phrase);
    }

    #[test]
    fn test_handle_input_search() {
        let mut interface = Interface::new(fix_url_records());

        println!("Should filter items in table on input...");
        let event = Event::Input(Key::Char('/'));
        let quit = interface
            .handle_input(event)
            .expect("Failed to handle event");
        assert!(!quit);

        for event in vec![
            Event::Input(Key::Char('t')),
            Event::Input(Key::Char('a')),
            Event::Input(Key::Char('g')),
        ] {
            let quit = interface
                .handle_input(event)
                .expect("Failed to handle event");
            assert!(!quit);
        }
        assert_eq!(2, interface.table.visible.len()); // URLs with tag 'tag'

        println!("Should filter items in table on backspace...");
        for event in vec![Event::Input(Key::Backspace), Event::Input(Key::Backspace)] {
            let quit = interface
                .handle_input(event)
                .expect("Failed to handle event");
            assert!(!quit);
        }
        assert_eq!(4, interface.table.visible.len()); // URLs with letter 't'
    }

    struct TestCase {
        description: String,
        events: Vec<Event<Key>>,
        selected: Vec<Option<usize>>,
    }

    #[test]
    fn test_handle_input_selections() {
        let test_cases = vec![
            TestCase {
                description: "multiple ups and downs".to_string(),
                events: vec![
                    Event::Input(Key::Down),
                    Event::Input(Key::Down),
                    Event::Input(Key::Down),
                    Event::Input(Key::Down),
                    Event::Input(Key::Down),
                    Event::Input(Key::Down),
                    Event::Input(Key::Down),
                    Event::Input(Key::Up),
                    Event::Input(Key::Up),
                ],
                selected: vec![
                    Some(0),
                    Some(1),
                    Some(2),
                    Some(3),
                    Some(4),
                    Some(0),
                    Some(1),
                    Some(0),
                    Some(4),
                ],
            },
            TestCase {
                description: "unselect".to_string(),
                events: vec![
                    Event::Input(Key::Down),
                    Event::Input(Key::Down),
                    Event::Input(Key::Left),
                ],
                selected: vec![Some(0), Some(1), None],
            },
            TestCase {
                description: "unselect with search".to_string(),
                events: vec![
                    Event::Input(Key::Down),
                    Event::Input(Key::Down),
                    Event::Input(Key::Ctrl('f')),
                ],
                selected: vec![Some(0), Some(1), None],
            },
            TestCase {
                description: "unselect with search and stop search with up".to_string(),
                events: vec![
                    Event::Input(Key::Down),
                    Event::Input(Key::Down),
                    Event::Input(Key::Ctrl('f')),
                    Event::Input(Key::Up),
                    Event::Input(Key::Up),
                ],
                selected: vec![Some(0), Some(1), None, None, Some(0)],
            },
            TestCase {
                description: "unselect with search and stop search with down".to_string(),
                events: vec![
                    Event::Input(Key::Down),
                    Event::Input(Key::Down),
                    Event::Input(Key::Char('/')),
                    Event::Input(Key::Down),
                    Event::Input(Key::Down),
                ],
                selected: vec![Some(0), Some(1), None, None, Some(0)],
            },
            TestCase {
                description: "unselect with search and stop search with esc".to_string(),
                events: vec![
                    Event::Input(Key::Up),
                    Event::Input(Key::Up),
                    Event::Input(Key::Char('/')),
                    Event::Input(Key::Esc),
                    Event::Input(Key::Down),
                ],
                selected: vec![Some(0), Some(4), None, None, Some(0)],
            },
        ];

        for test in &test_cases {
            println!("Running test case: {}", test.description);

            let mut interface = Interface::new(fix_url_records());

            for i in 0..test.events.len() {
                let quit = interface
                    .handle_input(test.events[i].clone())
                    .expect("Failed to handle event");
                assert!(!quit);
                assert_eq!(test.selected[i], interface.table.state.selected())
            }
        }
    }
}
