use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, Borders},
    Frame,
};

pub fn draw_interface<B: Backend>(f: &mut Frame<B>) {
    let size = f.size();

    // Surrounding block
    let main_window = Block::default()
        .borders(Borders::ALL)
        .title(Span::styled(" TRusTLLo ", Style::default().fg(Color::Red)))
        .title_alignment(Alignment::Center);

    f.render_widget(main_window, size);

    let column_layout = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints([Constraint::Percentage(35), Constraint::Percentage(65)].as_ref())
        .split(f.size());

    // left pane
    let left_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Percentage(100)].as_ref())
        .split(column_layout[0]);

    let board_block = Block::default().title("Board").borders(Borders::ALL);
    f.render_widget(board_block, left_layout[0]);

    let card_block = Block::default().title("Card").borders(Borders::ALL);
    f.render_widget(card_block, left_layout[1]);

    let card_detail_layout = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(left_layout[1]);

    let card_description = Block::default().title("Description").borders(Borders::ALL);
    f.render_widget(card_description, card_detail_layout[0]);
    let card_checklist = Block::default().title("Checklist").borders(Borders::ALL);
    f.render_widget(card_checklist, card_detail_layout[1]);

    // right pane
    let lists_block = Block::default().title("Lists").borders(Borders::ALL);
    f.render_widget(lists_block, column_layout[1]);

    let list_layout = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints(
            [
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
            ]
            .as_ref(),
        )
        .split(column_layout[1]);

    let list_1 = Block::default().title("List 1").borders(Borders::ALL);
    f.render_widget(list_1, list_layout[0]);
    let list_2 = Block::default().title("List 2").borders(Borders::ALL);
    f.render_widget(list_2, list_layout[1]);
    let list_3 = Block::default().title("List 3").borders(Borders::ALL);
    f.render_widget(list_3, list_layout[2]);
    let list_4 = Block::default().title("List 4").borders(Borders::ALL);
    f.render_widget(list_4, list_layout[3]);
}
