use std::fmt::format;
use crate::agents::Agent;
use crate::agents::simple::SimplePlacingAgent;
use crate::iterators::TurnIterator;
use crate::structs::{GamePhase, Team, Turn};
use crate::types::game_board::QueryableGameBoard;
use crate::types::{GameBoardHistoryCounter, GameContext};

pub const MINIMAX_DEPTH: u8 = 5;

pub struct MinimaxAgent {}

impl Agent for MinimaxAgent {
    fn get_next_turn(&self, context: &GameContext, history: &impl GameBoardHistoryCounter) -> Turn {
        let next_move = match context.phase {
            GamePhase::Placing => {
                Some(SimplePlacingAgent{}.get_next_turn(context, history))
            }
            GamePhase::Moving => self.mini_max(
                &context.team,
                &context,
                MINIMAX_DEPTH,
                history
            ).0
        };
        next_move.expect("No move found that I can do or game already finished!")
    }
}

impl MinimaxAgent {

    fn mini_max(&self,
                team_to_maximize: &Team,
                context: &GameContext,
                depth: u8,
                history: &impl GameBoardHistoryCounter
    ) -> (Option<Turn>, f32) {
        return if depth == 0 {
            (None, context.board.get_evaluation_for(team_to_maximize))
        } else {
            let opponent = team_to_maximize.get_opponent();

            let iterator = TurnIterator::new(
                context,
                opponent
            );

            iterator
                .map(|turn| {
                    let mut new_context = context.apply_unsafely_copied(turn);
                    new_context.toggle_team();

                    // If flying is in the mix, we can't do the same depth
                    let can_anyone_fly = Team::Black.is_allowed_to_fly(&new_context.board) ||
                        Team::White.is_allowed_to_fly(&new_context.board);

                    let new_depth = match can_anyone_fly {
                        true =>  depth.saturating_sub(2),
                        false => depth - 1,
                    };

                    let (child, grade) = self.mini_max(
                        &opponent,
                        &new_context,
                        new_depth,
                        history
                    );

                    (turn, (child, -grade))
                })
                .max_by(|(_, (_, a_grade)), (_, (_, b_grade))| a_grade.total_cmp(b_grade))
                .map(|(parent, (turn, grade))| (Some(parent), grade))
                .unwrap_or(
                    // There's nothing we can do anymore
                    (None, context.board.get_result_for(team_to_maximize, history) as f32)
                )
        }
    }
}