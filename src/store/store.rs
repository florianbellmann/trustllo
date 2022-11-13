// TODO: does this need to be in a separate folder? can it be just store.ts on top level?

use std::error::Error;
use std::fs::{self, File};
use std::io::Read;
use std::path::Path;

use anyhow::{anyhow, Result};

use serde::Deserialize;

use crate::{
    store::StoreData,
    trello::{Board, Card, List},
};

// TODO: unclear if I want to store data next to config?
// use crate::config::config_manager::ConfigManager;

pub struct Store {
    pub current_board: Option<Board>,
    pub current_lists: Option<Vec<List>>,
    pub current_list: Option<List>,
    pub current_cards: Option<Vec<Card>>,
    pub current_card: Option<Card>,
    pub last_card: Option<Card>,
}

impl Store {
    const DATA_PATH: &str = "data.json";

    // TODO: Make sure this is used as a singleton!
    pub fn new() -> Store {
        Store {
            // TODO: missing is the multiple board support
            current_board: None,
            current_lists: None,
            current_list: None,
            current_cards: None,
            current_card: None,
            last_card: None,
        }
    }

    pub async fn init_from_cache(&self) -> Result<()> {
        todo!("read data from files and init board + lists. panic if fails");
    }

    pub async fn nuke_all(&self) -> Result<()> {
        todo!("delete database file, flush memory");
    }

    // boards
    // ----------------------------------------------------------------------------------------------------------------
    pub fn set_boards(&self, board: Vec<Board>) -> Result<()> {
        todo!("store boards to file, meaning replace for vec");
        todo!("update both file and memory");
    }
    pub fn set_current_board(&self, boards: &Board) -> Result<()> {
        todo!("store board to file");
        todo!("update both file and memory");
    }

    // lists
    // ----------------------------------------------------------------------------------------------------------------
    pub async fn set_current_lists(&self, lists: &Vec<List>) -> Result<()> {
        todo!("store lists to file, meaning replace for vec");
        todo!("update both file and memory");
    }

    pub async fn set_current_list(&self, list: &List) -> Result<()> {
        todo!("update list in file");
        todo!("update both file and memory");
    }

    // cards
    // ----------------------------------------------------------------------------------------------------------------
    pub async fn set_current_cards(&self, cards: &Vec<Card>) -> Result<()> {
        todo!("store cards in memory, meaning replace for vec");
        todo!("update memory");
    }
    pub async fn set_current_card(&self, card: &Card) -> Result<()> {
        todo!("store card in memory");
        todo!("update memory");
    }
    pub async fn set_last_card(&self, card: &Card) -> Result<()> {
        todo!("update card in memory");
        todo!("update memory");
    }

    // file system
    // ----------------------------------------------------------------------------------------------------------------
    async fn read_data_from_file(&self, custom_path: Option<&str>) -> Result<StoreData> {
        let data_path = custom_path.unwrap_or(Store::DATA_PATH);
        let mut file = File::open(data_path).unwrap();
        let mut file_contents = String::new();
        file.read_to_string(&mut file_contents)?;
        let store_data: StoreData = serde_json::from_str(&file_contents)?;
        Ok(store_data)
    }

    async fn create_empty_store(&self, custom_path: Option<&str>) -> Result<()> {
        let data_path = custom_path.unwrap_or(Store::DATA_PATH);
        let empty_store_data: StoreData = StoreData {
            updated: "missing date".to_string(), //TODO: not implemented yet
            boards: vec![],
            lists: vec![],
        };
        let empty_store_data_string = serde_json::to_string(&empty_store_data).unwrap();

        println!(
            "StoreData with contents {} created.",
            &empty_store_data_string
        );
        Ok(fs::write(data_path, empty_store_data_string)?)
    }

