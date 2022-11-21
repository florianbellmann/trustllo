pub fn get_main_window() {
    // Surrounding block
    let main_window = Block::default()
        .borders(Borders::ALL)
        .title(Span::styled(" TRusTLLo ", Style::default().fg(Color::Red)))
        .title_alignment(Alignment::Center);

    f.render_widget(main_window, size);
}

pub fn get_column_layout() {
    let column_layout = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints([Constraint::Percentage(35), Constraint::Percentage(65)].as_ref())
        .split(f.size());
}

pub fn get_left_pane() {
    // left pane
    let left_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Percentage(100)].as_ref())
        .split(column_layout[0]);
}

pub fn get_right_pane() {
    // right pane
    let lists_block = Block::default().title("Lists").borders(Borders::ALL);
    f.render_widget(lists_block, column_layout[1]);
}
