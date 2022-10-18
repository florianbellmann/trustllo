use std::env;
use std::io::Read;

pub fn ApiConnector() {
    let apiKey = env::var("API_KEY").is_ok();
    let apiToken = env::var("API_TOKEN").is_ok();
    // TODO: break and display message if env not present. do I need a test here?
    // the test is rather necessary for "load data initially", this implies reading the envs.
    // Then I need to add test envs for the tests to pass, right?

    // println!("{}", apiKey);
    // println!("{}", apiToken);

    // let resp = reqwest::get("https://httpbin.org/ip");

    // use hyper or reqwest

    // println!("{:#?}", resp);
}

#[cfg(test)]
mod tests {
    use std::env;

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
}