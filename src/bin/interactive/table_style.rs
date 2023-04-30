
use tui::style::{Color, Style, Modifier};

pub struct TableStyles {
    pub normal: Style,
    pub selected: Style,
    pub header: Style,
}

impl Default for TableStyles {
    fn default() -> Self {
        Self {
            selected: Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
            normal: Style::default().fg(Color::White),
            header: Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        }
    }
}
