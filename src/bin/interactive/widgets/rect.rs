use tui::layout::{Constraint, Direction, Layout, Rect};

/// helper function to create a centered rect using up
/// certain percentage of the available rect `r`
pub(crate) fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}

/// helper function to create a centered rect with a specified size
pub(crate) fn centered_fixed_rect(size_x: u16, size_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length((r.height - size_y)/2),
                Constraint::Length(size_y),
                Constraint::Length((r.height - size_y)/2),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Length((r.width - size_x)/2),
                Constraint::Length(size_x),
                Constraint::Length((r.width - size_x)/2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}
