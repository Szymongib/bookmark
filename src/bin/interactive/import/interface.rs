use std::{error::Error, collections::HashMap};

use termion::event::Key;
use tui::{layout::{Layout, Direction, Constraint}, Frame, widgets::{Row, Table, Block, Borders}, style::Color};

use crate::interactive::{table::TableItem, event::{Event, Signal}, table_style::TableStyles, interface::{InputMode, SuppressedAction}, modules::{ImportsModule, help::HelpPanel}};

use super::import::ImportsTable;

const HELP_TEXT: &str = r#"
Action               Description
'ENTER'            | enter folder, mark/unmark URL to import
--'SHIFT + ENTER'     | Open URL in browser
--'Backspace'         | go back to parent folder -- TODO: different shortcut?
--'CTRL + ENTER'   | Modify URL and mark it to import

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
}


impl<B: tui::backend::Backend> ImportInterface<B> {

    pub fn new(table: ImportsTable) -> ImportInterface<B> {

        let help_mod: Box<dyn ImportsModule<B>> = Box::new(HelpPanel::new(HELP_TEXT));

        Self {
            imports_table: table,
            styles: TableStyles::default(),
            input_mode: InputMode::Normal,
            modules: hashmap![
                InputMode::Suppressed(SuppressedAction::ShowHelp) => help_mod
            ],
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
    
        // draw modules
        for module in self.modules.values() {
            module.draw(self.input_mode.clone(), f)
        }
    }

    pub(crate) fn handle_input(&mut self, event: Event<Key>) -> Result<bool, Box<dyn Error>> {
        if let Event::Input(input) = event {
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
                        self.imports_table.open()?;
                    }
                    // TODO: how? How would vim do it?
                    Key::Backspace => {
                        self.imports_table.exit_folder()?;
                    }
                    // Key::Char('i') => {
                    //     self.toggle_ids_display()?;
                    // }
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
}
