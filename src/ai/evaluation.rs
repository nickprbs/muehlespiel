use itertools::Itertools;
use crate::datastructures::{Encodable, GameBoard, Phase, Team};
use crate::datastructures::game_board::UsefulGameBoard;
use crate::iterators::ChildTurnIterator;

pub fn evaluate_position(
    team_to_eval: Team,
    phase: Phase,
    board: GameBoard,
    depth: u16
) -> f32 {
    let opponent_won = board.has_lost(team_to_eval);
    let we_won = board.has_lost(team_to_eval.get_opponent());
    // Don't handle ties, since they will be caught beforehand in minimax

    let result = if we_won {
        2.0 + (1.0 / (depth + 1) as f32)
    } else if opponent_won {
        1.0 - (1.0 / (depth + 1) as f32)
    } else {
        1.0 + evaluate_non_done_position(
            team_to_eval,
            phase,
            board,
        )
    };

    debug_assert!(result <= 3.0);
    debug_assert!(result >= 0.0);

    result
}

const STONE_COUNT_FACTOR: f32 = 0.5;
const FLY_BONUS_FACTOR: f32 = 0.01;
const NUM_MOVES_FACTOR: f32 = 1.0 - STONE_COUNT_FACTOR- FLY_BONUS_FACTOR;

fn evaluate_non_done_position(
    team_to_eval: Team,
    phase: Phase,
    board: GameBoard
) -> f32 {
    let own_stone_count = board.get_num_pieces(team_to_eval);
    let opponent_stone_count = board.get_num_pieces(team_to_eval.get_opponent());
    let can_i_fly = own_stone_count == 3;

    let stone_fraction = own_stone_count as f32 / 9.0;
    debug_assert!(stone_fraction <= 1.0);
    debug_assert!(stone_fraction >= 0.0);

    let fly_bonus = match can_i_fly {
        true => {
            if opponent_stone_count <= 4 { 1.0 } else { 0.0 }
        },
        false => 0.0
    };

    // Determine whether we are close to being locked in place
    // Upper limit is the most moves we could possibly make
    let upper_limit_of_moves = match can_i_fly {
        true => own_stone_count * (24 - own_stone_count - opponent_stone_count),
        false => own_stone_count * 4, // four for the number of directions in which we could possibly move. Overestimates a lot.
    } as f32;
    let actual_number_of_moves = ChildTurnIterator::new(phase, team_to_eval, board)
        .map(|turn| turn.action)
        .dedup()
        .count() as f32;
    let moves_fraction = actual_number_of_moves / upper_limit_of_moves;

    STONE_COUNT_FACTOR * stone_fraction + NUM_MOVES_FACTOR * moves_fraction + FLY_BONUS_FACTOR * fly_bonus
}

#[test]
fn test_evaluating_wins() {
    // Test different depths
    let case_white_won = GameBoard::decode(String::from("BBWWWEEEEEEEEEEEEEEEEEEE"));
    assert!(
        evaluate_position(Team::WHITE, Phase::MOVE, case_white_won, 1) > evaluate_position(Team::WHITE, Phase::MOVE, case_white_won, 2)
    );

    // Test won vs non-won
    let case_black_won = GameBoard::decode(String::from("BBBWWEEEEEEEEEEEEEEEEEEE"));
    assert!(
        evaluate_position(Team::WHITE, Phase::MOVE, case_white_won, 1) > evaluate_position(Team::WHITE, Phase::MOVE, case_black_won, 1)
    )
}