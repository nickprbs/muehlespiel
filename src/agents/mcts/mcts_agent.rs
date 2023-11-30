use crate::agents::Agent;
use crate::structs::Turn;
use crate::types::GameContext;

pub struct MctsAgent {}

impl Agent for MctsAgent {
    fn get_next_turn(&self, context: &GameContext) -> Turn {
        todo!()
    }
}