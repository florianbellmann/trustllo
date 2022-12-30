use anyhow::Result;
use log::{error, warn};

use crate::trello::{api_connector::ApiConnector, Board, Card, List};

use super::store::Store;

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
            warn!("No boards found in store. Loading from API.");
            match self.api_connector.get_boards().await {
                Ok(boards) => {
                    self.store.boards = boards;
                    self.store.current_board_index = 0;
                }
                Err(e) => error!("Error while loading boards from API: {}", e),
            }
        }
        self.store.boards.as_ref()
    }
    pub async fn get_current_board_index(&mut self) -> usize {
        if let 0 = self.store.boards.len() {
            warn!("No current board index found in store. Loading from API.");
            self.get_boards().await;
        }
        self.store.current_board_index
    }
    pub async fn get_current_board(&mut self) -> Board {
        if let 0 = self.store.boards.len() {
            warn!("No current board found in store. Loading from API.");
            self.get_boards().await;
        }
        self.store.get_current_board().clone()
    }

    pub async fn set_boards(&mut self, boards: Vec<Board>) -> Result<()> {
        self.store.set_boards(boards).await
    }

    // lists
    // --------------------------------------------------------------------------------------------
    pub async fn get_current_lists(&mut self) -> Vec<List> {
        if let 0 = self.store.current_lists.len() {
            warn!("No lists found in store. Loading from API.");
            match self
                .api_connector
                .get_lists_on_board(self.store.get_current_board().id.as_str())
                .await
            {
                Ok(lists) => {
                    self.store.current_lists = lists;
                    self.store.current_list_index = 0;
                }
                Err(e) => error!("Error while loading lists from API: {}", e),
            }
        }
        self.store.current_lists.clone()
    }
    pub async fn get_current_list_index(&mut self) -> usize {
        if let 0 = self.store.current_lists.len() {
            warn!("No current list index found in store. Loading from API.");
            self.get_current_lists().await;
        }
        self.store.current_list_index
    }
    pub async fn get_current_list(&mut self) -> &List {
        if let 0 = self.store.current_lists.len() {
            self.get_current_lists().await;
        }
        self.store.get_current_list()
    }
    pub async fn get_lists_on_board(&self, board_id: &str) -> Vec<List> {
        match self.api_connector.get_lists_on_board(board_id).await {
            Ok(lists) => lists,
            Err(e) => {
                error!("Error while loading lists from API: {}", e);
                vec![]
            }
        }
    }

    pub async fn set_current_lists(&mut self, lists: Vec<List>) -> Result<()> {
        self.store.set_current_lists(lists)
    }

    // cards
    // --------------------------------------------------------------------------------------------
    pub async fn get_current_cards(&mut self) -> Vec<Card> {
        if self.store.current_cards.is_none()
            || self.store.current_cards.as_ref().unwrap().is_empty()
        {
            warn!("No cards found in store. Loading from API.");
            // let id = self.get_current_list().await.id.as_str();
            match self.api_connector.get_cards_on_list("2").await {
                Ok(cards) => {
                    self.store.current_cards = Some(cards);
                    self.store.current_card_index = Some(0);
                }
                Err(e) => error!("Error while loading cards from API: {}", e),
            }
        }
        self.store.current_cards.clone().unwrap()
    }
    pub async fn get_current_card_index(&mut self) -> usize {
        if self.store.current_cards.is_none()
            || self.store.current_cards.as_ref().unwrap().is_empty()
        {
            warn!("No card index found. Reloading current cards from API.");
            self.get_current_cards().await;
        }
        self.store.current_card_index.unwrap_or(0)
    }
    pub async fn get_current_card(&mut self) -> &Card {
        if self.store.current_cards.is_none()
            || self.store.current_cards.as_ref().unwrap().is_empty()
        {
            warn!("No current card found in store. Loading from API.");
            self.get_current_cards().await;
        }
        self.store.get_current_card().unwrap()
    }
    pub async fn get_last_card(&self) -> Option<&Card> {
        self.store.last_card.as_ref()
    }
    pub async fn get_cards_on_list(&self, list_id: &str) -> Vec<Card> {
        match self.api_connector.get_cards_on_list(list_id).await {
            Ok(cards) => cards,
            Err(e) => {
                error!("Error while loading cards from API: {}", e);
                vec![]
            }
        }
    }
    pub async fn get_card_by_id(&mut self, card_id: &str) -> Card {
        if self.store.current_cards.is_none() {
            warn!("No current cards not found in store. Loading from API.");
            self.get_current_cards().await;
        }
        if let card = self
            .store
            .current_cards
            .as_ref()
            .unwrap()
            .iter()
            .find(|c| c.id == card_id)
        {
            card.unwrap().clone()
        } else {
            warn!("Card not found in current cards. Loading from API.");
            match self.api_connector.get_card_by_id(card_id).await {
                Ok(card) => card,
                Err(e) => {
                    error!("Error while loading card from API: {}", e);
                    self.store.current_cards.as_ref().unwrap()[0].clone()
                }
            }
        }
    }

    pub async fn set_current_card_index(&mut self, index: usize) {
        self.store.set_current_card_index(index)
    }
    pub async fn set_current_cards(&mut self, cards: &Vec<Card>) {
        self.store.set_current_cards(cards)
    }
    pub async fn set_last_card(&mut self, card: &Card) {
        self.store.set_last_card(card)
    }

    pub async fn add_card_to_list(
        &self,
        _name: &str,
        _description: &str,
        _list_id: &str,
    ) -> Result<Card> {
        self.api_connector
            .add_card(_name, _description, _list_id)
            .await
        //TODO: missing is the creation of a card in the store that gets added to the current_cards
    }
    pub async fn archive_card(&self, card_id: &str) -> Result<Card> {
        self.api_connector.archive_card(card_id).await
    }
    pub async fn unarchive_card(&self, card_id: &str) -> Result<Card> {
        self.api_connector.unarchive_card(card_id).await
    }
    // pub async fn add_checklist_to_card(&self, _card_id: &str, _name: &str) -> Result<()> {}
    // pub async fn get_checklists_on_card(&self, _card_id: &str) -> Result<()> {}
    // pub async fn add_item_to_checklist(}
    pub async fn update_card(&self, card_id: &str, field: &str, value: &str) -> Result<Card> {
        self.api_connector.update_card(card_id, field, value).await
    }
    // pub async fn update_checklist(}
    pub async fn update_card_description(&self, card_id: &str, description: &str) -> Result<Card> {
        self.api_connector
            .update_card_description(card_id, description)
            .await
    }
    pub async fn update_card_title(&self, card_id: &str, title: &str) -> Result<Card> {
        self.api_connector.update_card_title(card_id, title).await
    }
    pub async fn update_card_due_date(&self, card_id: &str, date_value: &str) -> Result<Card> {
        self.api_connector
            .update_card_due_date(card_id, date_value)
            .await
    }

    // actions
    // --------------------------------------------------------------------------------------------
    pub async fn nuke_all(&mut self) -> Result<()> {
        self.store.nuke_all()
    }
}

//TODO: Integration tests?
