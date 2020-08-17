use termion::event::Key;
use tui::backend::Backend;

pub trait Action {
    fn apply(&self);
    fn update(key: Key);
    fn draw<T: Backend>(backend: T);
}

pub struct EditTag {

}

