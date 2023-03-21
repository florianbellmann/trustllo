use async_trait::async_trait;
use std::collections::HashMap;

use anyhow::{anyhow, Result};

use log::{error, info};
use reqwest::{Method, StatusCode};

use crate::infrastructure::config_manager::ConfigManager;

use super::{Board, Card, KanbanRepository, List};

use serde::{Deserialize, Serialize};

pub struct ApiKanbanRepository {
    api_key: String,
    api_token: String,
    member_id: String,

    client: reqwest::Client,
}

#[async_trait]
impl KanbanRepository for ApiKanbanRepository {
    async fn get_current_board(&self) -> Board {
        let boards = self.get_boards().await.unwrap();
        let board = boards
            .iter()
            .find(|board| board.name == "Dev board")
            .unwrap()
            .clone();
        board
    }

    async fn get_current_lists(&self) -> Vec<List> {
        let board = self.get_current_board().await;
        let lists = self.get_lists_on_board(board.id.as_str()).await.unwrap();
        lists
    }

    async fn get_current_list_index(&self) -> usize {
        0
    }

    async fn get_current_cards(&self) -> Vec<Card> {
        let lists = self.get_current_lists().await;
        let cards = self.get_cards_on_list(lists[0].id.as_str()).await.unwrap();
        cards
    }

    async fn get_current_card_index(&self) -> usize {
        0
    }
}

pub struct Endpoint {}
impl Endpoint {
    const DEFAULT_CONFIG_LOCATION: &str = "/Users/florian.juesten/.config/trustllo/config.json";
    pub const CARDS: &str = "/cards";
    pub const BOARDS: &str = "/boards";
    pub const LISTS: &str = "/lists";
    pub const MEMBERS: &str = "/members";
    pub const ACTIONS: &str = "/actions";
    pub const SEARCH: &str = "/search";
    pub const CHECKLISTS: &str = "/checklists";
}

const API_URL: &str = "https://api.trello.com/1";

impl ApiKanbanRepository {
    pub fn new() -> ApiKanbanRepository {
        let config = ConfigManager::read_config(None).unwrap(); //TODO: this is also still hardcoded

        ApiKanbanRepository {
            api_key: config.api_key,
            api_token: config.api_token,
            member_id: config.member_id,
            client: reqwest::Client::new(),
        }
    }

    // const initialBoard = await this._trelloConnector.getBoardByName(this.getInitialBoardName())
    //         await this._trelloConnector.appendCard(appendName, (await this._storageProvider.getCurrentList()).id)
    //         await this._trelloConnector.prependCard(prependName, (await this._storageProvider.getCurrentList()).id)
    //         await this._trelloConnector.switchBoard()
    //         await this._trelloConnector.switchListRight()
    //         await this._trelloConnector.switchListLeft()
    //           await this._trelloConnector.cardDown(currentCardDown, nextCard.pos + 1)
    //           await this._trelloConnector.cardDown(currentCardUp, prevCard.pos - 1)
    //         await this._trelloConnector.moveToToday(actionCard)
    //         await this._trelloConnector.moveToTomorrow(actionCard)

    // boards
    // ----------------------------------------------------------------------------------------------------------------
    async fn get_boards(&self) -> Result<Vec<Board>> {
        info!("Getting boards from API.");
        let boards: Vec<Board> = self
            .make_request(
                Endpoint::MEMBERS,
                Method::GET,
                format!("/{}/boards", self.member_id),
                None,
            )
            .await?;
        Ok(boards)
    }

    // lists
    // ----------------------------------------------------------------------------------------------------------------
    async fn get_lists_on_board(&self, board_id: &str) -> Result<Vec<List>> {
        info!("Getting lists on board {} from API.", board_id);
        let lists: Vec<List> = self
            .make_request(
                Endpoint::BOARDS,
                Method::GET,
                format!("/{}/lists", board_id),
                None,
            )
            .await?;
        Ok(lists)
    }

