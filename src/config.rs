use serde::{Deserialize, Serialize};

pub mod config_manager;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub api_key: String,
    pub api_token: String,
    pub member_id:String,
}
