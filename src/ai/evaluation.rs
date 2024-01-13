use crate::datastructures::{Encodable, GameBoard, Team};
use crate::datastructures::game_board::UsefulGameBoard;

pub fn evaluate_position(
    team_to_eval: Team,
    board: GameBoard,
    depth: u16
) -> f32 {
    let opponent_won = board.has_lost(team_to_eval);
    let we_won = board.has_lost(team_to_eval.get_opponent());
    // Don't handle ties, since they will be caught beforehand in minimax

    let result = if we_won {
        1.0 + (1.0 / depth as f32)
    } else if opponent_won {
        0.0
    } else {
        evaluate_non_done_position(
            team_to_eval,
            board,
            depth
        )
    };

    result
}

const STONE_COUNT_FACTOR: f32 = 0.5;
const NUM_MOVES_FACTOR: f32 = 1.0 - STONE_COUNT_FACTOR;

fn evaluate_non_done_position(
    team_to_eval: Team,
    board: GameBoard,
    depth: u16
) -> f32 {
    let stone_count = board.get_num_pieces(team_to_eval);
    let stone_fraction = stone_count as f32 / 9.0;

    STONE_COUNT_FACTOR * stone_fraction + NUM_MOVES_FACTOR * 0.0
}

#[test]
fn test_evaluating_wins() {
    // Test different depths
    let case_white_won = GameBoard::decode(String::from("BBWWWEEEEEEEEEEEEEEEEEEE"));
    assert!(
        evaluate_position(Team::WHITE, case_white_won, 1) > evaluate_position(Team::WHITE, case_white_won, 2)
    );

    // Test won vs non-won
    let case_black_won = GameBoard::decode(String::from("BBBWWEEEEEEEEEEEEEEEEEEE"));
    assert!(
        evaluate_position(Team::WHITE, case_white_won, 1) > evaluate_position(Team::WHITE, case_black_won, 1)
    )
}