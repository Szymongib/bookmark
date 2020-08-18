use termion::event::Key;
use tui::backend::Backend;
use tui::Frame;
use std::error::Error;
use crate::interactive::table::StatefulTable;
use crate::interactive::url_table_item::URLItem;
use crate::interactive::interface::InputMode;
use bookmark_lib::Registry;
use bookmark_lib::record_filter::FilterSet;
use std::marker::PhantomData;
use tui::widgets::{Paragraph, Block, Borders, Clear};
use tui::style::Style;
use tui::layout::{Rect, Layout, Direction, Constraint};
use crate::interactive::modules::Module;


pub(crate) struct Search {
    search_phrase: String,
}

impl<R, B> Module<R, B> for Search
    where R: Registry, B: Backend
{
    fn handle_input(&mut self, input: Key, registry: &R, table: &mut StatefulTable<URLItem>) -> Result<InputMode, Box<dyn std::error::Error>> {
        match input {
            Key::Esc | Key::Up | Key::Down | Key::Char('\n') => {
                table.unselect();
                return Ok(InputMode::Normal)
            }
            Key::Char(c) => {
                self.search_phrase.push(c);
            }
            Key::Backspace => {
                self.search_phrase.pop();
            }
            _ => {}
        }

        Ok(self.apply_search(table))
    }

    fn draw(&self, mode: InputMode, f: &mut Frame<B>) {
        return match mode {
            InputMode::Search => {
                self.render_search_input(f);
                // Make the cursor visible and ask tui-rs to put it at the specified coordinates after rendering
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
        Search{search_phrase: "".to_string()}
    }

    /// updates URLs visibility inside the `table` according to the `search_phrase`
    fn apply_search(&mut self, table: &mut StatefulTable<URLItem>) -> InputMode {

        let filter = FilterSet::new_combined_filter(self.search_phrase.clone().as_str());

        for item in &mut table.items {
            item.filter(&filter)
        }

        table.refresh_visible();

        InputMode::Search
    }

    pub fn render_search_input<B: tui::backend::Backend>(&self, f: &mut Frame<B>) {
        let input_widget = Paragraph::new(self.search_phrase.as_ref())
            .style(Style::default())
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Press '/' or 'CTRL + f' to search for URLs"),
            );

        let block = self.centered_search_input(f.size());

        f.render_widget(Clear, block);
        f.render_widget(input_widget, block);  // TODO: render stateful widget?
    }

    fn centered_search_input(&self, r: Rect) -> Rect {
        let search_input = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Length(r.height - 3),
                    Constraint::Length(3),
                    Constraint::Length(r.height),
                ]
                    .as_ref(),
            )
            .split(r);

        Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Length(0),
                    Constraint::Length(r.width),
                    Constraint::Length(r.width),
                ]
                    .as_ref(),
            )
            .split(search_input[1])[1]
    }
}