    // cards
    // ----------------------------------------------------------------------------------------------------------------
    async fn get_cards_on_list(&self, list_id: &str) -> Result<Vec<Card>> {
        info!("Getting cards on list {} from API.", list_id);
        let cards: Vec<Card> = self
            .make_request(
                Endpoint::LISTS,
                Method::GET,
                format!("/{}/cards", list_id),
                None,
            )
            .await?;
        Ok(cards)
    }

    async fn get_card_by_id(&self, card_id: &str) -> Result<Card> {
        info!("Getting card {} from API.", card_id);
        let card: Card = self
            .make_request(Endpoint::CARDS, Method::GET, format!("/{}", card_id), None)
            .await?;
        Ok(card)
    }

    async fn add_card(&self, name: &str, description: &str, list_id: &str) -> Result<Card> {
        let mut params = HashMap::new();
        params.insert("idList", list_id);
        params.insert("name", name);
        params.insert("desc", description);

        info!("Adding card {} to list {}.", name, list_id);
        let card: Card = self
            .make_request(Endpoint::CARDS, Method::POST, "".to_string(), Some(params))
            .await?;

        Ok(card)
    }

    async fn archive_card(&self, card_id: &str) -> Result<Card> {
        info!("Archiving card {}.", card_id);
        let card: Card = self.update_card(card_id, "closed", "true").await?;
        Ok(card)
    }

    async fn unarchive_card(&self, card_id: &str) -> Result<Card> {
        info!("Unarchiving card {}.", card_id);
        let card: Card = self.update_card(card_id, "closed", "false").await?;
        Ok(card)
    }

    // async fn add_checklist_to_card(&self, _card_id: &str, _name: &str) -> Result<()> {
    //     todo!("Not implemented yet");
    // }

    // async fn get_checklists_on_card(&self, _card_id: &str) -> Result<()> {
    //     todo!("Not implemented yet");
    // }

    // async fn add_item_to_checklist(
    //     &self,
    //     _check_list_id: &str,
    //     _name: &str,
    //     _pos: &str,
    // ) -> Result<()> {
    //     todo!("Not implemented yet");
    // }

    async fn update_card(&self, card_id: &str, field: &str, value: &str) -> Result<Card> {
        let mut params = HashMap::new();
        params.insert("value", value);

        info!("Updating card {} on field {}.", card_id, field);
        let card: Card = self
            .make_request(
                Endpoint::CARDS,
                Method::PUT,
                format!("/{}/{}", card_id, field),
                Some(params),
            )
            .await?;

        Ok(card)
    }

    // async fn update_checklist(
    //     &self,
    //     _checklist_id: &str,
    //     _field: &str,
    //     _value: &str,
    // ) -> Result<()> {
    //     todo!("Not implemented yet");
    // }

    async fn update_card_description(&self, card_id: &str, description: &str) -> Result<Card> {
        info!("Updating description for card {}.", card_id);
        let card: Card = self.update_card(card_id, "desc", description).await?;
        Ok(card)
    }

    async fn update_card_title(&self, card_id: &str, title: &str) -> Result<Card> {
        info!("Updating title for card {}.", card_id);
        let card: Card = self.update_card(card_id, "name", title).await?;
        Ok(card)
    }

    async fn update_card_due_date(&self, card_id: &str, date_value: &str) -> Result<Card> {
        info!("Updating due date for card {}.", card_id);
        let card: Card = self.update_card(card_id, "due", date_value).await?;
        Ok(card)
    }

