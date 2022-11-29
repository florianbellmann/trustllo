use anyhow::Result;
use std::io::{stdin, stdout, Stdout, Write};

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io;
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

use super::layout::{
    get_board_header, get_card_detail, get_card_detail_checklist, get_card_detail_description,
    get_card_detail_layout, get_card_detail_title, get_left_pane_layout, get_list, get_lists,
    get_lists_layout, get_main_layout, get_main_window, get_right_pane_layout,
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
        self.terminal.draw(|f| Self::build_ui(f))?;
        Ok(())
    }

    pub fn render(
        board_name: &str,
        current_board_index: usize,
        current_lists: Vec<&str>,
        current_list_index: usize,
        current_cards: Vec<&str>,
        current_card_index: usize,
    ) {
        ...
        here i shuold build the ui and render it 
        only use the info that you need right now
        best case also as a reference
    }

    fn build_ui<B: Backend>(f: &mut Frame<B>) {
        let size = f.size();

        // INFO: removed main window for now
        // let main_window = get_main_window();
        // f.render_widget(main_window, size);

        let main_layout = get_main_layout().split(size);
        let left_pane_layout = get_left_pane_layout().split(main_layout[0]);
        let right_pane_layout = get_right_pane_layout().split(main_layout[1]);

        let board_header = get_board_header();
        f.render_widget(board_header, left_pane_layout[0]);

        let card_detail = get_card_detail();
        let card_detail_layout = get_card_detail_layout().split(left_pane_layout[1]);
        let card_detail_title = get_card_detail_title();
        let card_detail_description = get_card_detail_description();
        let card_detail_checklist = get_card_detail_checklist();
        f.render_widget(card_detail, left_pane_layout[1]);
        f.render_widget(card_detail_title, card_detail_layout[0]);
        f.render_widget(card_detail_description, card_detail_layout[1]);
        f.render_widget(card_detail_checklist, card_detail_layout[2]);

        let lists = get_lists();
        let lists_layout = get_lists_layout().split(right_pane_layout[0]);
        let list1 = get_list();
        let list2 = get_list();
        let list3 = get_list();
        f.render_widget(lists, main_layout[1]);
        f.render_widget(list1, lists_layout[0]);
        f.render_widget(list2, lists_layout[1]);
        f.render_widget(list3, lists_layout[2]);
    }

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
    // TODO: what can I actually test in this file?
    fn config_exists_spec() {}
}
