use crate::interactive::event::Event;
use crate::interactive::table::{StatefulTable, TableItem};
use crate::interactive::url_table_item::URLItem;
use std::error::Error;
use termion::event::Key;
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span};
use tui::widgets::{Block, Borders, Row, Table};
use tui::Frame;
use bookmark_lib::Registry;
use std::collections::HashMap;
use crate::interactive::modules::{Module};
use crate::interactive::modules::search::Search;
use crate::interactive::modules::help::HelpPanel;
use crate::interactive::modules::delete::Delete;
use crate::interactive::modules::command::Command;

// TODO: some decisions
// - drop Add functionality from interactive mode for now
// - :q quit
// - ':' to start action
// - ':et' - edit tag of selected
// - ':eg' - edit group of selected
// - ':g [GROUP]' - filter by group
// - ':t [TAG]' - filter by tag
// - '????" - remove filters

// InputMode:
// - Normal
// - Search - show search bar
// - Command - show command bar
//   - Action
// - Suppressed


// TODO: consider moving to some lib
macro_rules! hashmap {
    ($( $key: expr => $val: expr ),*) => {{
         let mut map = ::std::collections::HashMap::new();
         $( map.insert($key, $val); )*
         map
    }}
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub enum InputMode {
    Normal,
    Search,
    Command,
    Suppressed(SuppressedAction),
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub enum SuppressedAction {
    ShowHelp,
    Delete,
}

pub struct Interface<R: Registry, B: tui::backend::Backend> {
    registry: R,

    /// Interface modules
    modules: HashMap<InputMode, Box<dyn Module<R, B>>>,

    /// Current mode of input
    input_mode: InputMode,

    /// Current command input
    command_input: String,

    /// Table with URLs
    table: StatefulTable<URLItem>,

    /// Styles used for displaying user interface
    styles: Styles,
}

struct Styles {
    normal: Style,
    selected: Style,
    header: Style,
}

impl<R: Registry, B: tui::backend::Backend> Interface<R, B> {
    pub(crate) fn new(registry: R) -> Result<Interface<R, B>, Box<dyn std::error::Error>> {
        let items: Vec<URLItem> = URLItem::from_vec(registry.list_urls(None, None)?);

        let table = StatefulTable::with_items(items.as_slice());

        let search_mod: Box<dyn Module<R,B>> = Box::new(Search::new());
        let help_mod: Box<dyn Module<R,B>> = Box::new(HelpPanel::new());
        let delete_mod: Box<dyn Module<R,B>> = Box::new(Delete::new());
        let command_mod: Box<dyn Module<R,B>> = Box::new(Command::new());

        Ok(Interface {
            registry,
            input_mode: InputMode::Normal,
            command_input: "".to_string(),

            modules: hashmap![
                InputMode::Search => search_mod,
                InputMode::Suppressed(SuppressedAction::ShowHelp) => help_mod,
                InputMode::Suppressed(SuppressedAction::Delete) => delete_mod,
                InputMode::Command => command_mod
            ],

            table,
            styles: Styles {
                selected: Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
                normal: Style::default().fg(Color::White),
                header: Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            },
        })
    }

    pub(crate) fn handle_input(&mut self, event: Event<Key>) -> Result<bool, Box<dyn Error>> {
        if let Event::Input(input) = event {
            match &self.input_mode {
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
                    // Activate first module that can handle the key - if none just skip
                    _ => {
                        for m in self.modules.values_mut() {
                            if let Some(mode) = m.try_activate(input, &self.registry, &mut self.table)? {
                                self.input_mode = mode;
                                return Ok(false)
                            }
                        }
                    }
                },
                // InputMode::Command => match input {
                //     Key::Char('\n') => {
                //         // TODO: run the command
                //     }
                //     Key::Char(c) => {
                //         self.command_input.push(c);
                //     }
                //     Key::Backspace => {
                //         self.command_input.pop();
                //     }
                //     Key::Esc => {
                //         // TODO: discard command
                //         self.input_mode = InputMode::Normal;
                //     }
                //     _ => {}
                // }
                _ => {
                    if let Some(module) = self.modules.get_mut(&self.input_mode) {
                        if let Some(new_mode) = module.handle_input(input, &self.registry, &mut self.table)? {
                            self.input_mode = new_mode;
                        }
                    }
                }
            }
        }

        Ok(false)
    }

    pub(crate) fn draw(&mut self, f: &mut Frame<B>) {
        let size = f.size();
        let normal_style = self.styles.normal.clone();

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    // TODO: consider modules influencing dynamicly the main layout - maybe pass layout to draw?
                    Constraint::Length(size.height - 3), // URLs display table
                    Constraint::Length(3),               // Search input
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
            .header_style(self.styles.header)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("URLs - Press 'h' to show help"),
            )
            .highlight_style(self.styles.selected)
            .highlight_symbol("> ")
            .widths(&[
                Constraint::Percentage(20),
                Constraint::Percentage(40),
                Constraint::Percentage(15),
                Constraint::Percentage(15),
            ]);

        f.render_stateful_widget(t, chunks[0], &mut self.table.state);

        // draw modules
        for module in self.modules.values() {
            module.draw(self.input_mode.clone(), f)
        }
    }

    // fn add_record_popup<B: tui::backend::Backend>(&self, f: &mut Frame<B>) {
    //
    //     let area = centered_rect(60, 40, f.size());
    //     let name = Paragraph::new("")
    //         .style(Style::default().bg(Color::Black).fg(Color::White))
    //         // .block(self.create_block("Confirm (Enter)   ---   Discard (ESC)".to_string()))
    //         .alignment(Alignment::Left);
    //     let url = Paragraph::new("")
    //         .style(Style::default().bg(Color::Black).fg(Color::White))
    //         // .block(self.create_block("Confirm (Enter)   ---   Discard (ESC)".to_string()))
    //         .alignment(Alignment::Left);
    //     let group = Paragraph::new("")
    //         .style(Style::default().bg(Color::Black).fg(Color::White))
    //         // .block(self.create_block("Confirm (Enter)   ---   Discard (ESC)".to_string()))
    //         .alignment(Alignment::Left);
    //     // let group = Paragraph::new("")
    //     //     .style(Style::default().bg(Color::Black).fg(Color::White))
    //     //     // .block(self.create_block("Confirm (Enter)   ---   Discard (ESC)".to_string()))
    //     //     .alignment(Alignment::Left);
    //
    //     f.render_widget(Clear, area);
    //     f.render_widget(name, area);
    // }

    // fn add_tag_popup<B: tui::backend::Backend>(&self, f: &mut Frame<B>) {
    //     let text = vec![
    //         Spans::from("'ENTER'            - open bookmarked URL"),
    //         Spans::from("'/' or 'CTRL + F'  - search for URLs"),
    //     ];
    //
    //     let area = centered_rect(60, 40, f.size());
    //     let paragraph = Paragraph::new(text)
    //         .style(Style::default().bg(Color::Black).fg(Color::White))
    //         .block(self.create_block("Help - press ESC to close".to_string()))
    //         .alignment(Alignment::Left);
    //
    //     f.render_widget(Clear, area);
    //     f.render_widget(paragraph, area);
    // }

    // TODO: move this function to widgets?
    fn create_block(&self, title: String) -> Block {
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().bg(Color::Black).fg(Color::LightBlue))
            .title(Span::styled(
                title,
                Style::default().add_modifier(Modifier::BOLD),
            ))
    }
}

