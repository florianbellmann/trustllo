use std::env;

pub fn ApiConnector() {
    let apiKey = env::var("API_KEY").is_ok();
    let apiToken = env::var("API_TOKEN").is_ok();
    println!("test")
}
