use std::{error::Error, collections::HashMap};

use bookmark_lib::Registry;
use termion::event::Key;
use tui::{layout::{Layout, Direction, Constraint, Alignment}, Frame, widgets::{Row, Table, Block, Borders, Paragraph, Clear}, style::{Color, Style, Modifier}, backend::Backend, text::{Spans, Span}};

use crate::interactive::{table::TableItem, event::{Event, Signal}, table_style::TableStyles, interface::{InputMode, SuppressedAction}, modules::{ImportsModule, help::HelpPanel, edit_modal::EditModal, info_popup::InfoPopup}, widgets::rect::centered_fixed_rect};

use super::import::ImportsTable;

const HELP_TEXT: &str = r#"
Action               Description
'ENTER'            | enter folder, mark/unmark URL to import
'SPACEBAR'         | mark/unmark URL or whole folder to import
'BACKSPACE'        | go back to parent folder
'o'                | Open URL in the browser
'e'                | Modify URL and mark it to import
'CTRL + S'         | Save imports to bookmarks

--'/' or 'CTRL + F'  | search for URLs
--'d'                | delete URL
--'i'                | show/hide ids
'q'                | exit interactive import
--':'                | go to command mode

Command                Alias     Description
':tag <TAG_NAME>'    |         | add tag <TAG_NAME> to selected bookmark
':untag <TAG_NAME>'  |         | remove tag <TAG_NAME> from selected bookmark
':chgroup <GROUP>'   | chg     | change group to <GROUP> for selected bookmark
':chname <NAME>'     | chn     | change name to <NAME> for selected bookmark
':churl <URL>'       | chu     | change url to <URL> for selected bookmark
':sort [SORT_BY]'    |         | sort bookmarks by one of: [name, url, group]
':q'                 | quit    | exit interactive mode

"#;

pub struct ImportInterface<B: tui::backend::Backend> {
    imports_table: ImportsTable,
    styles: TableStyles,

    input_mode: InputMode,

    /// Interface modules
    modules: HashMap<InputMode, Box<dyn ImportsModule<B>>>,
    
    // TODO: this is crap. I should implement some eventing system...
    info_message: Option<String>,
}


impl<B: tui::backend::Backend> ImportInterface<B> {

    pub fn new(table: ImportsTable) -> ImportInterface<B> {

        let help_mod: Box<dyn ImportsModule<B>> = Box::new(HelpPanel::new(HELP_TEXT));
        let edit_mod: Box<dyn ImportsModule<B>> = Box::new(EditModal::new());

        // TODO: I need to rething those modules...
        let info_popup: Box<dyn ImportsModule<B>> = Box::new(InfoPopup::new());
        
        // TODO: I need search module here so bad!

        Self {
            imports_table: table,
            styles: TableStyles::default(),
            input_mode: InputMode::Normal,
            modules: hashmap![
                InputMode::Suppressed(SuppressedAction::ShowHelp) => help_mod,
                InputMode::Suppressed(SuppressedAction::Edit) => edit_mod,
                InputMode::InfoPopup => info_popup
            ],
            info_message: None,
        }
    }

    pub(crate) fn draw(&mut self, f: &mut Frame<B>) {
        let size = f.size();
        let normal_style = self.styles.normal;
    
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
    
        let columns = self.imports_table.columns().clone();
        let table = self.imports_table.table();
    
            // TODO: different style for URLs and folders
            // TODO: Sort folders to the top?
        let rows = table
            .items
            .iter()
            .map(|i| Row::StyledData(i.row().iter(), normal_style));
        let t = Table::new(columns.iter(), rows)
            .header_style(normal_style)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Import - Press 'h' to show help"),
            )
            .highlight_style(self.styles.selected)
            .highlight_symbol("> ")
            .widths(&[
                Constraint::Percentage(10),
                Constraint::Percentage(35),
                Constraint::Percentage(45),
                Constraint::Percentage(10),
            ]);
    
        f.render_stateful_widget(t, chunks[0], &mut table.state);
    
        if self.input_mode == InputMode::Confirmation {
            self.confirmation_popup(f);
        }

        // draw modules
        for module in self.modules.values() {
            module.draw(self.input_mode.clone(), f)
        }
    }

    pub(crate) fn handle_input(&mut self, event: Event<Key>) -> Result<bool, Box<dyn Error>> {
        if let Event::Input(input) = event {
            eprintln!("Input mode: {:?}", self.input_mode);
            match &self.input_mode {
                InputMode::Normal => match input {
                    Key::Char('q') => {
                        return Ok(true);
                    }
                    Key::Left => {
                        self.imports_table.unselect();
                    }
                    Key::Down => {
                        self.imports_table.next();
                    }
                    Key::Up => {
                        self.imports_table.previous();
                    }
                    Key::Char('\n') => {
                        self.imports_table.open_or_select()?;
                    }
                    // TODO: how? How would vim do it?
                    Key::Backspace => {
                        self.imports_table.exit_folder()?;
                    }
                    Key::Char(' ') => {
                        self.imports_table.toggle_selected()?;
                    }
                    Key::Char('o') => {
                        self.imports_table.open_url()?;
                    }
                    Key::Ctrl('s') => {
                        // TODO: display confirmation before saving
                        self.input_mode = InputMode::Confirmation;
                    }
                    // Activate first module that can handle the key - if none just skip
                    _ => {
                        for m in self.modules.values_mut() {
                            if let Some(mode) = m.try_activate(input, &mut self.imports_table)? {
                                self.input_mode = mode;
                                return Ok(false);
                            }
                        }
                    }
                }
                InputMode::Confirmation => match input {
                    Key::Char('\n') => {
                        self.imports_table.import_selected()?;
                        // TODO: Display confirmation that n URL have been imported                        

                        self.input_mode = InputMode::Normal;
                    }
                    Key::Esc => {
                        self.input_mode = InputMode::Normal;
                    }
                    _ => {}
                }
                _ => {
                    if let Some(module) = self.modules.get_mut(&self.input_mode) {
                        if let Some(new_mode) =
                            module.handle_input(input, &mut self.imports_table)?
                        {
                            self.input_mode = new_mode;
                        }
                    }
                }
            }
        }
        if let Event::Signal(s) = event {
            match s {
                Signal::Quit => return Ok(true),
            }
        }

        Ok(false)
    }

    // TODO: maybe this should be implemented as a module? I do not know
    // it feels cluncky. It would need to use some callbacks or closures.
    fn confirmation_popup(&self, f: &mut Frame<B>) {
        let area = centered_fixed_rect(60, 6, f.size());

        // TODO: maybe display number of selected URLs
        let text = vec![
            Spans::from(""),
            Spans::from(format!(
                "Are you sure to save selected URLs to your bookmarks?"
            )),
            Spans::from(""),
            Spans::from("Yes (Enter)   ---   No (ESC)"), // TODO: consider y and n as confirmation
        ];

        let block = Block::default()
            .borders(Borders::ALL)
            .style(Style::default().bg(Color::Black).fg(Color::LightBlue))
            .title(Span::styled(
                "Confirm import".to_string(),
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
