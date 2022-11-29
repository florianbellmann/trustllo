use tui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Span},
    widgets::{Block, Borders},
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
        .constraints([Constraint::Length(3) ,Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
}

pub fn get_card() -> Block<'static> {
    Block::default().title("Card or I can immediately use the card title here").borders(Borders::ALL)
}
pub fn get_card_title() -> Block<'static> {
    Block::default().title("title").borders(Borders::ALL)
}

pub fn get_card_description() -> Block<'static> {
    Block::default().title("Description").borders(Borders::ALL)
}

pub fn get_card_checklist() -> Block<'static> {
    Block::default().title("Checklist").borders(Borders::ALL)
}

// let card_block = Block::default().title("Card").borders(Borders::ALL);
// f.render_widget(card_block, left_layout[1]);

// // right pane
// let lists_block = Block::default().title("Lists").borders(Borders::ALL);
// f.render_widget(lists_block, column_layout[1]);

// let list_layout = Layout::default()
//     .direction(Direction::Horizontal)
//     .margin(1)
//     .constraints(
//         [
//             Constraint::Percentage(25),
//             Constraint::Percentage(25),
//             Constraint::Percentage(25),
//             Constraint::Percentage(25),
//         ]
//         .as_ref(),
//     )
//     .split(column_layout[1]);

// let list_1 = Block::default().title("List 1").borders(Borders::ALL);
// f.render_widget(list_1, list_layout[0]);
// let list_2 = Block::default().title("List 2").borders(Borders::ALL);
// f.render_widget(list_2, list_layout[1]);

// // Text
// let texts = vec![
//     Spans::from("tjdskl"),
//     Spans::from("tjdskl"),
//     Spans::from("tjdskl"),
// ];
// let p = Paragraph::new(texts);
// // .block(lists_block);
// f.render_widget(p, list_layout[2]);

// let lst_items: Vec<ListItem> = ["jkl", "jjdfskl", "fjdsklslkjf"]
//     .iter()
//     .map(|_i| ListItem::new("fdsjkl"))
//     .collect();

// let list = List::new(lst_items)
//     .block(
//         Block::default()
//             .title("titleee")
//             .borders(Borders::ALL)
//             .border_style(Style::default()),
//     )
//     .style(Style::default());
// f.render_widget(list, list_layout[3]);
