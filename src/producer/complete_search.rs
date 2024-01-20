use std::fs;
use fnv::FnvHashSet;
use itertools::Itertools;
use rayon::prelude::*;
use crate::GameBoard;
use crate::datastructures::{Team, Phase, Encodable};
use crate::datastructures::game_board::{CanonicalGameBoard, UsefulGameBoard};
use crate::iterators::{ParentBoardIterator, ChildTurnIterator};

use super::lost_positions::all_lost_positions;

const MAX_NUM_PIECES_PER_TEAM: u8 = 5;


/**
 * Output one: LOST, Output two: WON
 */

pub fn complete_search() -> (FnvHashSet<CanonicalGameBoard>, FnvHashSet<CanonicalGameBoard>) {
    let mut lost_states: FnvHashSet<CanonicalGameBoard> = FnvHashSet::default();
    let mut won_states: FnvHashSet<CanonicalGameBoard> = FnvHashSet::default();
    let input = all_lost_positions();

    mark_lost(input, Team::WHITE, &mut lost_states, &mut won_states);
    (lost_states, won_states)
}

fn mark_lost(states: FnvHashSet<CanonicalGameBoard>, team: Team, lost_states: &mut FnvHashSet<CanonicalGameBoard>, won_states: &mut FnvHashSet<CanonicalGameBoard>) {
    if !states.is_empty() {
        let mut possible_won_states: FnvHashSet<CanonicalGameBoard> = FnvHashSet::default();

        for state in states.iter() {
            if state.get_num_pieces(Team::WHITE) <= MAX_NUM_PIECES_PER_TEAM && state.get_num_pieces(Team::BLACK) <= MAX_NUM_PIECES_PER_TEAM {
                if lost_states.insert(*state) { // insert returns true if lost_states didn't contain state
                    for prev_state in ParentBoardIterator::new(team, *state) {
                        if prev_state.get_num_pieces(Team::WHITE) <= MAX_NUM_PIECES_PER_TEAM && prev_state.get_num_pieces(Team::BLACK) <= MAX_NUM_PIECES_PER_TEAM {
                            possible_won_states.insert(prev_state);
                        }
                    }
                }
            }
        }

        println!("executing mark_won, len of input hash:{}", possible_won_states.len());
        mark_won(possible_won_states, team.get_opponent(), lost_states, won_states);
    }
}

fn mark_won(states: FnvHashSet<CanonicalGameBoard>, team: Team, lost_states: &mut FnvHashSet<CanonicalGameBoard>, won_states: &mut FnvHashSet<CanonicalGameBoard>) {
    if !states.is_empty() {
        let mut prev_states = FnvHashSet::default();

        for state in states.iter() {
            if state.get_num_pieces(Team::WHITE) <= MAX_NUM_PIECES_PER_TEAM && state.get_num_pieces(Team::BLACK) <= MAX_NUM_PIECES_PER_TEAM {
                if won_states.insert(*state) { // insert returns true if won_states didn't contain state
                    ParentBoardIterator::new(team, *state)
                        .for_each(|prev_state| { prev_states.insert(prev_state); });
                }
            }
        }

        let possible_lost_states: FnvHashSet<CanonicalGameBoard> = prev_states.into_par_iter()
            .filter(|prev_state| {
                let mut child_iter = ChildTurnIterator::new(Phase::MOVE, team.get_opponent(), prev_state.clone());
                child_iter.all(|child_turn| {
                    let child_board = prev_state.apply(child_turn, team.get_opponent()).get_representative();
                    won_states.contains(&child_board)
                })
            })
            .collect();

        println!("executing mark_lost, len of input hash:{}", possible_lost_states.len());
        mark_lost(possible_lost_states, team.get_opponent(), lost_states, won_states);
    }
}


#[test]
fn test_3vs3() {
    test_x_vx_x(3);
}

#[test]
fn test_5vs5() {
    test_x_vx_x(5);
}

fn test_x_vx_x(x: u8) {
    if MAX_NUM_PIECES_PER_TEAM < x {
        panic!("Max num pieces is too small. Please set to at least {}", x);
    }

    let file_contents = fs::read_to_string(format!("./tests/complete-search/{x}vs{x}/input_felder.txt"))
        .expect("File could not be read");

    let mut boards = file_contents.split_terminator('\n');
    let mut actual: String = String::new();

    let (lost_states, won_states) = complete_search();

    while let Some(board) = boards.next() {
        let board = GameBoard::decode(String::from(board));
        let canonical_board = board.get_representative();
        let inverted_canonical_board = board.invert_teams().get_representative();
        let output_line = if lost_states.contains(&canonical_board) {
            0
        } else if won_states.contains(&inverted_canonical_board) {
            2
        } else {
            1
        };
        actual = format!("{actual}\n{output_line}");
    }

    let expected = fs::read_to_string(format!("./tests/complete-search/{x}vs{x}/output.txt"))
        .expect("File could not be read");
    assert_eq!(actual.trim(), expected.trim());
}