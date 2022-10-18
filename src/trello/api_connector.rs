use std::env;

pub struct ApiConnector {
    prop1: bool,
}

//TODO: maybe move this struct
//
impl ApiConnector {
    pub fn loadall() {
        println!("test");

        let _apiKey = env::var("API_KEY").is_ok();
        let _apiToken = env::var("API_TOKEN").is_ok();

        // TODO: break and display message if env not present. do I need a test here?
        // the test is rather necessary for "load data initially", this implies reading the envs.
        // Then I need to add test envs for the tests to pass, right?

        // println!("{}", apiKey);
        // println!("{}", apiToken);

        // let resp = reqwest::get("https://httpbin.org/ip");

        // use hyper or reqwest

        // println!("{:#?}", resp);
    }

    pub fn get_boards() {}
    pub fn get_lists() {}
    pub fn get_cards() {}
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
