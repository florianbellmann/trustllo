use async_trait::async_trait;
use crate::domain::entities::{Board, Card, List};

pub mod api_repository;

#[async_trait]
pub trait KanbanRepository {
    async fn get_current_board(&self) -> Board;
    async fn get_current_lists(&self) -> Vec<List>;
    async fn get_current_list_index(&self) -> usize;
    async fn get_current_cards(&self) -> Vec<Card>;
    async fn get_current_card_index(&self) -> usize;
}
