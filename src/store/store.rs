// TODO: does this need to be in a separate folder? can it be just store.ts on top level?

use anyhow::{anyhow, Result};

use serde::Deserialize;

use crate::trello::{Board, Card, List};

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
    async fn read_data_from_file(&self) -> Result<()> {
        todo!("load from file system or open new file");
    }
    async fn write_data_to_file(&self) -> Result<()> {
        // only full file? or also subdata allowed? should be the generic function used by all
        // other ones
        todo!("write data to preexistsing file or create a new one");
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

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
        todo!("create fake data and store in file. then read the data. full file and single properties");
        Ok(())
    }
    #[tokio::test]
    async fn write_data_to_file_spec() -> Result<()> {
        todo!("create fake data. write it to file. check if it's there. one time for non-existing file and one time for existing. also check for full file and single sub properties. also last updated");
        Ok(())
    }

    fn create_fake_date() {
        todo!("Setup fake data base in file")
        // https://github.com/cksac/fake-rs
    }
}