    // ----------------------------------------------------------------------------------------------------------------
    async fn make_request<T>(
        &self,
        endpoint: &str,
        request_method: Method,
        path: String,
        params: Option<HashMap<&str, &str>>,
    ) -> Result<T>
    where
        T: for<'a> Deserialize<'a>,
    {
        //TODO: static config is also still hardcoded.
        // also does this make sense to always read in from file? the config should be stored in
        // memory

        let request_url = format!("{}{}{}", API_URL, endpoint, path,);

        let mut url_params = params.unwrap_or_default();
        url_params.insert("key", &self.api_key);
        url_params.insert("token", &self.api_token);

        let client = reqwest::Client::builder().build()?;
        let request = match request_method {
            Method::GET => client.get(&request_url),
            Method::POST => client.post(&request_url),
            Method::PATCH => client.patch(&request_url),
            Method::PUT => client.put(&request_url),
            Method::DELETE => client.delete(&request_url),
            _ => client.get(&request_url),
        };

        // TODO: log the actual calls that are happening. Like an app log
        let response = request
            .query(&url_params)
            .header("Accept", "application/json")
            .send()
            .await?;

        match response.status() {
            StatusCode::OK => match response.json::<T>().await {
                Ok(response_data) => Ok(response_data),
                Err(e) => {
                    let error_msg = format!("ERROR: Failed to parse the Api response. {}", e);
                    error!("{}", error_msg);
                    Err(anyhow!(error_msg))
                }
            },
            _ => {
                let response_status = response.status();
                match response.text().await {
                    Ok(response_text) => {
                        let error_msg = format!(
                            "ERROR calling the Api. \n  Status code '{}' was received. \n  Message: '{}'\n  Url: '{}'\n  Params: '{:#}'",
                            response_status, response_text,request_url,serde_json::to_string(&url_params).unwrap()
                        );
                        error!("{}", error_msg);
                        // TODO: do I need this?
                        // let bt = Backtrace::new();
                        // println!("{:?}", bt);
                        Err(anyhow!(error_msg))
                    }
                    Err(e) => panic!("ERROR: Completely failed Api call. {}", e),
                }
            }
        }
    }
}

// // TODO: still a lot of the fields from the trello objects missing in the assertions
// #[cfg(test)]
// mod tests {
//     use crate::trello::api_connector::ApiConnector;
//     use crate::utils::types::get_type_of;
//     use anyhow::Result;

//     // INFO:
//     // We want to always be sure the actual api responses still work with this application. Therefore we
//     // need to validate the schemas for the responses. Based on these schemas we can then also do mocks,
//     // which we use for testing other components of this application.

//     #[tokio::test]
//     async fn get_boards_spec() -> Result<()> {
//         // load boards and verify parsed result type
//         let api_connector = ApiConnector::new();
//         let boards = api_connector.get_boards().await?;

//         assert_eq!(
//             get_type_of(&boards),
//             "alloc::vec::Vec<trustllo::trello::Board>"
//         );
//         assert!(!boards.first().unwrap().id.is_empty());
//         assert!(!boards.first().unwrap().name.is_empty());

//         Ok(())
//     }

//     #[tokio::test]
//     async fn get_card_spec() -> Result<()> {
//         // get a specific card on a board
//         let api_connector = ApiConnector::new();
//         let card_id = std::env::var("CARD_ID").unwrap().to_owned();
//         let card = api_connector.get_card_by_id(&card_id).await?;

//         assert_eq!(get_type_of(&card), "trustllo::trello::Card");
//         assert!(!card.id.is_empty());
//         assert!(!card.name.is_empty());
//         assert_eq!(card.name, "card 2");

//         Ok(())
//     }

//     #[tokio::test]
//     async fn get_lists_on_board_spec() -> Result<()> {
//         // load lists on a board and verify parsed result type
//         let board_id = std::env::var("BOARD_ID").unwrap().to_owned();
//         let api_connector = ApiConnector::new();
//         let lists = api_connector.get_lists_on_board(&board_id).await?;

//         assert_eq!(
//             get_type_of(&lists),
//             "alloc::vec::Vec<trustllo::trello::List>"
//         );
//         assert!(!lists.first().unwrap().id.is_empty());
//         assert!(!lists.first().unwrap().name.is_empty());

//         Ok(())
//     }

//     #[tokio::test]
//     async fn get_cards_on_list_spec() -> Result<()> {
//         // load cards on a list and verify parsed result type
//         let list_id = std::env::var("LIST_ID").unwrap().to_owned();
//         let api_connector = ApiConnector::new();
//         let cards = api_connector.get_cards_on_list(&list_id).await?;

//         assert_eq!(
//             get_type_of(&cards),
//             "alloc::vec::Vec<trustllo::trello::Card>"
//         );
//         assert!(!cards.first().unwrap().id.is_empty());
//         assert!(!cards.first().unwrap().name.is_empty());

