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

pub trait Module<R, B>
    where R: Registry, B: Backend
{
    fn handle_input(&mut self, input: Key, registry: &R, table: &mut StatefulTable<URLItem>) -> Result<InputMode, Box<dyn std::error::Error>>;
    fn draw(&mut self, f: &mut Frame<B>); // TODO: does it need to be mut self?
}


// TODO: how do I modify table? How do I set new mode?

// pub type Operation<R: Registry> = Box<dyn Fn(&R, &mut StatefulTable<URLItem>)>;

// pub struct StateUpdate<R>
// where R: Registry
// {
//     operation: Operation<R>, // TODO: should prob return some err
//     mode: InputMode,
//     __phantom: PhantomData<R>
// }
//
// impl<R> StateUpdate<R>
//     where R: Registry
// {
//     pub fn new(operation: Operation<R>, mode: InputMode) -> StateUpdate<R> {
//         StateUpdate{
//             operation,
//             mode,
//             __phantom: PhantomData
//         }
//     }
//
//     pub fn run(&mut self, registry: &R, table: &mut StatefulTable<URLItem>) -> InputMode {
//         &(self.operation)(registry, table);
//         self.mode.clone()
//     }
// }


pub(crate) struct Search {
    search_phrase: String,
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

        return Ok(self.apply_search(table))
    }

    fn draw(&mut self, f: &mut Frame<B>) {
        // Make the cursor visible and ask tui-rs to put it at the specified coordinates after rendering
        f.set_cursor(
            // Put cursor past the end of the input text
            self.search_phrase.len() as u16 + 1, // TODO: consider using crate UnicodeWidth
            // Move two line up from the bottom - search input
            f.size().height - 2,
        )
    }
}

