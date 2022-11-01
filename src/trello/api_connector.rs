use std::collections::HashMap;

use anyhow::{anyhow, ensure, Context, Error, Result};

use reqwest::{Method, StatusCode};
use serde::{Deserialize, Serializer};

use crate::config::config_manager::ConfigManager;

use super::{Board, Card, Endpoint, List};

pub struct ApiConnector {}

//TODO: maybe move this struct
//
impl ApiConnector {
    const API_URL: &str = "https://api.trello.com/1";

    pub fn new() -> ApiConnector {
        ApiConnector {}
    }

    //TODO: load data async to get better startup
    pub async fn init(&self) {}

    // const initialBoard = await this._trelloConnector.getBoardByName(this.getInitialBoardName())
    //         await this._trelloConnector.archiveCard(actionCard)
    //         await this._trelloConnector.unArchiveCard(lastCard)
    //         await this._trelloConnector.changeDate(actionCard, dateStringToDate(newDateString))
    //       // await this._trelloConnector.
    //         await this._trelloConnector.changeTitle(actionCard, newTitle)
    //         await this._trelloConnector.appendCard(appendName, (await this._storageProvider.getCurrentList()).id)
    //         await this._trelloConnector.prependCard(prependName, (await this._storageProvider.getCurrentList()).id)
    //         await this._trelloConnector.switchBoard()
    //         await this._trelloConnector.switchListRight()
    //         await this._trelloConnector.switchListLeft()
    //           await this._trelloConnector.cardDown(currentCardDown, nextCard.pos + 1)
    //           await this._trelloConnector.cardDown(currentCardUp, prevCard.pos - 1)
    //         await this._trelloConnector.moveToToday(actionCard)
    //         await this._trelloConnector.moveToTomorrow(actionCard)
    //         await this._trelloConnector.changeDescription(actionCard, newDesc)

    // boards
    // ----------------------------------------------------------------------------------------------------------------
    pub async fn get_boards(&self) -> Result<Vec<Board>> {
        let config = ConfigManager::read_config(None).unwrap(); //TODO: this is also still hardcoded

        let boards: Vec<Board> = self
            .make_request(
                Endpoint::MEMBERS,
                Method::GET,
                format!("/{}/boards", config.member_id),
                None,
            )
            .await?;
        Ok(boards)
    }