//         Ok(())
//     }

//     #[tokio::test]
//     async fn add_card_spec() -> Result<()> {
//         // // add a card to a list
//         let list_id = std::env::var("LIST_ID").unwrap().to_owned();
//         let api_connector = ApiConnector::new();
//         let result_card = api_connector
//             .add_card("Test card name", "Test card description", &list_id)
//             .await?;

//         assert_eq!(get_type_of(&result_card), "trustllo::trello::Card");
//         assert!(!&result_card.id.is_empty());
//         assert!(!&result_card.name.is_empty());
//         assert_eq!(&result_card.name, "Test card name");
//         assert_eq!(&result_card.desc, "Test card description");

//         let result_card = api_connector.archive_card(&result_card.id).await?;

//         assert_eq!(get_type_of(&result_card), "trustllo::trello::Card");
//         assert_eq!(result_card.closed, true);
//         Ok(())
//     }

//     #[tokio::test]
//     async fn archive_card_spec() -> Result<()> {
//         // archive a card
//         let card_id = std::env::var("CARD_ID").unwrap().to_owned();
//         let api_connector = ApiConnector::new();
//         let result_card = api_connector.archive_card(&card_id).await?;

//         assert_eq!(get_type_of(&result_card), "trustllo::trello::Card");
//         assert_eq!(result_card.closed, true);

//         let result_card = api_connector.unarchive_card(&card_id).await?;

//         assert_eq!(get_type_of(&result_card), "trustllo::trello::Card");
//         assert_eq!(result_card.closed, false);
//         Ok(())
//     }

//     #[tokio::test]
//     async fn unarchive_card_spec() -> Result<()> {
//         // unarchive a card
//         let card_id = std::env::var("CARD_ID2").unwrap().to_owned();
//         let api_connector = ApiConnector::new();
//         let result_card = api_connector.unarchive_card(&card_id).await?;

//         assert_eq!(get_type_of(&result_card), "trustllo::trello::Card");
//         assert_eq!(result_card.closed, false);

//         let result_card = api_connector.archive_card(&card_id).await?;

//         assert_eq!(get_type_of(&result_card), "trustllo::trello::Card");
//         assert_eq!(result_card.closed, true);
//         Ok(())
//     }

//     #[tokio::test]
//     async fn update_card_spec() -> Result<()> {
//         // update several card fields at once
//         let list_id = std::env::var("LIST_ID").unwrap().to_owned();
//         let api_connector = ApiConnector::new();
//         let result_card = api_connector
//             .add_card("Card with multiple field update", "", &list_id)
//             .await?;

//         assert_eq!(get_type_of(&result_card), "trustllo::trello::Card");
//         assert!(!&result_card.id.is_empty());
//         assert!(!&result_card.name.is_empty());
//         assert_eq!(&result_card.name, "Card with multiple field update");

//         let result_card = api_connector
//             .update_card(&result_card.id, "name", "new name")
//             .await?;
//         assert_eq!(get_type_of(&result_card), "trustllo::trello::Card");
//         assert!(!&result_card.id.is_empty());
//         assert_eq!(&result_card.name, "new name");

//         let result_card = api_connector
//             .update_card(&result_card.id, "desc", "new desc")
//             .await?;
//         assert_eq!(get_type_of(&result_card), "trustllo::trello::Card");
//         assert!(!&result_card.id.is_empty());
//         assert_eq!(&result_card.desc, "new desc");

//         let result_card = api_connector
//             .update_card(&result_card.id, "closed", "true")
//             .await?;
//         assert_eq!(get_type_of(&result_card), "trustllo::trello::Card");
//         assert!(!&result_card.id.is_empty());
//         assert_eq!(result_card.closed, true);

//         Ok(())
//     }

//     #[tokio::test]
//     async fn update_card_description_spec() -> Result<()> {
//         let list_id = std::env::var("LIST_ID").unwrap().to_owned();
//         let api_connector = ApiConnector::new();
//         let result_card = api_connector
//             .add_card("Update desc card", "", &list_id)
//             .await?;

