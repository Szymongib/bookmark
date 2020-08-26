use crate::interactive::bookmarks_table::BookmarksTable;
use crate::interactive::event::{Event, Signal};
use crate::interactive::table::{TableItem};
use std::error::Error;
use termion::event::Key;
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, Row, Table};
use tui::Frame;
use std::collections::HashMap;
use crate::interactive::modules::{Module};
use crate::interactive::modules::search::Search;
use crate::interactive::modules::help::HelpPanel;
use crate::interactive::modules::delete::Delete;
use crate::interactive::modules::command::Command;

// TODO: some decisions
// - drop Add functionality from interactive mode for now
// - ':et' - edit tag of selected
// - ':eg' - edit group of selected
// - ':g [GROUP]' - filter by group
// - ':t [TAG]' - filter by tag
// - '????" - remove filters
// TODO: Help not as a popup but toggle?
// TODO: Toggle show ids


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

pub struct Interface<B: tui::backend::Backend> {
    bookmarks_table: BookmarksTable,

    /// Interface modules
    modules: HashMap<InputMode, Box<dyn Module<B>>>,

    /// Current mode of input
    input_mode: InputMode,

    /// Styles used for displaying user interface
    styles: Styles,
}

struct Styles {
    normal: Style,
    selected: Style,
    header: Style,
}

