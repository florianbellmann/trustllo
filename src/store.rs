pub mod store;
use crate::trello::{Board, List};
use serde::{Deserialize, Serialize};

// TODO: Store data and the props of the the store are the same. with the difference of options
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StoreData {
    pub updated: String, //TODO: proper date handling
    pub boards: Vec<Board>,
    pub lists: Vec<List>,
}
