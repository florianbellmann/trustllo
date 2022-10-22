use std::{collections::HashMap, env};

use reqwest::Method;

pub struct ApiConnector {
    api_key: String,
    api_token: String,
}

//TODO: maybe move this struct
//
impl ApiConnector {
    const API_URL: &str = "https://api.trello.com/2";

    pub fn new() -> ApiConnector {
        ApiConnector {
            api_key: env::var("API_KEY").is_ok().to_string(),
            api_token: env::var("API_TOKEN").is_ok().to_string(),
        }
    }

    // TODO: refatr to the real error type
    pub async fn loadall() -> Result<(), Box<dyn std::error::Error>> {
        // TODO: break and display message if env not present. do I need a test here?
        // the test is rather necessary for "load data initially", this implies reading the envs.
        // Then I need to add test envs for the tests to pass, right?

        // println!("{}", apiKey);
        // println!("{}", apiToken);

        // println!("{:#?}", resp);

        // self.make_request();
        Ok(())
    }

    pub fn get_boards() {}
    pub fn get_lists() {}
    pub fn get_cards() {}

    // TODO: pass also params
    async fn make_request(
        &mut self,
        endpoint: String,
        request_method: Method,
        path: String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();

        let request_builder = client.request(request_method, "https://httpbin.org/ip");
        request_builder.header("Accept", "application/json");

        // TODO: I need to extend this based on https://docs.rs/reqwest/latest/reqwest/struct.RequestBuilder.html

        // TODO: add url_params
        let url_params = "".to_owned();
        let auth_params = format!("&key={}&token={}", self.api_key, self.api_token);

        let request_url = format!(
            "{}{}?{}{}",
            ApiConnector::API_URL,
            endpoint,
            auth_params,
            url_params
        );

        let resp = reqwest::get(request_url)
            .await?
            .json::<HashMap<String, String>>()
            .await?;
        println!("{:#?}", resp);

        Ok(())
    }

    // private transformParamsToQuery(params: {}): string {
    //   const urlParams = Object.keys(params)
    //     .map(function (k) {
    //       return '&' + encodeURIComponent(k) + '=' + encodeURIComponent((params as any)[k])
    //     })
    //     .join('&')
    //   return urlParams
    // }
    // }
    // }
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