impl<B: tui::backend::Backend> Interface<B> {
    pub(crate) fn new(bookmarks_table: BookmarksTable) -> Result<Interface<B>, Box<dyn std::error::Error>> {
        let search_mod: Box<dyn Module<B>> = Box::new(Search::new());
        let help_mod: Box<dyn Module<B>> = Box::new(HelpPanel::new());
        let delete_mod: Box<dyn Module<B>> = Box::new(Delete::new());
        let command_mod: Box<dyn Module<B>> = Box::new(Command::new());


        Ok(Interface {
            bookmarks_table,
            input_mode: InputMode::Normal,
            modules: hashmap![
                InputMode::Search => search_mod,
                InputMode::Suppressed(SuppressedAction::ShowHelp) => help_mod,
                InputMode::Suppressed(SuppressedAction::Delete) => delete_mod,
                InputMode::Command => command_mod
            ],
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
                        self.bookmarks_table.unselect();
                    }
                    Key::Down => {
                        self.bookmarks_table.next();
                    }
                    Key::Up => {
                        self.bookmarks_table.previous();
                    }
                    Key::Char('\n') => {
                        self.bookmarks_table.open()?;
                    }
                    // Activate first module that can handle the key - if none just skip
                    _ => {
                        for m in self.modules.values_mut() {
                            if let Some(mode) = m.try_activate(input, &mut self.bookmarks_table)? {
                                self.input_mode = mode;
                                return Ok(false)
                            }
                        }
                    }
                },
                _ => {
                    if let Some(module) = self.modules.get_mut(&self.input_mode) {
                        if let Some(new_mode) = module.handle_input(input, &mut self.bookmarks_table)? {
                            self.input_mode = new_mode;
                        }
                    }
                }
            }
        }
        if let Event::Signal(s) = event {
            match s {
                Signal::Quit => return Ok(true)
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

        let table = self.bookmarks_table.table();

        let header = ["  Name", "URL", "Group", "Tags"];
        let rows = table.items.iter().map(|i| {
            Row::StyledData(i.row().iter(), normal_style)
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

        f.render_stateful_widget(t, chunks[0], &mut table.state);

        // draw modules
        for module in self.modules.values() {
            module.draw(self.input_mode.clone(), f)
        }
    }
}

#[cfg(test)]
mod test {
    use crate::interactive::event::{Event, Events, Signal};
    use crate::interactive::bookmarks_table::BookmarksTable;
    use crate::interactive::interface::{InputMode, Interface, SuppressedAction};
    use crate::interactive::fake::{MockBackend};
    use bookmark_lib::types::URLRecord;
    use termion::event::Key;
    use bookmark_lib::registry::URLRegistry;
    use std::path::{PathBuf, Path};
    use bookmark_lib::Registry;
    use std::{fs};
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
            if Path::new(&self.file_path).exists() {
                fs::remove_file(&self.file_path).expect("Failed to remove file");
            }
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
            let cleaner = Cleaner::new(file_path); // makes sure that temp file is deleted even in case of panic
            $(
                for u in $urls {
                    registry.add_url(u).expect("Failed to add url");
                }
            )*

            let events = Events::new();
            let bookmarks_table = BookmarksTable::new(events.tx.clone(), Box::new(registry)).expect("Failed to initialize Bookmarks table");

            let interface = Interface::<MockBackend>::new(bookmarks_table).expect("Failed to initialize interface");

            (interface, cleaner)
        };
    );
    () => (
        init!(vec![])
    );
    }

    #[test]
    fn test_handle_input_returns() {
        let (mut interface, _) = init!();

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

        // Should quit on Signal(Quit)
        let event = Event::Signal(Signal::Quit);
        let quit = interface
            .handle_input(event)
            .expect("Failed to handle event");
        assert!(quit);
    }

    #[test]
    fn test_handle_input_input_modes() {
        let (mut interface, _)  = init!();

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
        let (mut interface, _)  = init!();

        assert!(InputMode::Normal == interface.input_mode);

        println!("Should switch InputModes...");
        let events = vec![
            Event::Input(Key::Char('h')),
            Event::Input(Key::Esc),
            Event::Input(Key::Char('h')),
            Event::Input(Key::Char('\n')),
            Event::Input(Key::Char('h')),
            Event::Input(Key::Char('h')),
            Event::Input(Key::Char('/')),
            Event::Input(Key::Esc),
            Event::Input(Key::Ctrl('f')),
            Event::Input(Key::Esc),
            Event::Input(Key::Char(':')),
            Event::Input(Key::Char('a')),
            Event::Input(Key::Esc),
        ];

        let expected_modes = vec![
            InputMode::Suppressed(SuppressedAction::ShowHelp),
            InputMode::Normal,
            InputMode::Suppressed(SuppressedAction::ShowHelp),
            InputMode::Normal,
            InputMode::Suppressed(SuppressedAction::ShowHelp),
            InputMode::Normal,
            InputMode::Search,
            InputMode::Normal,
            InputMode::Search,
            InputMode::Normal,
            InputMode::Command,
            InputMode::Command,
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
        let (mut interface, _cleaner)  = init!(fix_url_records());

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
        assert_eq!(2, interface.bookmarks_table.table().items.len()); // URLs with tag 'tag'

        println!("Should preserve search, when going in and out of Search mode...");
        let event = Event::Input(Key::Esc);
        let quit = interface
            .handle_input(event)
            .expect("Failed to handle event");
        assert!(!quit);
        assert!(InputMode::Normal == interface.input_mode);
        assert_eq!(2, interface.bookmarks_table.table().items.len()); // URLs with tag 'tag'

        let event = Event::Input(Key::Char('/'));
        let quit = interface
            .handle_input(event)
            .expect("Failed to handle event");
        assert!(!quit);
        assert_eq!(2, interface.bookmarks_table.table().items.len()); // URLs with tag 'tag'

        println!("Should filter items in table on backspace...");
        for event in vec![Event::Input(Key::Backspace), Event::Input(Key::Backspace)] {
            let quit = interface
                .handle_input(event)
                .expect("Failed to handle event");
            assert!(!quit);
        }
        assert_eq!(4, interface.bookmarks_table.table().items.len()); // URLs with letter 't'
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

            let (mut interface, _)  = init!(fix_url_records());

            for i in 0..test.events.len() {
                let quit = interface
                    .handle_input(test.events[i].clone())
                    .expect("Failed to handle event");
                assert!(!quit);
                assert_eq!(test.selected[i], interface.bookmarks_table.table().state.selected())
            }
        }
    }
}
