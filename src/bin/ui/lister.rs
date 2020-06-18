use std::{error::Error, io};
use termion::{event::Key, input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};
use tui::{
    backend::TermionBackend,
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders},
    Terminal,
};

use super::event::{Event, Events};

use crate::ui::table::StatefulTable;
use bookmark_lib::types::URLRecord;
use tui::widgets::{Row, Table};

pub fn display_urls(urls: Vec<URLRecord>) -> Result<(), Box<dyn Error>> {
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    let events = Events::new();

    let items: Vec<Vec<String>> = urls.iter().map(|u| url_to_row(u)).collect();

    let mut table = StatefulTable::with_items(items);

    loop {
        terminal.draw(|mut f| {
            let chunks = Layout::default()
                // .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(100), Constraint::Percentage(100)].as_ref())
                .split(f.size());

            let selected_style = Style::default().fg(Color::Green).modifier(Modifier::BOLD);
            let normal_style = Style::default().fg(Color::White);
            let header = ["  Name", "URL", "Group", "Tags"];
            let rows = table
                .items
                .iter()
                .map(|i| Row::StyledData(i.iter(), normal_style));
            let t = Table::new(header.iter(), rows)
                .block(Block::default().borders(Borders::ALL).title("URLs"))
                .highlight_style(selected_style)
                .highlight_symbol("> ")
                .widths(&[
                    Constraint::Percentage(20),
                    Constraint::Percentage(40),
                    Constraint::Percentage(15),
                    Constraint::Percentage(15),
                ]);
            f.render_stateful_widget(t, chunks[0], &mut table.state);
        })?;

        match events.next()? {
            Event::Input(input) => match input {
                Key::Char('q') => {
                    return Ok(());
                }
                Key::Left => {
                    table.unselect();
                }
                Key::Down => {
                    table.next();
                }
                Key::Up => {
                    table.previous();
                }
                Key::Char('\n') => {
                    let selected_id = table.state.selected();
                    if selected_id.is_none() {
                        continue;
                    }
                    let selected_id = selected_id.unwrap();

                    let item = &table.items[selected_id];

                    let res = open::that(item[1].as_str());
                    if let Err(err) = res {
                        return Err(From::from(format!(
                            "failed to open URL in the browser: {}",
                            err.to_string()
                        )));
                    }
                }
                _ => {}
            },
        }
    }
}

fn url_to_row(record: &URLRecord) -> Vec<String> {
    let tags: Vec<&str> = record.tags.keys().map(|k| k.as_str()).collect();

    vec![
        record.name.clone(),
        record.url.clone(),
        record.group.clone(),
        tags.join(", "),
    ]
}

#[cfg(test)]
mod test {
    use crate::ui::lister::url_to_row;
    use crate::ui::table::StatefulTable;
    use bookmark_lib::types::URLRecord;

    struct TestCase {
        url_record: URLRecord,
        expected: Vec<String>,
    }

    #[test]
    fn url_to_row_test() {
        let items = vec![
            TestCase {
                url_record: URLRecord::new("url1", "name1", "group1", vec!["tag1, tag1.2"]),
                expected: vec![
                    "name1".to_string(),
                    "url1".to_string(),
                    "group1".to_string(),
                    "tag1, tag1.2".to_string(),
                ],
            },
            TestCase {
                url_record: URLRecord::new("url2", "name2", "group2", vec!["tag2, tag2.2"]),
                expected: vec![
                    "name2".to_string(),
                    "url2".to_string(),
                    "group2".to_string(),
                    "tag2, tag2.2".to_string(),
                ],
            },
            TestCase {
                url_record: URLRecord::new("url3", "name3", "group3", vec!["tag3, tag3.2"]),
                expected: vec![
                    "name3".to_string(),
                    "url3".to_string(),
                    "group3".to_string(),
                    "tag3, tag3.2".to_string(),
                ],
            },
            TestCase {
                url_record: URLRecord::new("url4", "name4", "group4", vec![]),
                expected: vec![
                    "name4".to_string(),
                    "url4".to_string(),
                    "group4".to_string(),
                    "".to_string(),
                ],
            },
        ];

        for item in items {
            let vec = url_to_row(&item.url_record);
            assert_eq!(item.expected, vec)
        }
    }
}
