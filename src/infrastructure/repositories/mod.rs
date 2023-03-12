use crate::domain::entities::{Board, Card, List};

pub mod api_repository;

pub trait KanbanRepository {
    fn get_current_board(&self) -> Board;
    fn get_current_lists(&self) -> Vec<List>;
    fn get_current_list_index(&self) -> usize;
    fn get_current_cards(&self) -> Vec<Card>;
    fn get_current_card_index(&self) -> usize;
}