#[cfg(test)]
mod test {
    use crate::interactive::event::Event;
    use crate::interactive::interface::{InputMode, Interface, SuppressedAction};
    use crate::interactive::fake::{MockBackend};
    use bookmark_lib::types::URLRecord;
    use termion::event::Key;
    use bookmark_lib::registry::URLRegistry;
    use std::path::PathBuf;
    use bookmark_lib::Registry;
    use std::{fs};
    use bookmark_lib::storage::FileStorage;
    use rand::{thread_rng, Rng};
    use rand::distributions::Alphanumeric;

    fn fix_url_records() -> Vec<URLRecord> {
        vec![
            URLRecord::new("one", "one", "one", vec!["tag"]),
            URLRecord::new("two", "two", "two", vec![]),
            URLRecord::new("three", "three", "three", vec![]),
            URLRecord::new("four", "four", "four", vec!["tag"]),
            URLRecord::new("five", "five", "five", vec![]),
        ]
    }

    struct Cleaner {
        file_path: PathBuf,
    }

    // TODO: as general trait?
    impl Cleaner {
        fn new(file_path: PathBuf) -> Cleaner {
            Cleaner{ file_path }
        }

        fn clean(&self) {
            fs::remove_file(&self.file_path).expect("Failed to remove file");
        }
    }

