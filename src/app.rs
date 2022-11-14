use anyhow::Result;
use log::{error, info, debug};
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

use crate::{
    config::config_manager::ConfigManager, store::store::Store,
    trello::api_connector::ApiConnector, ui::cli::Cli,
};

pub struct ApplicationService {
    api_connector: ApiConnector,
    store: Store,
}

impl ApplicationService {
    pub fn new() -> ApplicationService {
        ApplicationService {
            api_connector: ApiConnector::new(),
            store: Store::new(),
        }
    }

    pub async fn init(&self) -> Result<()> {
        debug!("Initializing app.");

        // TODO add functionality for custom config
        if !ConfigManager::config_exists(None) {
            let (key, token, member_id) = Cli::read_config();
            ConfigManager::create_config(key, token, member_id, None);
        }

        // TODO: Do I really want init functions everywhere or do I use the new function because I instantiate everything anyway?
        // for now yes, because I need to split new store from init
        // match self.store.init_from_cache(None).await {
        //     Ok(x) => {}
        //     Err(e) => self.refresh_boards_and_lists().await?,
        // }

        // maybe better:
        //         if let list: &List = {
        //         self.store.init_from_cache().await ?;
        // &self.store.current_list.unwrap();
        //         }
        //         else{
        //             self.refresh_boards_and_lists().await?
        // &self.store.current_list.unwrap();
        //         }

        //
        //
        //if let current_list = option {
        //    self.store.init_from_cache().await?;

        //}

        //if self.store.current_board != nil

        ////TODO: load data async if cache present to get better startup
        //self.api_connector.init().await;

        //load data

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
        Ok(())
    }

    pub fn teardown(&self) {
        debug!("Tearing down app.");
    }

    pub fn run_app_loop(&self) {
        debug!("Starting app loop.");
        // TODO: actually build the app loop

        // todo!()
    }

    async fn refresh_boards_and_lists(&self) -> Result<()> {
        let boards = self.api_connector.get_boards().await?;
        let _board = boards.first().unwrap();

        //         self.store.set_current_board(&board);
        //         let lists = self.api_connector.get_lists_on_board(&board.id).await?;
        //         self.store.set_current_lists(&lists);
        //         let list = lists.first().unwrap();
        // self.store.set_current_list(&list);
        Ok(())
    }
}

// TODO:
// think about if you can actually test stuff here. It's the
// app loop after all. Is there something to test?
