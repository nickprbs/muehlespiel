use std::collections::HashMap;
use std::fmt::format;
use fnv::FnvBuildHasher;
use crate::agents::Agent;
use crate::agents::simple::SimplePlacingAgent;
use crate::iterators::TurnIterator;
use crate::structs::{GamePhase, Team, Turn};
use crate::types::game_board::QueryableGameBoard;
use crate::types::{GameBoard, GameBoardHistoryCounter, GameBoardHistoryMap, GameContext};

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
            (None, context.board.get_evaluation_for(team_to_maximize, history))
        } else if context.team.count_pieces(&context.board) <= 2 {
            (None, -1.0)
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
                    // There's nothing we can do anymore, return -1 since we lost because we can't do anything anymore
                    (None, -1.0)
                )
        }
    }
}

#[test]
fn test_one_step_finishes() {
    let empty_history: GameBoardHistoryMap = HashMap::with_hasher(FnvBuildHasher::default());
    // White only needs to move piece at north middle on inner ring cw, to enclose opponent
    let case = "M W EEEEEEEEEWWWWWWWWEBBBBBW";
    let mut state = GameContext::from_encoding(case);
    let actual_move = MinimaxAgent{}.get_next_turn(&state, &empty_history);
    state.apply_unsafely(actual_move);
    let expected_state = "EEEEEEEEEWWWWWWWEWBBBBBW";
    assert_eq!(state.board.encode(), expected_state);
}

#[test]
fn test_one_step_finish_by_taking() {
    let empty_history: GameBoardHistoryMap = HashMap::with_hasher(FnvBuildHasher::default());

    let case = "M W EEEBWEEWWBWEWEWEWEBEEEEE";
    let mut state = GameContext::from_encoding(case);
    let actual_move = MinimaxAgent{}.get_next_turn(&state, &empty_history);
    state.apply_unsafely(actual_move);
    let expected_states = ["WEEEWEEEWBWEWEWEWEBEEEEE", "WEEBWEEEWEWEWEWEWEBEEEEE", "WEEBWEEEWBWEWEWEWEEEEEEE"];

    expected_states.iter().for_each(|expected_state| {
        let board = GameBoard::from_encoding(expected_state);
        println!("Evaluation of expected state: {}, evaluation: {}, result: {}",
                 expected_state,
                 board.get_evaluation_for(&Team::Black, &empty_history),
            "Keks"//board.get_result_for(&Team::Black, &empty_history)
        )
    });
    println!("Evaluation of actual state: {}, evaluation: {}, result: {}",
        state.board.encode(),
        state.board.get_evaluation_for(&Team::Black, &empty_history),
        "Keks"//state.board.get_result_for(&Team::Black, &empty_history)
    );

    assert!(expected_states.contains(&&*state.board.encode()), "{}", format!("Not expected move, actual state: {}, took move: {}", state.board.encode(), actual_move.encode()));
}

#[test]
fn test_one_step_finish_by_taking_2() {
    let empty_history: GameBoardHistoryMap = HashMap::with_hasher(FnvBuildHasher::default());
    // White only needs to move piece at north middle on inner ring cw, to enclose opponent
    let case = "M W WBEEWEEWEWWEWEEBBWWWEEEE";
    let mut state = GameContext::from_encoding(case);
    let actual_move = MinimaxAgent{}.get_next_turn(&state, &empty_history);
    state.apply_unsafely(actual_move);
    let expected_states = ["WEEEWEEWEWWEWEEBBWWEWEEE", "WBEEWEEWEWWEWEEEBWWEWEEE", "WBEEWEEWEWWEWEEBEWWEWEEE"];
    assert!(expected_states.contains(&&*state.board.encode()), "{}", format!("Not expected move, actual state: {}, took move: {}", state.board.encode(), actual_move.encode()));
}