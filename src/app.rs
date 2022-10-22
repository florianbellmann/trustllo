// use crossterm::{
//     event::{DisableMouseCapture, EnableMouseCapture},
//     execute,
//     terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
// };
// use std::{error::Error, io};
// use tui::{backend::CrosstermBackend, Terminal};

// use std::{
//     boxed::Box,
//     io::{stderr, stdout, Write},
//     thread,
//     time::Duration,
// };

// mod app;
// mod ui;
//
// use std::io;

// use crossterm::event::Event;
// use crossterm::event::{ self, KeyCode };
// use tui::backend::Backend;
// use tui::Terminal;

// // use crate::ui;

// pub fn run_app<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
//     loop {
//         terminal.draw(ui::draw_interface)?;

//         if let Event::Key(key) = event::read()? {
//             if let KeyCode::Char('q') = key.code {
//                 return Ok(());
//             }
//         }
//     }
// }

// // TODO: Test if q exits the app
//
//

use crate::trello::api_connector::ApiConnector;

pub struct ApplicationService {
    api_connector: ApiConnector,
}

impl ApplicationService {
    pub fn new() -> ApplicationService {
        ApplicationService {
            api_connector: ApiConnector::new(),
        }
    }

    pub async fn init(&mut self) {
        println!("Initializing the app...");

        //TODO: load data async to get better startup
        //
        // Load data from trello
        //
        //
        //
        let a = ApiConnector::new();

        
        self.api_connector.loadall();

        // optional: store/cache

        // init terminal
        // fn main() -> Result<(), Box<dyn Error>> {
        // // Check if the current environment is in a terminal.
        // check_if_terminal();

        // // setup terminal
        // enable_raw_mode()?; // send data byte by byte to terminal
        // let mut stdout = io::stdout();
        // execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        // let backend = CrosstermBackend::new(stdout);
        // let mut terminal = Terminal::new(backend)?;

        // // create app and run it
        // let res = app::run_app(&mut terminal);

        // // restore terminal
        // disable_raw_mode()?;
        // execute!(
        //     terminal.backend_mut(),
        //     LeaveAlternateScreen,
        //     DisableMouseCapture
        // )?;
        // terminal.show_cursor()?;

        // // Error handling
        // if let Err(err) = res {
        //     println!("{:?}", err);
        // }

        // Ok(())

        // display data

        // init keyboard listener
    }

    pub fn teardown() {
        println!("Tearing down the app...");
    }

    pub fn run_app_loop() {
        // TODO: actually build the app loop

        // todo!()
    }
}

// TODO:
// think about if you can actually test stuff here. It's the
// app loop after all. Is there something to test?
