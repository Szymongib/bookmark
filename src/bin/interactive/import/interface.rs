use std::error::Error;

use termion::event::Key;
use tui::{layout::{Layout, Direction, Constraint}, Frame, widgets::{Row, Table, Block, Borders}, style::Color};

use crate::interactive::{table::TableItem, event::{Event, Signal}, table_style::TableStyles};

use super::import::ImportsTable;



pub struct ImportInterface<B: tui::backend::Backend> {
    table: ImportsTable,
    styles: TableStyles,

    _phantom: std::marker::PhantomData<B>,
}


impl<B: tui::backend::Backend> ImportInterface<B> {

    pub fn new(table: ImportsTable) -> ImportInterface<B> {
        Self {
            table,
            styles: TableStyles::default(),
            _phantom: std::marker::PhantomData,
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
    
        let columns = self.table.columns().clone();
        let table = self.table.table();
    
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
        // for module in self.modules.values() {
        //     module.draw(self.input_mode.clone(), f)
        // }
    }

    pub(crate) fn handle_input(&mut self, event: Event<Key>) -> Result<bool, Box<dyn Error>> {
        if let Event::Input(input) = event {
            match input {
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
                        self.table.open()?;
                    }
                    // TODO: how? How would vim do it?
                    Key::Backspace => {
                        self.table.exit_folder()?;
                    }
                    // Key::Char('i') => {
                    //     self.toggle_ids_display()?;
                    // }
                    _ => {}
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
