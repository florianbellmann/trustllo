use std::io;

use crossterm::event::Event;
use crossterm::event::{ self, KeyCode };
use tui::backend::Backend;
use tui::Terminal;

use crate::ui;

pub fn run_app<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    loop {
        terminal.draw(ui::draw_interface)?;

        if let Event::Key(key) = event::read()? {
            if let KeyCode::Char('q') = key.code {
                return Ok(());
            }
        }
    }
}

// TODO: Test if q exits the app