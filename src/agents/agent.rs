use crate::structs::Turn;
use crate::types::GameContext;

pub trait Agent {
    fn get_next_turn(&self, context: &GameContext) -> Turn;
}