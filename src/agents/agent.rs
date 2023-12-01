use crate::structs::Turn;
use crate::types::{GameBoardHistoryCounter, GameContext};

pub trait Agent {
    fn get_next_turn(&self, context: &GameContext, history: &impl GameBoardHistoryCounter) -> Turn;
}