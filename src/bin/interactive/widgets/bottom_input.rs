
pub fn bottom_input(title: &str, ) {
    let input_widget = Paragraph::new(self.command_input.as_ref())
        .style(Style::default())
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Press '/' or 'CTRL + f' to search for URLs"),
        );

}