    impl Drop for Cleaner {
        fn drop(&mut self) {
            self.clean()
        }
    }

    fn rand_str() -> String {
        let rand_string: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(30)
            .collect();

        rand_string
    }

    macro_rules! init {
    ($($urls:expr), *) => (
        {
            let (registry, file_path) = URLRegistry::with_temp_file(&rand_str()).expect("Failed to initialize registry");
            let _cleaner = Cleaner::new(file_path); // makes sure that temp file is deleted even in case of panic
            $(
                for u in $urls {
                    registry.add_url(u).expect("Failed to add url");
                }
            )*
            let interface = Interface::<URLRegistry<FileStorage>, MockBackend>::new(registry).expect("Failed to initialize interface");

            (interface)
        };
    );
    () => (
        init!(vec![])
    );
    }

    #[test]
    fn test_handle_input_returns() {
        let mut interface = init!();

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
        let mut interface = init!();

        assert!(InputMode::Normal == interface.input_mode);

        println!("Should switch input modes...");
        let event = Event::Input(Key::Char('/'));
        let quit = interface
            .handle_input(event)
            .expect("Failed to handle event");
        assert!(!quit);
        assert!(InputMode::Search == interface.input_mode);

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
        assert!(InputMode::Search == interface.input_mode);

        let event = Event::Input(Key::Esc);
        let quit = interface
            .handle_input(event)
            .expect("Failed to handle event");
        assert!(!quit);
        assert!(InputMode::Normal == interface.input_mode);

        let event = Event::Input(Key::Char('h'));
        let quit = interface
            .handle_input(event)
            .expect("Failed to handle event");
        assert!(!quit);
        assert!(InputMode::Suppressed(SuppressedAction::ShowHelp) == interface.input_mode);

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
            interface.input_mode = InputMode::Search;
            let quit = interface
                .handle_input(event)
                .expect("Failed to handle event");
            assert!(!quit);
            assert!(InputMode::Normal == interface.input_mode);
        }

        println!("Should go to Search mode...");
        let go_to_search_events = vec![Event::Input(Key::Char('/')), Event::Input(Key::Ctrl('f'))];

        for event in go_to_search_events {
            interface.input_mode = InputMode::Normal;
            let quit = interface
                .handle_input(event)
                .expect("Failed to handle event");
            assert!(!quit);
            assert!(InputMode::Search == interface.input_mode);
        }
    }

    #[test]
    fn test_handle_input_switch_input_modes() {
        let mut interface = init!();

        assert!(InputMode::Normal == interface.input_mode);

        println!("Should switch InputModes...");
        let events = vec![
            Event::Input(Key::Char('h')),
            Event::Input(Key::Esc),
            Event::Input(Key::Char('h')),
            Event::Input(Key::Char('\n')),
            Event::Input(Key::Char('h')),
            Event::Input(Key::Char('h')),
        ];

        let expected_modes = vec![
            InputMode::Suppressed(SuppressedAction::ShowHelp),
            InputMode::Normal,
            InputMode::Suppressed(SuppressedAction::ShowHelp),
            InputMode::Normal,
            InputMode::Suppressed(SuppressedAction::ShowHelp),
            InputMode::Normal,
        ];

        for i in 0..events.len() {
            let quit = interface
                .handle_input(events[i].clone())
                .expect("Failed to handle event");
            assert!(!quit);
            assert!(expected_modes[i] == interface.input_mode);
        }
    }

    #[test]
    fn test_handle_input_search() {
        let mut interface = init!(fix_url_records());

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

        println!("Should preserve search, when going in and out of Search mode...");
        let event = Event::Input(Key::Esc);
        let quit = interface
            .handle_input(event)
            .expect("Failed to handle event");
        assert!(!quit);
        assert!(InputMode::Normal == interface.input_mode);
        assert_eq!(2, interface.table.visible.len()); // URLs with tag 'tag'

        let event = Event::Input(Key::Char('/'));
        let quit = interface
            .handle_input(event)
            .expect("Failed to handle event");
        assert!(!quit);
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

            let mut interface = init!(fix_url_records());

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
