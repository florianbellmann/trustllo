use anyhow::Result;
use crossterm::event::{self, Event, KeyCode};
use log::{debug};

use crate::{
    config::config_manager::ConfigManager, store::data_provider::DataProvider, ui::cli::Cli,
};

pub struct ApplicationService {
    data_provider: DataProvider,
}

impl ApplicationService {
    pub fn new() -> ApplicationService {
        ApplicationService {
            data_provider: DataProvider::new(),
        }
    }

    pub async fn init(&mut self) -> Result<()> {
        debug!("Initializing app.");

        let cli = Cli::new(); // TODO: remove this cli instance

        // TODO add functionality for custom config
        if !ConfigManager::config_exists(None) {
            let (key, token, member_id) = cli.read_config_from_user_input();
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

        // display data
        //

        // init keyboard listener
        Ok(())
    }

    pub fn teardown(&self) {
        debug!("Tearing down app.");
        // cli restore needed
    }

    pub async fn run_app_loop(&mut self) -> Result<()> {
        debug!("Starting app loop.");
        // TODO: actually build the app loop
        let _cli = Cli::new(); // TODO: remove this cli instance

        loop {
            // get data
            let _current_board = self.data_provider.get_current_board().await;
            let _current_lists = self.data_provider.get_current_lists().await;
            let _current_list_index = self.data_provider.get_current_list_index().await;
            let _current_cards = self.data_provider.get_current_cards().await;
            let _current_card_index = self.data_provider.get_current_card_index().await;
            // // cli.draw();
            // cli.render(
            //     current_board.name,
            //     current_lists.into_iter().map(|l| l.name).collect(),
            //     current_list_index,
            //     current_cards.into_iter().map(|c| c.name).collect(),
            //     current_card_index,
            // );

            if let Event::Key(key) = event::read()? {
                if let KeyCode::Char('q') = key.code {
                    return Ok(());
                }
            }
        }
        // pub fn run_app<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
        //     loop {
        //     }
        // }
        //
        // todo!()
    }

    async fn refresh_boards_and_lists(&self) -> Result<()> {
        // let boards = self.api_connector.get_boards().await?;
        // let _board = boards.first().unwrap();

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
