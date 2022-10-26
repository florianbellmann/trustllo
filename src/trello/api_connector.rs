use std::collections::HashMap;
use std::env;

use anyhow::{anyhow, Result};

use reqwest::Method;
use serde::Deserialize;

use crate::config::config_manager::ConfigManager;

use super::{Endpoint, List};

pub struct ApiConnector {}

//TODO: maybe move this struct
//
impl ApiConnector {
    const API_URL: &str = "https://api.trello.com/2";
    const BOARD_NAME: &str = "Private"; // TODO: this needs to be removed

    pub fn new() -> ApiConnector {
        ApiConnector {}
    }

    pub async fn init(&self) {}

    // boards
    // ----------------------------------------------------------------------------------------------------------------
    pub async fn get_boards(&self, member_id: &str) -> Result<()> {
        todo!("Not implemented yet");
    }

    // lists
    // ----------------------------------------------------------------------------------------------------------------
    pub async fn get_lists_on_board(&self, board_id: &str) -> Result<()> {
        let mut params = HashMap::new();

        params.insert("testekey", "testevalue");

        let lists: List = self
            .make_request(Endpoint::LISTS, Method::GET, "/pathtest", Some(params))
            .await?;
        // }
        Ok(())
    }
    // pub async fn get_lists_on_board_by_filter(
    //     &self,
    //     board_id: &str,
    //     filter: &str,
    // ) -> Result<()> {
    //     todo!("Not implemented yet");
    // }
    pub async fn get_labels_for_board(&self, board_id: &str) -> Result<()> {
        todo!("Not implemented yet");
    }
    pub async fn get_actions_on_board(&self, board_id: &str) -> Result<()> {
        todo!("Not implemented yet");
    }
    pub async fn add_label_to_card(&self, card_id: &str, label_id: &str) -> Result<()> {
        todo!("Not implemented yet");
    }
    pub async fn delete_label_from_card(&self, card_id: &str, label_id: &str) -> Result<()> {
        todo!("Not implemented yet");
    }
    pub async fn add_due_date_to_card(&self, card_id: &str, date_value: &str) -> Result<()> {
        todo!("Not implemented yet");
    }

    // cards
    // ----------------------------------------------------------------------------------------------------------------
    pub async fn get_card(&self, board_id: &str, card_id: &str) -> Result<()> {
        todo!("Not implemented yet");
    }
    // TODO: refatr to the real error type
    pub async fn get_cards_for_list(&self, list_id: &str) -> Result<()> {
        let list: List = self
            .make_request(Endpoint::CARDS, Method::GET, "", None)
            .await?;
        Ok(())
        // todo!("Not implemented yet");
    }
    // pub async fn get_cards_on_board(&self, board_id: &str) -> Result<()> {
    //     todo!("Not implemented yet");
    // }
    pub async fn add_card(&self, name: &str, description: &str, list_id: &str) -> Result<()> {
        todo!("Not implemented yet");
    }
    pub async fn add_checklist_to_card(&self, card_id: &str, name: &str) -> Result<()> {
        todo!("Not implemented yet");
    }
    pub async fn get_checklists_on_card(&self, card_id: &str) -> Result<()> {
        todo!("Not implemented yet");
    }
    pub async fn add_item_to_checklist(
        &self,
        check_list_id: &str,
        name: &str,
        pos: &str,
    ) -> Result<()> {
        todo!("Not implemented yet");
    }
    pub async fn update_card(&self, card_id: &str, field: &str, value: &str) -> Result<()> {
        todo!("Not implemented yet");
    }
    pub async fn update_checklist(
        &self,
        checklist_id: &str,
        field: &str,
        value: &str,
    ) -> Result<()> {
        todo!("Not implemented yet");
    }
    pub async fn update_card_name(&self, card_id: &str, name: &str) -> Result<()> {
        todo!("Not implemented yet");
    }
    pub async fn update_card_description(&self, card_id: &str, description: &str) -> Result<()> {
        todo!("Not implemented yet");
    }
    pub async fn delete_card(&self, card_id: &str) -> Result<()> {
        todo!("Not implemented yet");
    }
    // pub async fn update_card_list(
    //     &self,
    //     card_id: &str,
    //     list_id: &str,
    // ) -> Result<()> {
    //     todo!("Not implemented yet");
    // }

    // ----------------------------------------------------------------------------------------------------------------
    // TODO: pass also params
    async fn make_request<T>(
        &self,
        endpoint: &str,
        request_method: Method,
        path: &str,
        params: Option<HashMap<&str, &str>>,
    ) -> Result<T>
    where
        T: for<'a> Deserialize<'a>,
    {
        let client = reqwest::Client::new();
        let config = ConfigManager::read_config(None).unwrap(); //TODO: this is also still hardcoded

        let request_builder = client.request(request_method, "https://httpbin.org/ip");
        request_builder.header("Accept", "application/json");

        let url_params = ApiConnector::transform_params_to_query(params.unwrap_or(HashMap::new()));
        let auth_params = format!("&key={}&token={}", config.api_key, config.api_token);

        let request_url = format!(
            "{}{}{}?{}{}",
            ApiConnector::API_URL,
            endpoint,
            path,
            auth_params,
            url_params
        );
        println!("{:#?}", request_url);

        let resp = reqwest::get(request_url).await?;
        let jsonn = resp.json::<T>().await?;

        // println!("{:#?}", &jsonn);

        Ok(jsonn)
    }

    fn transform_params_to_query<'a>(params: HashMap<&'a str, &'a str>) -> &'a str {
        let mut params_as_string = "";

        for (key, value) in params {
            println!("{} / {}", key, value);
            // params_as_string = format!("{}&{}={}", params_as_string, &key, &value);
            params_as_string = "";
        }

        params_as_string
    }
}

#[cfg(test)]
mod tests {
    // #[test]
    // fn load_lists_from_board() {}

    //TODO: readd tests
    // #[test]
    // fn read_env_api_key() {
    //     let apiKey = env::var("API_KEY").is_ok();
    //     assert_ne!(false, apiKey)
    // }

    // #[test]
    // fn read_env_api_token() {
    //     let apiToken = env::var("API_TOKEN").is_ok();
    //     assert_ne!(false, apiToken)
    // }
    //

    // Reasoning behind testing with mock responses:
    // We want to always be sure the actual api responses still work with this application. Therefore we
    // need to validate the schemas for the responses. Based on these schemas we can then also do mocks,
    // which we use for testing other components of this application.
}
