use anyhow::Result;
use log::debug;

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
            store: Store::new(None),
        }
    }

    pub async fn init(&self) -> Result<()> {
        debug!("Initializing app.");

        // TODO add functionality for custom config
        if !ConfigManager::config_exists(None) {
            let (key, token, member_id) = Cli::read_config_from_user_input();
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

        // init keyboard listener
        Ok(())
    }

    pub fn teardown(&self) {
        debug!("Tearing down app.");
    }

    pub fn run_app_loop(&self) {
        debug!("Starting app loop.");
        // TODO: actually build the app loop

// pub fn run_app<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
//     loop {
//     }
// }
//
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
