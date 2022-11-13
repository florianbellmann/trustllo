pub mod store;
use crate::trello::{Board, Card, Endpoint, List};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct StoreData {
    updated: String, //TODO: proper date handling
    boards: Vec<Board>,
    lists: Vec<List>,
}
