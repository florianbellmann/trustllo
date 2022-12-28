use anyhow::Result;
use log::{error, info, warn};

use crate::trello::{api_connector::ApiConnector, Board, Card, List};

use super::store::{self, Store};

pub struct DataProvider {
    api_connector: ApiConnector,
    store: Store,
}

impl DataProvider {
    // TODO: Make sure this is used as a singleton!
    pub fn new() -> DataProvider {
        let mut store = Store::new(None);
        match store.refresh_from_cache() {
            Ok(_) => DataProvider {
                api_connector: ApiConnector::new(),
                store,
            },
            Err(e) => {
                error!(
                    "Error while loading data from cache: {}. Using in memory store.",
                    e
                );
                DataProvider {
                    api_connector: ApiConnector::new(),
                    store: Store::new(None),
                }
            }
        }
    }

    // boards
    // --------------------------------------------------------------------------------------------
    pub async fn get_boards(&mut self) -> &Vec<Board> {
        if let 0 = self.store.boards.len() {
            warn!("No boards found in store. Loading from api.");
            match self.api_connector.get_boards().await {
                Ok(boards) => {
                    self.store.boards = boards;
                }
                Err(e) => error!("Error while loading boards from api: {}", e),
            }
        }
        self.store.boards.as_ref()
    }
    pub async fn get_current_board_index(&mut self) -> usize {
        if let 0 = self.store.boards.len() {
            self.get_boards().await;
        }
        self.store.current_board_index
    }
    pub async fn get_current_board(&mut self) -> &Board {
        if let 0 = self.store.boards.len() {
            self.get_boards().await;
        }
        self.store.get_current_board()
    }

    pub async fn set_boards(&mut self, boards: Vec<Board>) -> Result<()> {
        self.store.set_boards(boards).await
    }
    // listscards
    // --------------------------------------------------------------------------------------------
    pub async fn get_current_lists(&mut self) -> &Vec<List> {
        if let 0 = self.store.current_lists.len() {
            warn!("No lists found in store. Loading from api.");
            match self
                .api_connector
                .get_lists_on_board(self.store.get_current_board().id.as_str())
                .await
                // TODO: this does not reset the index yet
            {
                Ok(lists) => {
                    self.store.current_lists = lists;
                }
                Err(e) => error!("Error while loading lists from api: {}", e),
            }
        }
        self.store.current_lists.as_ref()
    }
    pub async fn get_current_list_index(&mut self) -> usize {
        if let 0 = self.store.current_lists.len() {
            self.get_current_lists().await;
        }
        self.store.current_list_index
    }
    pub async fn get_current_list(&mut self, lists: Vec<List>) -> &List {
        if let 0 = self.store.current_lists.len() {
            self.get_current_lists().await;
        }
        todo!();
        // self.store.get_current_list()
        // self.store.current_lists[self.get_current_list_index()]
    }
    pub async fn get_lists_on_board(&self, board_id: &str) -> Vec<List> {
        todo!("not implemented");
    }

    pub async fn set_current_lists(&mut self, lists: Vec<List>) -> Result<()> {
        todo!("not implemented");
    }

    // cards
    // --------------------------------------------------------------------------------------------
    pub async fn get_current_cards() -> Vec<Card> {
        todo!("not implemented");
    }
    pub async fn get_current_card_index() -> usize {
        todo!("not implemented");
    }
    pub async fn get_current_card() -> Card {
        todo!("not implemented");
    }
    pub async fn get_last_card() -> Option<Card> {
        todo!("not implemented");
    }
    pub async fn get_cards_on_list(&self, list_id: &str) -> Result<Vec<Card>> {
        todo!("not implemented");
    }
    pub async fn get_card_by_id(&self, card_id: &str) -> Result<Card> {
        todo!("not implemented");
    }

    pub async fn set_current_card_index(&mut self, index: usize) {
        todo!("not implemented");
    }
    pub async fn set_current_cards(&mut self, cards: &Vec<Card>) {
        todo!("not implemented");
    }
    pub async fn set_last_card(&mut self, card: &Card) {
        todo!("not implemented");
    }

    pub async fn add_card_to_list(
        &self,
        name: &str,
        description: &str,
        list_id: &str,
    ) -> Result<Card> {
        todo!("not implemented");
    }
    pub async fn archive_card(&self, card_id: &str) -> Result<Card> {
        todo!("not implemented");
    }
    pub async fn unarchive_card(&self, card_id: &str) -> Result<Card> {
        todo!("not implemented");
    }
    // pub async fn add_checklist_to_card(&self, _card_id: &str, _name: &str) -> Result<()> {}
    // pub async fn get_checklists_on_card(&self, _card_id: &str) -> Result<()> {}
    // pub async fn add_item_to_checklist(}
    pub async fn update_card(&self, card_id: &str, field: &str, value: &str) -> Result<Card> {
        todo!("not implemented");
    }
    // pub async fn update_checklist(}
    pub async fn update_card_description(&self, card_id: &str, description: &str) -> Result<Card> {
        todo!("not implemented");
    }
    pub async fn update_card_title(&self, card_id: &str, title: &str) -> Result<Card> {
        todo!("not implemented");
    }
    pub async fn update_card_due_date(&self, card_id: &str, date_value: &str) -> Result<Card> {
        todo!("not implemented");
    }

    // actions
    // --------------------------------------------------------------------------------------------
    pub async fn nuke_all(&mut self) -> Result<()> {
        todo!("not implemented");
    }
}

//TODO: Integration tests?
