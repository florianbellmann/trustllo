pub mod store;
use crate::trello::{Board, Card, Endpoint, List};
use serde::{Deserialize, Serialize};

// TODO: Store data and the props of the the store are the same. with the difference of options
#[derive(Debug, Serialize, Deserialize)]
pub struct StoreData {
    updated: String, //TODO: proper date handling
    boards: Vec<Board>,
    lists: Vec<List>,
}
