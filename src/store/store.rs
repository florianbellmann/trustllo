// TODO: does this need to be in a separate folder? can it be just store.ts on top level?

use std::fs::{self, File};
use std::io::Read;

use anyhow::Result;
use log::{error, info};

use crate::{
    store::StoreData,
    trello::{Board, Card, List},
};

// TODO: unclear if I want to store data next to config?
// use crate::config::config_manager::ConfigManager;

pub struct Store {
    pub boards: Vec<Board>, // base data from file
    pub current_board_index: usize,
    pub current_lists: Vec<List>, // base data from file
    pub current_list_index: usize,
    pub current_cards: Option<Vec<Card>>, // base data that was fetched
    pub current_card_index: Option<usize>,
    pub last_card: Option<Card>, // can't be a reference, because it might be on the old list
    data_path: String,
}

impl Store {
    const DATA_PATH: &str = "data.json";

    // TODO: Make sure this is used as a singleton!
    pub fn new(custom_path: Option<&str>) -> Store {
        let data_path = custom_path.unwrap_or(Store::DATA_PATH);

        let store_data = match read_data_from_file(data_path) {
            Ok(data) => data,
            Err(e) => {
                let error_msg = format!("ERROR: Failed to parse the Api response. {}", e);
                error!("{}", error_msg);
                create_empty_store_file(data_path).unwrap()
            }
        };

        Store {
            // TODO: missing is the multiple board support
            boards: store_data.boards,
            current_board_index: 0,
            current_lists: store_data.lists,
            current_list_index: 0,
            current_cards: None,
            current_card_index: None,
            last_card: None,
            data_path: data_path.to_string(),
        }
    }

    pub fn refresh_from_cache(&mut self) -> Result<()> {
        let store_data = read_data_from_file(&self.data_path)?;

        self.boards = store_data.boards;
        self.current_lists = store_data.lists;
        self.current_board_index = 0;
        self.current_list_index = 0;

        info!("Initialized store from cache.");
        Ok(())
    }

    pub fn nuke_all(&mut self) -> Result<()> {
        remove_data_file(&self.data_path)?;

        self.boards = vec![];
        self.current_board_index = 0;
        self.current_lists = vec![];
        self.current_list_index = 0;
        self.current_cards = None;
        self.current_card_index = None;
        self.last_card = None;

        info!("Nuked full store.");
        Ok(())
    }

    // boards
    // ----------------------------------------------------------------------------------------------------------------
    pub async fn set_boards(&mut self, boards: Vec<Board>) -> Result<()> {
        self.boards = boards.clone();
        self.current_board_index = 0;
        let lists = vec![];
        let store_data = StoreData {
            updated: "updated missing".to_string(),
            boards,
            lists,
        };

        let new_boards_data_store_string = serde_json::to_string(&store_data).unwrap();

        Ok(fs::write(&self.data_path, new_boards_data_store_string)?)
    }
    pub fn get_current_board(&self) -> &Board {
        &self.boards[self.current_board_index]
    }

    // lists
    // ----------------------------------------------------------------------------------------------------------------
    pub async fn set_current_lists(&mut self, lists: Vec<List>) -> Result<()> {
        self.current_lists = lists.clone();
        self.current_list_index = 0;
        let boards = self.boards.clone();
        let store_date = StoreData {
            updated: "updated missing".to_string(),
            boards,
            lists,
        };

        let new_lists_data_store_string = serde_json::to_string(&store_date).unwrap();

        Ok(fs::write(&self.data_path, new_lists_data_store_string)?)
    }

    pub fn set_current_list_index(&mut self, index: usize) {
        self.current_list_index = index;
    }
    pub fn get_current_list(&self) -> &List {
        &self.current_lists[self.current_list_index]
    }

    // cards
    // ----------------------------------------------------------------------------------------------------------------
    pub async fn set_current_cards(&mut self, cards: Vec<Card>) {
        self.current_cards = Some(cards);
        self.current_card_index = Some(0);
    }
    pub fn set_current_card_index(&mut self, index: usize) {
        self.current_card_index = Some(index);
    }
    pub fn get_current_card(&self) -> Option<&Card> {
        if self.current_cards.is_some() && self.current_card_index.is_some() {
            return Some(&self.current_cards.as_ref().unwrap()[self.current_card_index.unwrap()]);
        }
        None
    }
    pub fn set_last_card(&mut self, card: &Card) {
        self.last_card = Some(card.clone());
    }
}

