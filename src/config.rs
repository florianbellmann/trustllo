use serde::{Deserialize, Serialize};

pub mod config_manager;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    api_key: String,
    api_token: String,
    member_id:String,
}
