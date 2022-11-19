// TODO: does this need to be in a separate folder? can it be just store.ts on top level?

use std::fs::{self, File};
use std::io::Read;

use anyhow::Result;
use log::info;

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

    pub async fn init_from_cache(&mut self, custom_path: Option<&str>) -> Result<()> {
        let data_path = custom_path.unwrap_or(Store::DATA_PATH);
        let store_data = self.read_data_from_file(Some(data_path))?;

        self.current_board = store_data.boards.first().cloned();
        self.current_lists = Some(store_data.lists.clone());
        self.current_list = store_data.lists.first().cloned();

        info!("Initialized store from cache.");
        Ok(())
    }

    pub async fn nuke_all(&mut self, custom_path: Option<&str>) -> Result<()> {
        let _data_path = custom_path.unwrap_or(Store::DATA_PATH);
        self.remove_data_file(custom_path).await?;

        self.current_board = None;
        self.current_lists = None;
        self.current_list = None;
        self.current_cards = None;
        self.current_card = None;
        self.last_card = None;

        info!("Nuked full store.");
        Ok(())
    }

    // boards
    // ----------------------------------------------------------------------------------------------------------------
    pub async fn set_boards(&self, _board: Vec<Board>) -> Result<()> {
        todo!("store boards to file, meaning replace for vec. update both file and memory");
        todo!("update date updated field in file");
    }
    pub fn set_current_board(&mut self, board: &Board) {
        //TODO: when multi board support, change this to index based setter
        self.current_board = Some(board.clone());
    }

    // lists
    // ----------------------------------------------------------------------------------------------------------------
    pub async fn set_current_lists(&self, _lists: &Vec<List>) -> Result<()> {
        todo!("update both file and memory");
        todo!("update date updated field in file");
        Ok(())
    }

    pub fn set_current_list(&mut self, index: usize) {
        self.current_list = Some(self.current_lists.unwrap()[index]);
    }

    // cards
    // ----------------------------------------------------------------------------------------------------------------
    pub fn set_current_cards(&mut self, cards: &Vec<Card>) {
        self.current_cards = Some(cards.clone());
    }
    pub fn set_current_card(&mut self, card: &Card) {
        self.current_card = Some(card.clone());
    }
    pub fn set_last_card(&mut self, card: &Card) {
        self.last_card = Some(card.clone());
    }

    // file system
    // ----------------------------------------------------------------------------------------------------------------
    fn read_data_from_file(&self, custom_path: Option<&str>) -> Result<StoreData> {
        let data_path = custom_path.unwrap_or(Store::DATA_PATH);
        let mut file = File::open(data_path).unwrap();
        let mut file_contents = String::new();
        file.read_to_string(&mut file_contents)?;

        let store_data: StoreData = serde_json::from_str(&file_contents)?;
        Ok(store_data)
    }

    fn create_empty_store(&self, custom_path: Option<&str>) -> Result<()> {
        let data_path = custom_path.unwrap_or(Store::DATA_PATH);
        let empty_store_data: StoreData = StoreData {
            updated: "missing date".to_string(), //TODO: not implemented yet
            boards: vec![],
            lists: vec![],
        };
        let empty_store_data_string = serde_json::to_string(&empty_store_data).unwrap();

        info!(
            "New StoreData with contents {} created.",
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
        info!("Updating store file.");
        Ok(fs::write(data_path, new_store_data_string)?)
    }

    async fn remove_data_file(&self, custom_path: Option<&str>) -> Result<()> {
        let data_path = custom_path.unwrap_or(Store::DATA_PATH);
        info!("Removing store file.");
        fs::remove_file(data_path);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::{
        fs::{self, File},
        io::Read,
        path::Path,
    };

    use anyhow::Result;

    use crate::{
        store::StoreData,
        trello::{Board, Card, List},
        utils::fake_data::FakeData,
    };

    use super::Store;

    #[tokio::test]
    async fn init_from_cache_spec() -> Result<()> {
        let mut store = Store::new();

        // TODO: updated field is missing
        assert_eq!(store.current_board.is_none(), true);
        assert_eq!(store.current_lists.is_none(), true);
        assert_eq!(store.current_list.is_none(), true);
        assert_eq!(store.current_cards.is_none(), true);
        assert_eq!(store.current_card.is_none(), true);
        assert_eq!(store.last_card.is_none(), true);

        let init_cache_data_store_path = "/tmp/trustllo_init_cache_data_store_path.json";
        let fake_store_data = FakeData::get_fake_store_data();
        let fake_store_data_string = serde_json::to_string(&fake_store_data).unwrap();
        fs::write(init_cache_data_store_path, fake_store_data_string);

        store.init_from_cache(Some(init_cache_data_store_path));

        // TODO: multiple board support still missing
        assert_eq!(store.current_board.is_none(), false);
        assert_eq!(store.current_lists.is_none(), false);
        assert_eq!(store.current_list.is_none(), false);
        assert_eq!(
            store.current_list.unwrap().id,
            fake_store_data.lists.first().unwrap().id
        );
        assert_eq!(
            store.current_list.unwrap().name,
            fake_store_data.lists.first().unwrap().name
        );
        assert_eq!(store.current_cards.is_none(), true);
        assert_eq!(store.current_card.is_none(), true);
        assert_eq!(store.last_card.is_none(), true);

        fs::remove_file(init_cache_data_store_path);
        assert_eq!(false, Path::new(init_cache_data_store_path).is_file());
        Ok(())
    }

    #[tokio::test]
    async fn nuke_all_spec() -> Result<()> {
        let store = Store::new();
        let nuke_store_path = "/tmp/trustllo_init_cache_data_store_path.json";
        store.create_empty_store(Some(nuke_store_path));

        assert_eq!(true, Path::new(nuke_store_path).is_file());
        let mut file = File::open(nuke_store_path).unwrap();
        let mut file_contents = String::new();
        file.read_to_string(&mut file_contents)?;
        let store_data: StoreData = serde_json::from_str(&file_contents)?;
        assert_eq!(store_data.boards.len(), 0);
        assert_eq!(store_data.lists.len(), 0);

        store.nuke_all(Some(nuke_store_path));
        assert_eq!(false, Path::new(nuke_store_path).is_file());
        assert!(store.current_board.is_none());
        assert!(store.current_lists.is_none());
        assert!(store.current_list.is_none());
        assert!(store.current_cards.is_none());
        assert!(store.current_card.is_none());
        assert!(store.last_card.is_none());

        Ok(())
    }

    #[tokio::test]
    async fn set_boards_spec() -> Result<()> {
        //TODO: implement with multiple board support
        Ok(())
    }

    #[test]
    fn set_current_board_spec() {
        let store = Store::new();
        assert!(store.current_board.is_none());

        let board1: Board = FakeData::get_fake_board();
        store.set_current_board(&board1);

        assert_eq!(store.current_board.unwrap().id, board1.id);
        assert_eq!(store.current_board.unwrap().name, board1.name);

        let board2: Board = FakeData::get_fake_board();
        store.set_current_board(&board2);

        assert_eq!(store.current_card.unwrap().id, board2.id);
        assert_eq!(store.current_card.unwrap().name, board2.name);
    }

    #[tokio::test]
    async fn set_current_lists_spec() -> Result<()> {
        let store = Store::new();

        let set_current_lists_store_path = "/tmp/trustllo_empty_data_store_path.json";
        store.create_empty_store(Some(set_current_lists_store_path));

        assert_eq!(true, Path::new(set_current_lists_store_path).is_file());

        let mut file = File::open(set_current_lists_store_path).unwrap();
        let mut file_contents = String::new();
        file.read_to_string(&mut file_contents)?;
        let store_data: StoreData = serde_json::from_str(&file_contents)?;
        assert_eq!(store_data.lists.len(), 0);

        assert!(store.current_lists.is_none());
        assert!(store.current_list.is_none());

        let list1: List = FakeData::get_fake_list();
        let list2: List = FakeData::get_fake_list();
        let list3: List = FakeData::get_fake_list();
        let list4: List = FakeData::get_fake_list();

        let lists = vec![list1, list2, list3, list4];
        store.set_current_lists(&lists);

        assert_eq!(store.current_lists.unwrap()[0].id, list1.id);
        assert_eq!(store.current_lists.unwrap()[0].name, list1.name);
        assert_eq!(store.current_lists.unwrap()[1].id, list2.id);
        assert_eq!(store.current_lists.unwrap()[1].name, list2.name);
        assert_eq!(store.current_lists.unwrap()[2].id, list3.id);
        assert_eq!(store.current_lists.unwrap()[2].name, list3.name);
        assert_eq!(store.current_lists.unwrap()[3].id, list4.id);
        assert_eq!(store.current_lists.unwrap()[3].name, list4.name);
        assert_eq!(store.current_lists.unwrap().len(), 4);

        let mut file = File::open(set_current_lists_store_path).unwrap();
        let mut file_contents = String::new();
        file.read_to_string(&mut file_contents)?;
        let store_data: StoreData = serde_json::from_str(&file_contents)?;
        assert_eq!(store_data.lists[0].id, list1.id);
        assert_eq!(store_data.lists[0].name, list1.name);
        assert_eq!(store_data.lists[1].id, list2.id);
        assert_eq!(store_data.lists[1].name, list2.name);
        assert_eq!(store_data.lists[2].id, list3.id);
        assert_eq!(store_data.lists[2].name, list3.name);
        assert_eq!(store_data.lists[3].id, list4.id);
        assert_eq!(store_data.lists[3].name, list4.name);
        assert_eq!(store_data.lists.len(), 4);

        let list5: List = FakeData::get_fake_list();
        let list6: List = FakeData::get_fake_list();
        let list7: List = FakeData::get_fake_list();

        let lists = vec![list5, list6, list7];
        store.set_current_lists(&lists);

        assert_eq!(store.current_lists.unwrap()[0].id, list5.id);
        assert_eq!(store.current_lists.unwrap()[0].name, list5.name);
        assert_eq!(store.current_lists.unwrap()[1].id, list6.id);
        assert_eq!(store.current_lists.unwrap()[1].name, list6.name);
        assert_eq!(store.current_lists.unwrap()[2].id, list7.id);
        assert_eq!(store.current_lists.unwrap()[2].name, list7.name);
        assert_eq!(store.current_lists.unwrap().len(), 3);

        let mut file = File::open(set_current_lists_store_path).unwrap();
        let mut file_contents = String::new();
        file.read_to_string(&mut file_contents)?;
        let store_data: StoreData = serde_json::from_str(&file_contents)?;
        assert_eq!(store_data.lists[0].id, list1.id);
        assert_eq!(store_data.lists[0].name, list1.name);
        assert_eq!(store_data.lists[1].id, list2.id);
        assert_eq!(store_data.lists[1].name, list2.name);
        assert_eq!(store_data.lists[2].id, list3.id);
        assert_eq!(store_data.lists[2].name, list3.name);
        assert_eq!(store_data.lists.len(), 3);

        fs::remove_file(set_current_lists_store_path);
        assert_eq!(false, Path::new(set_current_lists_store_path).is_file());
        Ok(())
    }

    #[tokio::test]
    async fn set_current_list_spec() {
        let store = Store::new();
        assert!(store.current_list.is_none());

        let list: List = FakeData::get_fake_list();
        store.set_current_list(&list);

        assert_eq!(store.current_list.unwrap().id, list.id);
        assert_eq!(store.current_list.unwrap().name, list.name);

        let list2: List = FakeData::get_fake_list();
        store.set_current_list(&list2);

        assert_eq!(store.current_list.unwrap().id, list2.id);
        assert_eq!(store.current_list.unwrap().name, list2.name);
    }

    #[test]
    fn set_current_cards_spec() {
        let store = Store::new();
        assert!(store.current_cards.is_none());
        assert!(store.current_card.is_none());

        let card1: Card = FakeData::get_fake_card();
        let card2: Card = FakeData::get_fake_card();
        let card3: Card = FakeData::get_fake_card();
        let card4: Card = FakeData::get_fake_card();

        let cards = vec![card1, card2, card3, card4];

        store.set_current_cards(&cards);

        assert_eq!(store.current_cards.unwrap()[0].id, card1.id);
        assert_eq!(store.current_cards.unwrap()[0].name, card1.name);
        assert_eq!(store.current_cards.unwrap()[1].id, card2.id);
        assert_eq!(store.current_cards.unwrap()[1].name, card2.name);
        assert_eq!(store.current_cards.unwrap()[2].id, card3.id);
        assert_eq!(store.current_cards.unwrap()[2].name, card3.name);
        assert_eq!(store.current_cards.unwrap()[3].id, card4.id);
        assert_eq!(store.current_cards.unwrap()[3].name, card4.name);
        assert_eq!(store.current_cards.unwrap().len(), 4);

        let card5: Card = FakeData::get_fake_card();
        let card6: Card = FakeData::get_fake_card();
        let card7: Card = FakeData::get_fake_card();

        let cards = vec![card5, card6, card7];

        store.set_current_cards(&cards);

        assert_eq!(store.current_cards.unwrap()[0].id, card5.id);
        assert_eq!(store.current_cards.unwrap()[0].name, card5.name);
        assert_eq!(store.current_cards.unwrap()[1].id, card6.id);
        assert_eq!(store.current_cards.unwrap()[1].name, card6.name);
        assert_eq!(store.current_cards.unwrap()[2].id, card7.id);
        assert_eq!(store.current_cards.unwrap()[2].name, card7.name);
        assert_eq!(store.current_cards.unwrap().len(), 3)
    }

    #[test]
    fn set_current_card_spec() {
        let store = Store::new();
        assert!(store.current_card.is_none());

        let card: Card = FakeData::get_fake_card();
        store.set_current_card(&card);

        assert_eq!(store.current_card.unwrap().id, card.id);
        assert_eq!(store.current_card.unwrap().name, card.name);

        let card2: Card = FakeData::get_fake_card();
        store.set_current_card(&card2);

        assert_eq!(store.current_card.unwrap().id, card2.id);
        assert_eq!(store.current_card.unwrap().name, card2.name);
    }

    #[test]
    fn set_last_card_spec() {
        let store = Store::new();
        assert!(store.last_card.is_none());

        let card: Card = FakeData::get_fake_card();
        store.set_last_card(&card);

        assert_eq!(store.last_card.unwrap().id, card.id);
        assert_eq!(store.last_card.unwrap().name, card.name);

        let card2: Card = FakeData::get_fake_card();
        store.set_current_card(&card2);

        assert_eq!(store.last_card.unwrap().id, card2.id);
        assert_eq!(store.last_card.unwrap().name, card2.name);
    }

    #[tokio::test]
    async fn read_data_from_file_spec() -> Result<()> {
        let store = Store::new();

        let read_data_store_path = "/tmp/trustllo_read_data_store_path.json";
        let fake_store_data = FakeData::get_fake_store_data();
        let fake_store_data_string = serde_json::to_string(&fake_store_data).unwrap();
        fs::write(read_data_store_path, fake_store_data_string);
        assert_eq!(true, Path::new(read_data_store_path).is_file());

        let store_data: StoreData = store.read_data_from_file(Some(read_data_store_path))?;

        assert_eq!(store_data.boards.len(), fake_store_data.boards.len());
        assert_eq!(
            store_data.boards.first().unwrap().id,
            fake_store_data.boards.first().unwrap().id
        );
        assert_eq!(store_data.lists.len(), fake_store_data.lists.len());
        assert_eq!(
            store_data.lists.first().unwrap().id,
            fake_store_data.lists.first().unwrap().id
        );
        assert_eq!(store_data.updated, fake_store_data.updated);

        fs::remove_file(read_data_store_path);
        assert_eq!(false, Path::new(read_data_store_path).is_file());
        todo!("create fake data and store in file. then read the data. full file and single properties");
        Ok(())
    }

    #[test]
    fn create_empty_store_spec() -> Result<()> {
        let store = Store::new();

        let empty_data_store_path = "/tmp/trustllo_empty_data_store_path.json";
        store.create_empty_store(Some(empty_data_store_path));

        assert_eq!(true, Path::new(empty_data_store_path).is_file());

        let mut file = File::open(empty_data_store_path).unwrap();
        let mut file_contents = String::new();
        file.read_to_string(&mut file_contents)?;

        let store_data: StoreData = serde_json::from_str(&file_contents)?;
        assert_eq!(store_data.updated, "missing date");
        assert_eq!(store_data.boards.len(), 0);
        assert_eq!(store_data.lists.len(), 0);

        fs::remove_file(empty_data_store_path);
        assert_eq!(false, Path::new(empty_data_store_path).is_file());

        Ok(())
    }

    #[tokio::test]
    async fn write_data_to_file_spec() -> Result<()> {
        let store = Store::new();

        let write_data_store_path = "/tmp/trustllo_write_data_store_path.json";
        let fake_store_data = FakeData::get_fake_store_data();
        let fake_store_data_string = serde_json::to_string(&fake_store_data).unwrap();

        assert_eq!(false, Path::new(write_data_store_path).is_file());

        fs::write(write_data_store_path, fake_store_data_string);

        assert_eq!(true, Path::new(write_data_store_path).is_file());

        todo!("time for existing. also check for full file and single sub properties. also last updated");
        todo!("check different scenarios of subdata");

        fs::remove_file(write_data_store_path);
        assert_eq!(false, Path::new(write_data_store_path).is_file());
        Ok(())
    }

    #[tokio::test]
    async fn remove_data_file_spec() -> Result<()> {
        let store = Store::new();

        let remove_data_store_path = "/tmp/trustllo_remove_data_store_path.json";
        let fake_store_data = FakeData::get_fake_store_data();
        let fake_store_data_string = serde_json::to_string(&fake_store_data).unwrap();
        fs::write(remove_data_store_path, fake_store_data_string);

        assert_eq!(true, Path::new(remove_data_store_path).is_file());

        store.remove_data_file(Some(remove_data_store_path));
        assert_eq!(false, Path::new(remove_data_store_path).is_file());

        // do it twice so we see it doesn't panic
        store.remove_data_file(Some(remove_data_store_path));
        assert_eq!(false, Path::new(remove_data_store_path).is_file());

        Ok(())
    }
}
