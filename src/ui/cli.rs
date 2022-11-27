use anyhow::Result;
use std::io::{stdin, stdout, Stdout, Write};

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{io};
use tui::{backend::CrosstermBackend, Terminal};

use std::{io::stderr, thread, time::Duration};

use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};




pub struct Cli {
    terminal: Terminal<CrosstermBackend<Stdout>>,
}

impl Cli {
    pub fn new() -> Cli {
        // Check if the current environment is in a terminal.
        Cli::check_if_terminal();

        // setup terminal
        enable_raw_mode(); // send data byte by byte to terminal
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture);
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend).unwrap();

        Cli { terminal }
    }

    pub fn restore(&mut self) {
        // restore terminal
        disable_raw_mode();

        execute!(
            self.terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        );

        let _ = self.terminal.show_cursor();
    }

    pub fn draw(&mut self) -> Result<()> {
        self.terminal.draw(|f| Self::ui(f))?;
        Ok(())
    }

    fn ui<B: Backend>(f: &mut Frame<B>) {
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

        // Text
        let texts = vec![
            Spans::from("tjdskl"),
            Spans::from("tjdskl"),
            Spans::from("tjdskl"),
        ];
        let p = Paragraph::new(texts);
        // .block(lists_block);
        f.render_widget(p, list_layout[2]);

        let lst_items: Vec<ListItem> = ["jkl", "jjdfskl", "fjdsklslkjf"]
            .iter()
            .map(|_i| ListItem::new("fdsjkl"))
            .collect();

        let list = List::new(lst_items)
            .block(
                Block::default()
                    .title("titleee")
                    .borders(Borders::ALL)
                    .border_style(Style::default()),
            )
            .style(Style::default());
        f.render_widget(list, list_layout[3]);

        // let card_block = Block::default().title("Card").borders(Borders::ALL);
        // f.render_widget(card_block, left_layout[1]);
        //         let board_block = Block::default().title("Board").borders(Borders::ALL);
        // f.render_widget(board_block, left_layout[0]);

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
        //         let lst_items: Vec<ListItem> = ["jkl", "jjdfskl", "fjdsklslkjf"]
        //     .iter()
        //     .map(|_i| ListItem::new("fdsjkl"))
        //     .collect();
    }

    pub fn redraw() {}

    // fn drawboard(){

    // }
    // fn drawcards(){

    // }
    // fn drawlists(){
    // }

    pub fn read_config_from_user_input(&self) -> (String, String, String) {
        let mut api_key = String::new();
        print!("Please enter your api key: ");
        let _ = stdout().flush();
        stdin()
            .read_line(&mut api_key)
            .expect("Did not enter a correct string");
        if let Some('\n') = api_key.chars().next_back() {
            api_key.pop();
        }
        if let Some('\r') = api_key.chars().next_back() {
            api_key.pop();
        }

        let mut api_token = String::new();
        print!("Please enter your api token: ");
        let _ = stdout().flush();
        stdin()
            .read_line(&mut api_token)
            .expect("Did not enter a correct string");
        if let Some('\n') = api_token.chars().next_back() {
            api_token.pop();
        }
        if let Some('\r') = api_token.chars().next_back() {
            api_token.pop();
        }

        let mut member_id = String::new();
        print!("Please enter your member id: ");
        let _ = stdout().flush();
        stdin()
            .read_line(&mut member_id)
            .expect("Did not enter a correct string");
        if let Some('\n') = member_id.chars().next_back() {
            member_id.pop();
        }
        if let Some('\r') = member_id.chars().next_back() {
            member_id.pop();
        }

        (api_key, api_token, member_id)
    }

    /// Check and report to the user if the current environment is not a terminal.
    fn check_if_terminal() {
        use crossterm::tty::IsTty;

        if !stdout().is_tty() {
            eprintln!(
            "Warning: bottom is not being output to a terminal. Things might not work properly."
        );
            eprintln!("If you're stuck, press 'q' or 'Ctrl-c' to quit the program.");
            stderr().flush().unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn config_exists_spec() {}
}
