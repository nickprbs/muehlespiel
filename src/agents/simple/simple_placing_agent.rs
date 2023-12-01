use crate::agents::Agent;
use crate::iterators::TurnIterator;
use crate::structs::Turn;
use crate::types::game_board::QueryableGameBoard;
use crate::types::{GameBoardHistoryCounter, GameContext};

/**
A simple placing agent
Uses only the evaluation function to determine which element to place next
**/
pub struct SimplePlacingAgent {}

impl Agent for SimplePlacingAgent {
    fn get_next_turn(&self, context: &GameContext, _: &impl GameBoardHistoryCounter) -> Turn {
        TurnIterator::new(context, context.team.get_opponent())
            .max_by(|turn_a, turn_b|
                context.apply_unsafely_copied(*turn_a)
                    .board.get_evaluation_for(&context.team)
                    .partial_cmp(
                        &context.apply_unsafely_copied(*turn_b)
                            .board.get_evaluation_for(&context.team)
                    )
                    .expect("Could not make ordering")
            )
            .expect("No free place found")
    }
}