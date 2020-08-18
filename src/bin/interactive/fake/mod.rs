use tui::buffer::Cell;
use std::io::Error;
use tui::layout::Rect;

pub struct MockBackend {}
impl tui::backend::Backend for MockBackend {
    fn draw<'a, I>(&mut self, content: I) -> Result<(), Error> where
        I: Iterator<Item=(u16, u16, &'a Cell)> {
        unimplemented!()
    }

    fn hide_cursor(&mut self) -> Result<(), Error> {
        unimplemented!()
    }

    fn show_cursor(&mut self) -> Result<(), Error> {
        unimplemented!()
    }

    fn get_cursor(&mut self) -> Result<(u16, u16), Error> {
        unimplemented!()
    }

    fn set_cursor(&mut self, x: u16, y: u16) -> Result<(), Error> {
        unimplemented!()
    }

    fn clear(&mut self) -> Result<(), Error> {
        unimplemented!()
    }

    fn size(&self) -> Result<Rect, Error> {
        unimplemented!()
    }

    fn flush(&mut self) -> Result<(), Error> {
        unimplemented!()
    }
}
