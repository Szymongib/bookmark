use crate::interactive::helpers::{horizontal_layout, vertical_layout};
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

// TODO: test

/// helper function to create a centered rect with a specified size
pub(crate) fn centered_fixed_rect(size_x: u16, size_y: u16, r: Rect) -> Rect {
    let popup_layout = vertical_layout(vec![
        (r.height - size_y) / 2,
        size_y,
        (r.height - size_y) / 2,
    ])
    .split(r);

    horizontal_layout(vec![(r.width - size_x) / 2, size_x, (r.width - size_x) / 2])
        .split(popup_layout[1])[1]
}

#[cfg(test)]
mod test {
    use crate::interactive::widgets::rect::{centered_fixed_rect, centered_rect};
    use tui::layout::Rect;

    #[test]
    fn test_create_centered_fixed_rect() {
        let base = Rect::new(0, 0, 10, 10);
        let rect = centered_fixed_rect(6, 4, base);

        assert_eq!(rect.width, 6);
        assert_eq!(rect.height, 4);
        assert_eq!(rect.x, 2);
        assert_eq!(rect.y, 3);

        let rect = centered_fixed_rect(7, 3, base);

        assert_eq!(rect.width, 7);
        assert_eq!(rect.height, 3);
        assert_eq!(rect.x, 1);
        assert_eq!(rect.y, 3);
    }

    #[test]
    fn test_create_centered_rect() {
        let base = Rect::new(0, 0, 10, 10);
        let rect = centered_rect(60, 40, base);

        assert_eq!(rect.width, 6);
        assert_eq!(rect.height, 4);
        assert_eq!(rect.x, 2);
        assert_eq!(rect.y, 3);

        let rect = centered_rect(70, 30, base);

        assert_eq!(rect.width, 7);
        assert_eq!(rect.height, 3);
        assert_eq!(rect.x, 1);
        assert_eq!(rect.y, 3);
    }
}
