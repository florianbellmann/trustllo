use serde::Deserialize;

pub mod config_manager;

#[derive(Deserialize, Debug)]
pub struct Config {
    api_key: String,
    api_token: String,
}
