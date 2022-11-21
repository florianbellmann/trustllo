pub fn get_list(list: &List) {
    let list = List::new(lst_items)
        .block(
            Block::default()
                .title("titleee")
                .borders(Borders::ALL)
                .border_style(Style::default()),
        )
        .style(Style::default());
    f.render_widget(list, list_layout[3]);
}
