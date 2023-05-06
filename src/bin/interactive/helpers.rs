use crate::interactive::event::Event;
use termion::event::Key;
use tui::layout::{Constraint, Direction, Layout, Rect};

// TODO: consider moving to some lib
macro_rules! hashmap {
    ($( $key: expr => $val: expr ),*) => {{
         let mut map = ::std::collections::HashMap::new();
         $( map.insert($key, $val); )*
         map
    }}
}

pub fn vertical_layout(heights: Vec<u16>) -> Layout {
    let constraints = to_constraints(heights);

    Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints)
}

pub fn horizontal_layout(widths: Vec<u16>) -> Layout {
    let constraints = to_constraints(widths);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(constraints)
}

fn to_constraints(vals: Vec<u16>) -> Vec<Constraint> {
    vals.iter().map(|h| Constraint::Length(*h)).collect()
}

pub fn to_keys(text: &str) -> Vec<Key> {
    text.chars().map(Key::Char).collect()
}

pub fn to_key_events(text: &str) -> Vec<Event<Key>> {
    text.chars().map(|c| Event::Input(Key::Char(c))).collect()
}

pub fn to_string(vec: Vec<&str>) -> Vec<String> {
    vec.iter().map(|s| s.to_string()).collect()
}

pub fn pad_layout(area: Rect, padding: [u16; 4]) -> Rect {
    let [top, right, bottom, left] = padding;

    let top = if top > area.height { area.height } else { top };
    let right = if right > area.width { area.width } else { right };
    let bottom = if bottom > area.height { area.height } else { bottom };
    let left = if left > area.width { area.width } else { left };

    let top = area.y + top;
    let right = area.x + area.width - right;
    let bottom = area.y + area.height - bottom;
    let left = area.x + left;

    Rect::new(left, top, right - left, bottom - top)
}