// file system
// ----------------------------------------------------------------------------------------------------------------
fn read_data_from_file(data_path: &str) -> Result<StoreData> {
    let mut file = File::open(data_path)?;
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)?;

    let store_data: StoreData = serde_json::from_str(&file_contents)?;

    Ok(store_data)
}

fn create_empty_store_file(data_path: &str) -> Result<StoreData> {
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
    fs::write(data_path, empty_store_data_string)?;
    Ok(empty_store_data)
}

fn write_data_to_file(data_path: &str, new_store_data: StoreData) -> Result<()> {
    let new_store_data_string = serde_json::to_string(&new_store_data).unwrap();
    info!("Updating store file.");
    Ok(fs::write(data_path, new_store_data_string)?)
}

fn remove_data_file(data_path: &str) -> Result<()> {
    info!("Removing store file.");
    fs::remove_file(data_path)?;
    Ok(())
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
        store::{
            store::{
                create_empty_store_file, read_data_from_file, remove_data_file, write_data_to_file,
            },
            StoreData,
        },
        trello::{Board, Card, List},
        utils::fake_data::FakeData,
    };

    use super::Store;

    #[tokio::test]
    async fn init_from_cache_spec() -> Result<()> {
        let init_cache_data_store_path = "/tmp/trustllo_init_cache_data_store_path.json";
        let mut store = Store::new(Some(init_cache_data_store_path));

        // TODO: updated field is missing
        assert_eq!(store.current_board_index, 0);
        assert_eq!(store.current_lists.len(), 0);
        assert_eq!(store.current_list_index, 0);
        assert_eq!(store.current_cards.is_none(), true);
        assert_eq!(store.current_card_index.is_none(), true);
        assert_eq!(store.last_card.is_none(), true);

        let fake_store_data = FakeData::get_fake_store_data();
        let fake_store_data_string = serde_json::to_string(&fake_store_data).unwrap();
        fs::write(init_cache_data_store_path, fake_store_data_string)?;

        store.refresh_from_cache()?;

        // TODO: multiple board support still missing
        assert_eq!(store.current_board_index, 0);
        assert_eq!(store.current_lists.len(), 4);
        assert_eq!(store.current_list_index, 0);
        assert_eq!(store.current_cards.is_none(), true);
        assert_eq!(store.current_card_index.is_none(), true);
        assert_eq!(store.last_card.is_none(), true);

        fs::remove_file(init_cache_data_store_path)?;
        assert_eq!(false, Path::new(init_cache_data_store_path).is_file());
        Ok(())
    }

    #[tokio::test]
    async fn nuke_all_spec() -> Result<()> {
        let nuke_store_path = "/tmp/trustllo_nuke_data_store_path.json";
        let mut store = Store::new(Some(nuke_store_path));
        create_empty_store_file(nuke_store_path)?;

        assert_eq!(true, Path::new(nuke_store_path).is_file());
        let mut file = File::open(nuke_store_path).unwrap();
        let mut file_contents = String::new();
        file.read_to_string(&mut file_contents)?;
        let store_data: StoreData = serde_json::from_str(&file_contents)?;
        assert_eq!(store_data.boards.len(), 0);
        assert_eq!(store_data.lists.len(), 0);

        store.nuke_all()?;
        assert_eq!(false, Path::new(nuke_store_path).is_file());
        assert_eq!(store.current_board_index, 0);
        assert_eq!(store.current_lists.len(), 0);
        assert_eq!(store.current_list_index, 0);
        assert!(store.current_cards.is_none());
        assert!(store.current_card_index.is_none());
        assert!(store.last_card.is_none());

        Ok(())
    }

    #[tokio::test]
    async fn set_boards_spec() -> Result<()> {
        let set_boards_store_path = "/tmp/trustllo_set_boards_data_store_path.json";
        let mut store = Store::new(Some(set_boards_store_path));

        create_empty_store_file(set_boards_store_path)?;

        assert_eq!(true, Path::new(set_boards_store_path).is_file());

        let mut file = File::open(set_boards_store_path).unwrap();
        let mut file_contents = String::new();
        file.read_to_string(&mut file_contents)?;
        let store_data: StoreData = serde_json::from_str(&file_contents)?;
        assert_eq!(store_data.boards.len(), 0);
        assert_eq!(store.current_board_index, 0);

        let board1: Board = FakeData::get_fake_board();
        let board2: Board = FakeData::get_fake_board();
        let board3: Board = FakeData::get_fake_board();
        let board4: Board = FakeData::get_fake_board();

        let boards = vec![
            board1.clone(),
            board2.clone(),
            board3.clone(),
            board4.clone(),
        ];
        store.set_boards(boards).await?;

        assert_eq!(store.boards[0].id, board1.id);
        assert_eq!(store.boards[0].name, board1.name);
        assert_eq!(store.boards[1].id, board2.id);
        assert_eq!(store.boards[1].name, board2.name);
        assert_eq!(store.boards[2].id, board3.id);
        assert_eq!(store.boards[2].name, board3.name);
        assert_eq!(store.boards[3].id, board4.id);
        assert_eq!(store.boards[3].name, board4.name);
        assert_eq!(store.boards.len(), 4);
        assert_eq!(store.current_board_index, 0);

        let mut file = File::open(set_boards_store_path).unwrap();
        let mut file_contents = String::new();
        file.read_to_string(&mut file_contents)?;
        let store_data_from_file: StoreData = serde_json::from_str(&file_contents)?;
        assert_eq!(store_data_from_file.boards[0].id, board1.id);
        assert_eq!(store_data_from_file.boards[0].name, board1.name);
        assert_eq!(store_data_from_file.boards[1].id, board2.id);
        assert_eq!(store_data_from_file.boards[1].name, board2.name);
        assert_eq!(store_data_from_file.boards[2].id, board3.id);
        assert_eq!(store_data_from_file.boards[2].name, board3.name);
        assert_eq!(store_data_from_file.boards[3].id, board4.id);
        assert_eq!(store_data_from_file.boards[3].name, board4.name);
        assert_eq!(store_data_from_file.boards.len(), 4);
        assert_eq!(store.current_board_index, 0);

        let board5: Board = FakeData::get_fake_board();
        let board6: Board = FakeData::get_fake_board();
        let board7: Board = FakeData::get_fake_board();

        let boards = vec![board5.clone(), board6.clone(), board7.clone()];
        store.set_boards(boards).await?;

        assert_eq!(store.boards[0].id, board5.id);
        assert_eq!(store.boards[0].name, board5.name);
        assert_eq!(store.boards[1].id, board6.id);
        assert_eq!(store.boards[1].name, board6.name);
        assert_eq!(store.boards[2].id, board7.id);
        assert_eq!(store.boards[2].name, board7.name);
        assert_eq!(store.boards.len(), 3);
        assert_eq!(store.current_board_index, 0);

        let mut file = File::open(set_boards_store_path).unwrap();
        let mut file_contents = String::new();
        file.read_to_string(&mut file_contents)?;
        let store_data_from_file2: StoreData = serde_json::from_str(&file_contents)?;
        assert_eq!(store_data_from_file2.boards[0].id, board5.id);
        assert_eq!(store_data_from_file2.boards[0].name, board5.name);
        assert_eq!(store_data_from_file2.boards[1].id, board6.id);
        assert_eq!(store_data_from_file2.boards[1].name, board6.name);
        assert_eq!(store_data_from_file2.boards[2].id, board7.id);
        assert_eq!(store_data_from_file2.boards[2].name, board7.name);
        assert_eq!(store_data_from_file2.boards.len(), 3);
        assert_eq!(store.current_board_index, 0);

        fs::remove_file(set_boards_store_path)?;
        assert_eq!(false, Path::new(set_boards_store_path).is_file());
        Ok(())
    }

    #[test]
    fn set_current_board_index_spec() {
        let set_current_board_store_path = "/tmp/trustllo_set_current_board_data_store_path.json";
        let mut store = Store::new(Some(set_current_board_store_path));
        assert_eq!(store.current_board_index, 0);

        store.current_board_index = 0;
        assert_eq!(store.current_board_index, 0);
        store.current_board_index = 1;
        assert_eq!(store.current_board_index, 1);
        store.current_board_index = 99;
        assert_eq!(store.current_board_index, 99);
        store.current_board_index = usize::MAX;
        assert_eq!(store.current_board_index, usize::MAX);

        fs::remove_file(set_current_board_store_path);
        assert_eq!(false, Path::new(set_current_board_store_path).is_file());
    }

    #[tokio::test]
    async fn set_current_lists_spec() -> Result<()> {
        let set_current_lists_store_path = "/tmp/trustllo_set_current_lists_data_store_path.json";
        let mut store = Store::new(Some(set_current_lists_store_path));

        create_empty_store_file(set_current_lists_store_path)?;

        assert_eq!(true, Path::new(set_current_lists_store_path).is_file());

        let mut file = File::open(set_current_lists_store_path).unwrap();
        let mut file_contents = String::new();
        file.read_to_string(&mut file_contents)?;
        let store_data: StoreData = serde_json::from_str(&file_contents)?;
        assert_eq!(store_data.lists.len(), 0);
        assert_eq!(store.current_lists.len(), 0);
        assert_eq!(store.current_list_index, 0);

        let list1: List = FakeData::get_fake_list();
        let list2: List = FakeData::get_fake_list();
        let list3: List = FakeData::get_fake_list();
        let list4: List = FakeData::get_fake_list();

        let lists = vec![list1.clone(), list2.clone(), list3.clone(), list4.clone()];
        store.set_current_lists(lists).await?;

        assert_eq!(store.current_lists[0].id, list1.id);
        assert_eq!(store.current_lists[0].name, list1.name);
        assert_eq!(store.current_lists[1].id, list2.id);
        assert_eq!(store.current_lists[1].name, list2.name);
        assert_eq!(store.current_lists[2].id, list3.id);
        assert_eq!(store.current_lists[2].name, list3.name);
        assert_eq!(store.current_lists[3].id, list4.id);
        assert_eq!(store.current_lists[3].name, list4.name);
        assert_eq!(store.current_lists.len(), 4);
        assert_eq!(store.current_list_index, 0);

        let mut file = File::open(set_current_lists_store_path).unwrap();
        let mut file_contents = String::new();
        file.read_to_string(&mut file_contents)?;
        let store_data_from_file: StoreData = serde_json::from_str(&file_contents)?;
        assert_eq!(store_data_from_file.lists[0].id, list1.id);
        assert_eq!(store_data_from_file.lists[0].name, list1.name);
        assert_eq!(store_data_from_file.lists[1].id, list2.id);
        assert_eq!(store_data_from_file.lists[1].name, list2.name);
        assert_eq!(store_data_from_file.lists[2].id, list3.id);
        assert_eq!(store_data_from_file.lists[2].name, list3.name);
        assert_eq!(store_data_from_file.lists[3].id, list4.id);
        assert_eq!(store_data_from_file.lists[3].name, list4.name);
        assert_eq!(store_data_from_file.lists.len(), 4);
        assert_eq!(store.current_list_index, 0);

        let list5: List = FakeData::get_fake_list();
        let list6: List = FakeData::get_fake_list();
        let list7: List = FakeData::get_fake_list();

        let lists = vec![list5.clone(), list6.clone(), list7.clone()];
        store.set_current_lists(lists).await?;

        assert_eq!(store.current_lists[0].id, list5.id);
        assert_eq!(store.current_lists[0].name, list5.name);
        assert_eq!(store.current_lists[1].id, list6.id);
        assert_eq!(store.current_lists[1].name, list6.name);
        assert_eq!(store.current_lists[2].id, list7.id);
        assert_eq!(store.current_lists[2].name, list7.name);
        assert_eq!(store.current_lists.len(), 3);
        assert_eq!(store.current_list_index, 0);

        let mut file = File::open(set_current_lists_store_path).unwrap();
        let mut file_contents = String::new();
        file.read_to_string(&mut file_contents)?;
        let store_data_from_file2: StoreData = serde_json::from_str(&file_contents)?;
        assert_eq!(store_data_from_file2.lists[0].id, list5.id);
        assert_eq!(store_data_from_file2.lists[0].name, list5.name);
        assert_eq!(store_data_from_file2.lists[1].id, list6.id);
        assert_eq!(store_data_from_file2.lists[1].name, list6.name);
        assert_eq!(store_data_from_file2.lists[2].id, list7.id);
        assert_eq!(store_data_from_file2.lists[2].name, list7.name);
        assert_eq!(store_data_from_file2.lists.len(), 3);
        assert_eq!(store.current_list_index, 0);

        fs::remove_file(set_current_lists_store_path)?;
        assert_eq!(false, Path::new(set_current_lists_store_path).is_file());
        Ok(())
    }

    #[tokio::test]
    async fn set_current_list_index_spec() -> Result<()> {
        let set_current_lists_store_path = "/tmp/trustllo_set_current_lists_data_store_path.json";
        let mut store = Store::new(Some(set_current_lists_store_path));

        let list1: List = FakeData::get_fake_list();
        let list2: List = FakeData::get_fake_list();
        let lists = vec![list1, list2];

        assert_eq!(store.current_list_index, 0);
        store.set_current_lists(lists).await?;
        assert_eq!(store.current_list_index, 0);

        store.set_current_list_index(0);
        assert_eq!(store.current_list_index, 0);
        store.set_current_list_index(1);
        assert_eq!(store.current_list_index, 1);
        store.set_current_list_index(99);
        assert_eq!(store.current_list_index, 99);
        store.set_current_list_index(usize::MAX);
        assert_eq!(store.current_list_index, usize::MAX);

        assert_eq!(true, Path::new(set_current_lists_store_path).is_file());
        fs::remove_file(set_current_lists_store_path)?;
        assert_eq!(false, Path::new(set_current_lists_store_path).is_file());
        Ok(())
    }

    #[test]
    fn set_current_cards_spec() {
        let set_current_cards_data_store_path =
            "/tmp/trustllo_set_current_cards_data_store_path.json";
        let mut store = Store::new(Some(set_current_cards_data_store_path));
        assert!(store.current_cards.is_none());
        assert!(store.current_card_index.is_none());

        let card1: Card = FakeData::get_fake_card();
        let card2: Card = FakeData::get_fake_card();
        let card3: Card = FakeData::get_fake_card();
        let card4: Card = FakeData::get_fake_card();

        let cards = vec![card1.clone(), card2.clone(), card3.clone(), card4.clone()];

        store.set_current_cards(cards).await;

        assert_eq!(store.current_cards.as_ref().unwrap()[0].id, card1.id);
        assert_eq!(store.current_cards.as_ref().unwrap()[0].name, card1.name);
        assert_eq!(store.current_cards.as_ref().unwrap()[1].id, card2.id);
        assert_eq!(store.current_cards.as_ref().unwrap()[1].name, card2.name);
        assert_eq!(store.current_cards.as_ref().unwrap()[2].id, card3.id);
        assert_eq!(store.current_cards.as_ref().unwrap()[2].name, card3.name);
        assert_eq!(store.current_cards.as_ref().unwrap()[3].id, card4.id);
        assert_eq!(store.current_cards.as_ref().unwrap()[3].name, card4.name);
        assert_eq!(store.current_cards.as_ref().unwrap().len(), 4);
        assert_eq!(store.current_card_index.unwrap(), 0);

        let card5: Card = FakeData::get_fake_card();
        let card6: Card = FakeData::get_fake_card();
        let card7: Card = FakeData::get_fake_card();

        let cards = vec![card5.clone(), card6.clone(), card7.clone()];

        store.set_current_cards(cards);

        assert_eq!(store.current_cards.as_ref().unwrap()[0].id, card5.id);
        assert_eq!(store.current_cards.as_ref().unwrap()[0].name, card5.name);
        assert_eq!(store.current_cards.as_ref().unwrap()[1].id, card6.id);
        assert_eq!(store.current_cards.as_ref().unwrap()[1].name, card6.name);
        assert_eq!(store.current_cards.as_ref().unwrap()[2].id, card7.id);
        assert_eq!(store.current_cards.as_ref().unwrap()[2].name, card7.name);
        assert_eq!(store.current_cards.as_ref().unwrap().len(), 3);
        assert_eq!(store.current_card_index.unwrap(), 0);

        fs::remove_file(set_current_cards_data_store_path);
        assert_eq!(
            false,
            Path::new(set_current_cards_data_store_path).is_file()
        );
    }

    #[test]
    fn set_current_card_index_spec() {
        let set_current_card_data_store_path =
            "/tmp/trustllo_set_current_card_data_store_path.json";
        let mut store = Store::new(Some(set_current_card_data_store_path));
        assert!(store.current_card_index.is_none());

        store.set_current_card_index(0);
        assert_eq!(store.current_card_index.unwrap(), 0);
        store.set_current_card_index(1);
        assert_eq!(store.current_card_index.unwrap(), 1);
        store.set_current_card_index(99);
        assert_eq!(store.current_card_index.unwrap(), 99);
        store.set_current_card_index(usize::MAX);
        assert_eq!(store.current_card_index.unwrap(), usize::MAX);

        fs::remove_file(set_current_card_data_store_path);
        assert_eq!(false, Path::new(set_current_card_data_store_path).is_file());
    }

    #[test]
    fn set_last_card_spec() {
        let set_last_card_data_store_path = "/tmp/trustllo_set_last_card_data_store_path.json";
        let mut store = Store::new(Some(set_last_card_data_store_path));
        assert!(store.last_card.is_none());

        let card: Card = FakeData::get_fake_card();
        store.set_last_card(&card);

        assert_eq!(store.last_card.as_ref().unwrap().id, card.id);
        assert_eq!(store.last_card.as_ref().unwrap().name, card.name);

        let card2: Card = FakeData::get_fake_card();
        store.set_last_card(&card2);

        assert_eq!(store.last_card.as_ref().unwrap().id, card2.id);
        assert_eq!(store.last_card.as_ref().unwrap().name, card2.name);

        fs::remove_file(set_last_card_data_store_path);
        assert_eq!(false, Path::new(set_last_card_data_store_path).is_file());
    }

    #[tokio::test]
    async fn read_data_from_file_spec() -> Result<()> {
        let read_data_store_path = "/tmp/trustllo_read_data_store_path.json";
        let store = Store::new(Some(read_data_store_path));

        let fake_store_data = FakeData::get_fake_store_data();
        let fake_store_data_string = serde_json::to_string(&fake_store_data).unwrap();
        fs::write(read_data_store_path, fake_store_data_string)?;
        assert_eq!(true, Path::new(read_data_store_path).is_file());

        let store_data: StoreData = read_data_from_file(read_data_store_path)?;
        assert_eq!(store_data.boards.len(), fake_store_data.boards.len());
        assert_eq!(
            store_data.boards.first().unwrap().id,
            fake_store_data.boards.first().unwrap().id
        );
        assert_eq!(store.current_board_index, 0);
        assert_eq!(store_data.lists.len(), fake_store_data.lists.len());
        assert_eq!(
            store_data.lists.first().unwrap().id,
            fake_store_data.lists.first().unwrap().id
        );
        assert_eq!(store.current_board_index, 0);
        assert_eq!(store_data.updated, fake_store_data.updated);
        assert!(store.current_cards.is_none());
        assert!(store.current_card_index.is_none());

        fs::remove_file(read_data_store_path)?;
        assert_eq!(false, Path::new(read_data_store_path).is_file());
        Ok(())
    }

    #[test]
    fn create_empty_store_file_spec() -> Result<()> {
        let empty_data_store_path = "/tmp/trustllo_empty_data_store_path.json";
        let store = Store::new(Some(empty_data_store_path));

        create_empty_store_file(empty_data_store_path)?;

        assert_eq!(true, Path::new(empty_data_store_path).is_file());

        let mut file = File::open(empty_data_store_path).unwrap();
        let mut file_contents = String::new();
        file.read_to_string(&mut file_contents)?;

        let store_data: StoreData = serde_json::from_str(&file_contents)?;
        assert_eq!(store_data.updated, "missing date");
        assert_eq!(store_data.boards.len(), 0);
        assert_eq!(store.current_board_index, 0);
        assert_eq!(store_data.lists.len(), 0);
        assert_eq!(store.current_board_index, 0);
        assert!(store.current_cards.is_none());
        assert!(store.current_card_index.is_none());

        fs::remove_file(empty_data_store_path)?;
        assert_eq!(false, Path::new(empty_data_store_path).is_file());

        Ok(())
    }

    #[tokio::test]
    async fn write_data_to_file_spec() -> Result<()> {
        let write_data_store_path = "/tmp/trustllo_write_data_store_path.json";
        let store = Store::new(Some(write_data_store_path));

        let fake_store_data = FakeData::get_fake_store_data();

        write_data_to_file(write_data_store_path, fake_store_data.clone())?;

        assert_eq!(true, Path::new(write_data_store_path).is_file());

        let mut file = File::open(write_data_store_path).unwrap();
        let mut file_contents = String::new();
        file.read_to_string(&mut file_contents)?;
        let store_data_from_file: StoreData = serde_json::from_str(&file_contents)?;

        assert_eq!(
            store_data_from_file.boards.len(),
            fake_store_data.boards.len()
        );
        assert_eq!(
            store_data_from_file.boards.first().unwrap().id,
            fake_store_data.boards.first().unwrap().id
        );
        assert_eq!(store.current_board_index, 0);
        assert_eq!(
            store_data_from_file.lists.len(),
            fake_store_data.lists.len()
        );
        assert_eq!(
            store_data_from_file.lists.first().unwrap().id,
            fake_store_data.lists.first().unwrap().id
        );
        assert_eq!(store.current_board_index, 0);
        assert_eq!(store_data_from_file.updated, fake_store_data.updated);
        assert!(store.current_cards.is_none());
        assert!(store.current_card_index.is_none());

        let fake_store_data2 = FakeData::get_fake_store_data();

        assert_eq!(true, Path::new(write_data_store_path).is_file());
        write_data_to_file(write_data_store_path, fake_store_data2.clone())?;

        assert_eq!(true, Path::new(write_data_store_path).is_file());

        let mut file2 = File::open(write_data_store_path).unwrap();
        let mut file_contents2 = String::new();
        file2.read_to_string(&mut file_contents2)?;
        let store_data_from_file2: StoreData = serde_json::from_str(&file_contents2)?;

        assert_eq!(
            store_data_from_file2.boards.len(),
            fake_store_data2.boards.len()
        );
        assert_eq!(
            store_data_from_file2.boards.first().unwrap().id,
            fake_store_data2.boards.first().unwrap().id
        );
        assert_eq!(store.current_board_index, 0);
        assert_eq!(
            store_data_from_file2.lists.len(),
            fake_store_data2.lists.len()
        );
        assert_eq!(
            store_data_from_file2.lists.first().unwrap().id,
            fake_store_data2.lists.first().unwrap().id
        );
        assert_eq!(store.current_board_index, 0);
        assert_eq!(store_data_from_file2.updated, fake_store_data2.updated);
        assert!(store.current_cards.is_none());
        assert!(store.current_card_index.is_none());

        fs::remove_file(write_data_store_path)?;
        assert_eq!(false, Path::new(write_data_store_path).is_file());
        Ok(())
    }

    #[tokio::test]
    async fn remove_data_file_spec() -> Result<()> {
        let remove_data_store_path = "/tmp/trustllo_remove_data_store_path.json";
        let _store = Store::new(Some(remove_data_store_path));

        let fake_store_data = FakeData::get_fake_store_data();
        let fake_store_data_string = serde_json::to_string(&fake_store_data).unwrap();
        fs::write(remove_data_store_path, fake_store_data_string)?;

        assert_eq!(true, Path::new(remove_data_store_path).is_file());

        remove_data_file(remove_data_store_path)?;
        assert_eq!(false, Path::new(remove_data_store_path).is_file());

        Ok(())
    }
}
