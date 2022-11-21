
pub fn get_card(card:&Card){

    let card_detail_layout = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(left_layout[1]);

    let card_description = Block::default().title("Description").borders(Borders::ALL);
    f.render_widget(card_description, card_detail_layout[0]);
    let card_checklist = Block::default().title("Checklist").borders(Borders::ALL);
    f.render_widget(card_checklist, card_detail_layout[1]);

}