    async fn write_data_to_file(
        &self,
        new_store_data: StoreData,
        custom_path: Option<&str>,
    ) -> Result<()> {
        let data_path = custom_path.unwrap_or(Store::DATA_PATH);
        let new_store_data_string = serde_json::to_string(&new_store_data).unwrap();
        Ok(fs::write(data_path, new_store_data_string)?)
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    use crate::{store::StoreData, trello::Board, utils::fake_data::get_fake_board};

    #[tokio::test]
    async fn init_from_cache_spec() -> Result<()> {
        todo!("init from cache and panic if impossible");

        Ok(())
    }
    #[tokio::test]
    async fn nuke_all_spec() -> Result<()> {
        todo!("create fake data. init store, check some data from the file, nuke, make sure it's not there anymore");

        Ok(())
    }
    #[tokio::test]
    async fn store_board_spec() -> Result<()> {
        todo!("create fake data. check board not there. create board. store it. check its there. do the same test with an empty database");
        Ok(())
    }
    #[tokio::test]
    async fn store_boards_spec() -> Result<()> {
        todo!("create fake data. check boards not there. create boards. store them (maybe 3). check they are there. do the same test with an empty database");
        Ok(())
    }
    #[tokio::test]
    async fn update_board_spec() -> Result<()> {
        todo!("create fake data. check a board. update it. check new fields are there. make the test also with the board not there yet. meaning in this case it must insert!");
        Ok(())
    }
    #[tokio::test]
    async fn store_list_spec() -> Result<()> {
        todo!("create fake data. check list not there. create list. store it. check its there. do the same test with an empty database");
        Ok(())
    }
    #[tokio::test]
    async fn store_lists_spec() -> Result<()> {
        todo!("create fake data. check lists not there. create lists. store them (maybe 3). check they are there. do the same test with an empty database");
        Ok(())
    }
    #[tokio::test]
    async fn update_list_spec() -> Result<()> {
        todo!("create fake data. check a list. update it. check new fields are there. make the test also with the list not there yet. meaning in this case it must insert!");
        Ok(())
    }
    #[tokio::test]
    async fn store_card_spec() -> Result<()> {
        todo!("create fake data. check card not there. create card. store it. check its there. do the same test with an empty database");
        Ok(())
    }
    #[tokio::test]
    async fn store_cards_spec() -> Result<()> {
        todo!("create fake data. check cards not there. create cards. store them (maybe 3). check they are there. do the same test with an empty database");
        Ok(())
    }
    #[tokio::test]
    async fn update_card_spec() -> Result<()> {
        todo!("create fake data. check a card. update it. check new fields are there. make the test also with the card not there yet. meaning in this case it must insert!");
        Ok(())
    }
    #[tokio::test]
    async fn read_data_from_file_spec() -> Result<()> {
        let read_data_store_path = "/tmp/trustllo_read_data_store_path.json";
        todo!("create fake data and store in file. then read the data. full file and single properties");
        Ok(())
    }
    #[tokio::test]
    async fn create_empty_store_spec() -> Result<()> {
        let empty_data_store_path = "/tmp/trustllo_empty_data_store_path.json";
        todo!("create empty store. check values");
        Ok(())
    }
    #[tokio::test]
    async fn write_data_to_file_spec() -> Result<()> {
        let write_data_store_path = "/tmp/trustllo_write_data_store_path.json";
        todo!("create fake data. write it to file. check if it's there. one time for non-existing file and one time for existing. also check for full file and single sub properties. also last updated");
        todo!("check different scenarios of subdata");
        Ok(())
    }

    fn create_fake_date() {
        let boards = vec![Board {
            name: todo!(),
            desc: todo!(),
            closed: todo!(),
            id: todo!(),
            url: todo!(),
            subscribed: todo!(),
        },Board{ name: todo!(), desc: todo!(), closed: todo!(), id: todo!(), url: todo!(), subscribed: todo!() }];


        let a =get_fake_board();
        // let listsNB

        StoreData {
            updated: "missing date", //TODO: not implemented yet
            boards,
            lists: todo!(),
        }
    }
}
