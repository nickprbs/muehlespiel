use std::fs;
use std::sync::{Arc, Mutex, RwLock};
use fnv::FnvHashSet;
use itertools::Itertools;
use rayon::prelude::*;
use crate::GameBoard;
use crate::datastructures::{Team, Phase, Encodable};
use crate::datastructures::game_board::{CanonicalGameBoard, UsefulGameBoard};
use crate::iterators::{ParentBoardIterator, ChildTurnIterator};

use super::lost_positions::all_lost_positions;

const MAX_NUM_PIECES_PER_TEAM: u8 = 8;


/**
 * Output one: LOST, Output two: WON
 */

pub fn complete_search(
    lost_states: Arc<RwLock<FnvHashSet<CanonicalGameBoard>>>,
    won_states: Arc<RwLock<FnvHashSet<CanonicalGameBoard>>>,
) {
    let input = all_lost_positions();
    mark_lost(Arc::new(Mutex::new(input)), Team::WHITE, lost_states, won_states);
}

fn mark_lost(
    states: Arc<Mutex<FnvHashSet<CanonicalGameBoard>>>,
    team: Team,
    lost_states: Arc<RwLock<FnvHashSet<CanonicalGameBoard>>>,
    won_states: Arc<RwLock<FnvHashSet<CanonicalGameBoard>>>,
) {
    let states = states.lock().unwrap();
    if !states.is_empty() {
        let possible_won_states: Arc<Mutex<FnvHashSet<CanonicalGameBoard>>> = Arc::new(Mutex::new(FnvHashSet::default()));

        states.par_iter()
            .for_each(|state| {
                if state.get_num_pieces(Team::WHITE) <= MAX_NUM_PIECES_PER_TEAM && state.get_num_pieces(Team::BLACK) <= MAX_NUM_PIECES_PER_TEAM {
                    if !lost_states.read().unwrap().contains(state) {
                        lost_states.write().unwrap().insert(state.clone());

                        for prev_state in ParentBoardIterator::new(team, state.clone()) {
                            if prev_state.get_num_pieces(Team::WHITE) <= MAX_NUM_PIECES_PER_TEAM && prev_state.get_num_pieces(Team::BLACK) <= MAX_NUM_PIECES_PER_TEAM {
                                possible_won_states.lock().unwrap().insert(prev_state);
                            }
                        }
                    }
                }
            });

        eprintln!("executing mark_won, len of input hash:{}", possible_won_states.lock().unwrap().len());
        mark_won(Arc::clone(&possible_won_states), team.get_opponent(), Arc::clone(&lost_states), won_states);
    }
}

fn mark_won(
    states: Arc<Mutex<FnvHashSet<CanonicalGameBoard>>>,
    team: Team,
    lost_states: Arc<RwLock<FnvHashSet<CanonicalGameBoard>>>,
    won_states: Arc<RwLock<FnvHashSet<CanonicalGameBoard>>>,
) {
    let states = states.lock().unwrap();
    if !states.is_empty() {
        let mut prev_states: Arc<Mutex<FnvHashSet<CanonicalGameBoard>>> = Arc::new(Mutex::new(FnvHashSet::default()));

        states.par_iter()
            .for_each(|state| {
                if state.get_num_pieces(Team::WHITE) <= MAX_NUM_PIECES_PER_TEAM && state.get_num_pieces(Team::BLACK) <= MAX_NUM_PIECES_PER_TEAM {
                    if !won_states.read().unwrap().contains(state) {
                        won_states.write().unwrap().insert(state.clone());

                        ParentBoardIterator::new(team, state.clone())
                            .for_each(|prev_state| {
                                prev_states.lock().unwrap().insert(prev_state);
                            });
                    }
                }
            });

        let possible_lost_states: Arc<Mutex<FnvHashSet<CanonicalGameBoard>>> = Arc::new(Mutex::new(FnvHashSet::default()));

        prev_states.lock().unwrap().par_iter()
            .for_each(|prev_state| {
                let mut child_iter = ChildTurnIterator::new(Phase::MOVE, team.get_opponent(), prev_state.clone());
                let x = child_iter.all(|child_turn| {
                    let child_board = prev_state.apply(child_turn, team.get_opponent()).get_representative();
                    won_states.read().unwrap().contains(&child_board)
                });
                if x {
                    possible_lost_states.lock().unwrap().insert(prev_state.clone());
                }
            });

        eprintln!("executing mark_lost, len of input hash:{}", possible_lost_states.lock().unwrap().len());
        mark_lost(possible_lost_states, team.get_opponent(), lost_states, Arc::clone(&won_states));
    }
}


#[test]
fn test_3vs3() {
    test_x_vx_x(3);
}

#[test]
fn test_4vs4() {
    test_x_vx_x(4);
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

    let lost_states = Arc::new(RwLock::new(FnvHashSet::default()));
    let won_states = Arc::new(RwLock::new(FnvHashSet::default()));
    complete_search(Arc::clone(&lost_states), Arc::clone(&won_states));

    let lost_states = lost_states.read().unwrap();
    let won_states = won_states.read().unwrap();

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