//         assert_eq!(get_type_of(&result_card), "trustllo::trello::Card");
//         assert!(!&result_card.id.is_empty());
//         assert!(!&result_card.name.is_empty());
//         assert_eq!(&result_card.desc, "");

//         let result_card = api_connector
//             .update_card_description(&result_card.id, "Test desc 123")
//             .await?;

//         assert_eq!(get_type_of(&result_card), "trustllo::trello::Card");
//         assert!(!&result_card.id.is_empty());
//         assert!(!&result_card.name.is_empty());
//         assert_eq!(&result_card.desc, "Test desc 123");

//         let result_card = api_connector.archive_card(&result_card.id).await?;

//         assert_eq!(get_type_of(&result_card), "trustllo::trello::Card");
//         assert_eq!(result_card.closed, true);

//         Ok(())
//     }

//     #[tokio::test]
//     async fn update_card_due_date_spec() -> Result<()> {
//         // TODO: field not implemented yet

//         // let list_id = std::env::var("LIST_ID").unwrap().to_owned();
//         // let api_connector = ApiConnector::new();
//         // let result_card = api_connector
//         //     .add_card("Update due card", "", &list_id)
//         //     .await?;

//         // assert_eq!(get_type_of(&result_card), "trustllo::trello::Card");
//         // assert!(!&result_card.id.is_empty());
//         // assert!(!&result_card.name.is_empty());
//         // // TODO: dates not implemented yet!
//         // // assert_eq!(&result_card.due, "");

//         // let result_card = api_connector
//         //     .update_card_due_date(&result_card.id, "2022-10-10")
//         //     .await?;

//         // assert_eq!(get_type_of(&result_card), "trustllo::trello::Card");
//         // assert!(!&result_card.id.is_empty());
//         // assert!(!&result_card.name.is_empty());
//         // assert_eq!(&result_card.name, "2022-10-10");

//         // let result_card = api_connector
//         //     .update_card_due_date(&result_card.id, "2021-10-11")
//         //     .await?;

//         // assert_eq!(get_type_of(&result_card), "trustllo::trello::Card");
//         // assert!(!&result_card.id.is_empty());
//         // assert!(!&result_card.name.is_empty());
//         // assert_eq!(&result_card.name, "2021`-10-11");

//         // let result_card = api_connector
//         //     .update_card_due_date(&result_card.id, "")
//         //     .await?;

//         // assert_eq!(get_type_of(&result_card), "trustllo::trello::Card");
//         // assert!(!&result_card.id.is_empty());
//         // assert!(!&result_card.name.is_empty());
//         // assert_eq!(&result_card.name, "");

//         // let result_card = api_connector.archive_card(&result_card.id).await?;

//         // assert_eq!(get_type_of(&result_card), "trustllo::trello::Card");
//         // assert_eq!(result_card.closed(), true);

//         Ok(())
//     }

//     #[tokio::test]
//     async fn update_card_title_spec() -> Result<()> {
//         let list_id = std::env::var("LIST_ID").unwrap().to_owned();
//         let api_connector = ApiConnector::new();
//         let result_card = api_connector
//             .add_card("Update tutle card", "", &list_id)
//             .await?;

//         assert_eq!(get_type_of(&result_card), "trustllo::trello::Card");
//         assert!(!&result_card.id.is_empty());
//         assert!(!&result_card.name.is_empty());
//         assert_eq!(&result_card.name, "Update tutle card");

//         let result_card = api_connector
//             .update_card_title(&result_card.id, "Better title!")
//             .await?;

//         assert_eq!(get_type_of(&result_card), "trustllo::trello::Card");
//         assert!(!&result_card.id.is_empty());
//         assert!(!&result_card.name.is_empty());
//         assert_eq!(&result_card.name, "Better title!");

//         let result_card = api_connector.archive_card(&result_card.id).await?;

//         assert_eq!(get_type_of(&result_card), "trustllo::trello::Card");
//         assert_eq!(result_card.closed, true);

//         Ok(())
//     }
// }
