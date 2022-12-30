use tui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, List, ListItem},
};

pub fn get_main_window() -> Block<'static> {
    Block::default()
        .borders(Borders::ALL)
        .title(Span::styled(" TRusTLLo ", Style::default().fg(Color::Red)))
        .title_alignment(Alignment::Center)
}

pub fn get_main_layout() -> Layout {
    Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints([Constraint::Percentage(35), Constraint::Percentage(65)].as_ref())
}

pub fn get_left_pane_layout() -> Layout {
    Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Percentage(100)].as_ref())
}

pub fn get_board_header() -> Block<'static> {
    Block::default().title("Board").borders(Borders::ALL)
}

pub fn get_card_detail_layout() -> Layout {
    Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ]
            .as_ref(),
        )
}

pub fn get_card_detail() -> Block<'static> {
    Block::default()
        .title("Card or I can immediately use the card title here")
        .borders(Borders::ALL)
}

pub fn get_card_detail_title() -> Block<'static> {
    Block::default().title("title").borders(Borders::ALL)
}

pub fn get_card_detail_description() -> Block<'static> {
    Block::default().title("Description").borders(Borders::ALL)
}

pub fn get_card_detail_checklist() -> Block<'static> {
    Block::default().title("Checklist").borders(Borders::ALL)
}

pub fn get_right_pane_layout() -> Layout {
    Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(100)].as_ref())
}

pub fn get_lists() -> Block<'static> {
    Block::default().title("Lists").borders(Borders::ALL)
}

pub fn get_lists_layout() -> Layout {
    Layout::default()
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
}

pub fn get_list(card_names: Vec<String>) -> List<'static> {
    let list_items: Vec<ListItem> = card_names
        .into_iter()
        .map(|card_name| ListItem::new(card_name).style(Style::default().fg(Color::White)))
        .collect();

    // Create a List from all list items and highlight the currently selected one
    List::new(list_items)
        .block(Block::default().borders(Borders::ALL).title("List"))
        .highlight_style(
            Style::default()
                .bg(Color::LightGreen)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ")
}