    // lists
    // ----------------------------------------------------------------------------------------------------------------
    pub async fn get_lists_on_board(&self, board_id: &str) -> Result<Vec<List>> {
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
    // pub async fn get_card(&self, _board_id: &str, _card_id: &str) -> Result<()> {
    //     todo!("Not implemented yet");
    // }

    pub async fn get_cards_for_list(&self, list_id: &str) -> Result<Vec<Card>> {
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

    // pub async fn get_labels_for_board(&self, _board_id: &str) -> Result<()> {
    //     todo!("Not implemented yet");
    // }

    pub async fn add_label_to_card(&self, _card_id: &str, _label_id: &str) -> Result<()> {
        todo!("Not implemented yet");
    }

    pub async fn delete_label_from_card(&self, _card_id: &str, _label_id: &str) -> Result<()> {
        todo!("Not implemented yet");
    }

    pub async fn add_due_date_to_card(&self, _card_id: &str, _date_value: &str) -> Result<()> {
        todo!("Not implemented yet");
    }
    pub async fn add_card(&self, name: &str, description: &str, list_id: &str) -> Result<Card> {
        let mut params = HashMap::new();
        params.insert("idList", list_id);
        params.insert("name", name);
        params.insert("desc", description);

        let card: Card = self
            .make_request(Endpoint::CARDS, Method::POST, "".to_string(), Some(params))
            .await?;
        Ok(card)
    }

    pub async fn add_checklist_to_card(&self, _card_id: &str, _name: &str) -> Result<()> {
        todo!("Not implemented yet");
    }

    pub async fn get_checklists_on_card(&self, _card_id: &str) -> Result<()> {
        todo!("Not implemented yet");
    }

    pub async fn add_item_to_checklist(
        &self,
        _check_list_id: &str,
        _name: &str,
        _pos: &str,
    ) -> Result<()> {
        todo!("Not implemented yet");
    }

    pub async fn update_card(&self, _card_id: &str, _field: &str, _value: &str) -> Result<()> {
        todo!("Not implemented yet");
    }

    pub async fn update_checklist(
        &self,
        _checklist_id: &str,
        _field: &str,
        _value: &str,
    ) -> Result<()> {
        todo!("Not implemented yet");
    }

    pub async fn update_card_name(&self, _card_id: &str, _name: &str) -> Result<()> {
        todo!("Not implemented yet");
    }

    pub async fn update_card_description(&self, _card_id: &str, _description: &str) -> Result<()> {
        todo!("Not implemented yet");
    }

    pub async fn delete_card(&self, _card_id: &str) -> Result<()> {
        todo!("Not implemented yet");
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
        let config = ConfigManager::read_config(None).unwrap(); //TODO: static config is also still hardcoded

        let request_url = format!("{}{}{}", ApiConnector::API_URL, endpoint, path,);

        let mut url_params = params.unwrap_or_default();
        url_params.insert("key", &config.api_key);
        url_params.insert("token", &config.api_token);

        let client = reqwest::Client::builder().build()?;
        let request = match request_method {
            Method::GET => client.get(&request_url),
            Method::POST => client.post(&request_url),
            Method::PATCH => client.patch(&request_url),
            Method::PUT => client.put(&request_url),
            Method::DELETE => client.delete(&request_url),
            _ => client.get(&request_url),
        };
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
                    println!("{}", error_msg);
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
                        println!("{}", error_msg);
                        Err(anyhow!(error_msg))
                    }
                    Err(e) => panic!("ERROR: Completely failed Api call. {}", e),
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::trello::api_connector::ApiConnector;
    use crate::utils::types::get_type_of;
    use anyhow::Result;

    // INFO:
    // Reasoning behind testing with mock responses:
    // We want to always be sure the actual api responses still work with this application. Therefore we
    // need to validate the schemas for the responses. Based on these schemas we can then also do mocks,
    // which we use for testing other components of this application.

    #[tokio::test]
    async fn get_boards_spec() -> Result<()> {
        // load boards and verify parsed result type
        let api_connector = ApiConnector::new();
        let boards = api_connector.get_boards().await?;
        assert_eq!(
            get_type_of(&boards),
            "alloc::vec::Vec<trustllo::trello::Board>"
        );
        assert!(!boards.first().unwrap().id.is_empty());
        assert!(!boards.first().unwrap().name.is_empty());
        Ok(())
    }

    #[tokio::test]
    async fn get_lists_on_board_spec() -> Result<()> {
        // load lists on a board and verify parsed result type
        let board_id = std::env::var("BOARD_ID").unwrap().to_owned();
        let api_connector = ApiConnector::new();
        let lists = api_connector.get_lists_on_board(&board_id).await?;
        assert_eq!(
            get_type_of(&lists),
            "alloc::vec::Vec<trustllo::trello::List>"
        );
        assert!(!lists.first().unwrap().id.is_empty());
        assert!(!lists.first().unwrap().name.is_empty());
        Ok(())
    }

    #[tokio::test]
    async fn get_cards_for_list_spec() -> Result<()> {
        // load cards on a list and verify parsed result type
        let list_id = std::env::var("LIST_ID").unwrap().to_owned();
        let api_connector = ApiConnector::new();
        let cards = api_connector.get_cards_for_list(&list_id).await?;
        assert_eq!(
            get_type_of(&cards),
            "alloc::vec::Vec<trustllo::trello::Card>"
        );
        assert!(!cards.first().unwrap().id.is_empty());
        assert!(!cards.first().unwrap().name.is_empty());
        Ok(())
    }

    #[tokio::test]
    async fn add_card_spec() -> Result<()> {
        // add a card to a list
        let list_id = std::env::var("LIST_ID").unwrap().to_owned();
        let api_connector = ApiConnector::new();
        let result_card = api_connector
            .add_card("Test card name", "test description", &list_id)
            .await;
        assert_eq!(
            get_type_of(&result_card),
            "alloc::vec::Vec<trustllo::trello::Card>"
        );
        // assert!(!&result_card.unwrap().id.is_empty());
        // assert!(!&result_card.unwrap().name.is_empty());

        Ok(())
    }
}
