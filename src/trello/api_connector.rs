use std::collections::HashMap;

use anyhow::Result;

use reqwest::Method;
use serde::Deserialize;

use crate::config::config_manager::ConfigManager;

use super::{Board, Endpoint, List};

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

    // boards
    // ----------------------------------------------------------------------------------------------------------------
    pub async fn get_boards(&self) -> Result<Vec<Board>> {
        let params = HashMap::new();

        let config = ConfigManager::read_config(None).unwrap(); //TODO: this is also still hardcoded

        let boards: Vec<Board> = self
            .make_request(
                Endpoint::MEMBERS,
                Method::GET,
                format!("/{}/boards", config.member_id),
                Some(params),
            )
            .await?;
        Ok(boards)
    }

    // lists
    // ----------------------------------------------------------------------------------------------------------------
    pub async fn get_lists_on_board(&self, _board_id: &str) -> Result<()> {
        let mut params = HashMap::new();

        params.insert("testekey", "testevalue");

        let _lists: List = self
            .make_request(
                Endpoint::LISTS,
                Method::GET,
                "/pathtest".to_string(),
                Some(params),
            )
            .await?;
        // }
        Ok(())
    }


    // cards
    // ----------------------------------------------------------------------------------------------------------------
    pub async fn get_card(&self, _board_id: &str, _card_id: &str) -> Result<()> {
        todo!("Not implemented yet");
    }

    pub async fn get_cards_for_list(&self, _list_id: &str) -> Result<()> {
        let _list: List = self
            .make_request(Endpoint::CARDS, Method::GET, "".to_string(), None)
            .await?;
        Ok(())
        // todo!("Not implemented yet");
    }

    pub async fn get_labels_for_board(&self, _board_id: &str) -> Result<()> {
        todo!("Not implemented yet");
    }

    pub async fn add_label_to_card(&self, _card_id: &str, _label_id: &str) -> Result<()> {
        todo!("Not implemented yet");
    }

    pub async fn delete_label_from_card(&self, _card_id: &str, _label_id: &str) -> Result<()> {
        todo!("Not implemented yet");
    }

    pub async fn add_due_date_to_card(&self, _card_id: &str, _date_value: &str) -> Result<()> {
        todo!("Not implemented yet");
    }
    pub async fn add_card(&self, _name: &str, _description: &str, _list_id: &str) -> Result<()> {
        todo!("Not implemented yet");
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
        let res = match request_method {
            Method::GET => client.get(&request_url),
            Method::POST => client.post(&request_url),
            Method::PATCH => client.patch(&request_url),
            Method::PUT => client.put(&request_url),
            Method::DELETE => client.delete(&request_url),
            _ => client.get(&request_url),
        };
        let response_json = res
            .query(&url_params)
            .header("Accept", "application/json")
            .send()
            .await?
            .json::<T>()
            .await?;

        Ok(response_json)
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
}
