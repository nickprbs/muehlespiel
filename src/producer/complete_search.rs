use std::fs;
use std::sync::{Arc, Mutex, RwLock};
use rayon::prelude::*;
use crate::GameBoard;
use crate::datastructures::{Team, Phase, Encodable, CanonicalBoardSet, WonLostMap};
use crate::datastructures::game_board::UsefulGameBoard;
use crate::iterators::{ParentBoardIterator, ChildTurnIterator};

use super::lost_positions::all_lost_positions;

const MAX_NUM_PIECES_PER_TEAM: u8 = 5;


pub fn complete_search(
    lost_states: Arc<RwLock<WonLostMap>>,
    won_states: Arc<RwLock<WonLostMap>>,
) {
    let input = all_lost_positions();
    mark_lost(input, Team::WHITE, 0, lost_states, won_states);
}

fn mark_lost(
    mut states: CanonicalBoardSet,
    team: Team,
    distance_from_lost: u16,
    lost_states: Arc<RwLock<WonLostMap>>,
    won_states: Arc<RwLock<WonLostMap>>,
) {
    if !states.is_empty() {
        let mut possible_won_states = Mutex::new(CanonicalBoardSet::default());

        states.par_drain()
            .for_each(|state| {
                if state.get_num_pieces(Team::WHITE) <= MAX_NUM_PIECES_PER_TEAM && state.get_num_pieces(Team::BLACK) <= MAX_NUM_PIECES_PER_TEAM {
                    let newly_inserted = lost_states.write().unwrap().insert(state, distance_from_lost).is_none();

                    if newly_inserted {
                        for prev_state in ParentBoardIterator::new(team, state.clone()) {
                            if prev_state.get_num_pieces(Team::WHITE) <= MAX_NUM_PIECES_PER_TEAM && prev_state.get_num_pieces(Team::BLACK) <= MAX_NUM_PIECES_PER_TEAM {
                                possible_won_states.lock().unwrap().insert(prev_state);
                            }
                        }
                    }
                }
            });

        drop(states);

        eprintln!("executing mark_won, len of input hash:{}", possible_won_states.lock().unwrap().len());
        mark_won(possible_won_states.into_inner().unwrap(), team.get_opponent(), distance_from_lost + 1, Arc::clone(&lost_states), won_states);
    }
}

fn mark_won(
    mut states: CanonicalBoardSet,
    team: Team,
    distance_from_lost: u16,
    lost_states: Arc<RwLock<WonLostMap>>,
    won_states: Arc<RwLock<WonLostMap>>,
) {
    if !states.is_empty() {
        let mut prev_states = Mutex::new(CanonicalBoardSet::default());

        states.par_drain()
            .for_each(|state| {
                let newly_inserted = won_states.write().unwrap().insert(state.clone(), distance_from_lost).is_none();

                if newly_inserted {
                    let mut local_prev_states = CanonicalBoardSet::default();
                    ParentBoardIterator::new(team, state)
                        .for_each(|prev_state| {
                            if prev_state.get_num_pieces(Team::WHITE) <= MAX_NUM_PIECES_PER_TEAM && prev_state.get_num_pieces(Team::BLACK) <= MAX_NUM_PIECES_PER_TEAM {
                                local_prev_states.insert(prev_state);
                            }
                        });
                    prev_states.lock().unwrap().extend(local_prev_states);
                }
            });

        drop(states);

        let mut possible_lost_states = Mutex::new(CanonicalBoardSet::default());

        prev_states.lock().unwrap().par_drain()
            .for_each(|prev_state| {
                let mut child_iter = ChildTurnIterator::new(Phase::MOVE, team.get_opponent(), prev_state.clone());
                let readonly_won_states = won_states.read().unwrap();
                let all_children_are_winners = child_iter.all(|child_turn| {
                    let child_board = prev_state.apply(child_turn, team.get_opponent()).get_representative();
                    readonly_won_states.contains_key(&child_board)
                });
                drop(readonly_won_states);
                if all_children_are_winners {
                    possible_lost_states.lock().unwrap().insert(prev_state.clone());
                }
            });


        drop(prev_states);

        eprintln!("executing mark_lost, len of input hash:{}", possible_lost_states.lock().unwrap().len());
        mark_lost(possible_lost_states.into_inner().unwrap(), team.get_opponent(), distance_from_lost + 1, lost_states, Arc::clone(&won_states));
    }
}


#[test]
fn test_3vs3() {
    test_x_vs_x(3);
}

#[test]
fn test_4vs4() {
    test_x_vs_x(4);
}

#[test]
fn test_5vs5() {
    test_x_vs_x(5);
}

#[test]
fn test_6vs6() {
    test_x_vs_x(6);
}

fn test_x_vs_x(x: u8) {
    if MAX_NUM_PIECES_PER_TEAM < x {
        panic!("Max num pieces is too small. Please set to at least {}", x);
    }

    let file_contents = fs::read_to_string(format!("./tests/complete-search/{x}vs{x}/input_felder.txt"))
        .expect("File could not be read");

    let mut boards = file_contents.split_terminator('\n');
    let mut actual: String = String::new();

    let lost_states = Arc::new(RwLock::new(WonLostMap::default()));
    let won_states = Arc::new(RwLock::new(WonLostMap::default()));
    complete_search(Arc::clone(&lost_states), Arc::clone(&won_states));

    let lost_states = lost_states.read().unwrap();
    let won_states = won_states.read().unwrap();

    while let Some(board) = boards.next() {
        let board = GameBoard::decode(String::from(board));
        let canonical_board = board.get_representative();
        let inverted_canonical_board = board.invert_teams().get_representative();
        let output_line = if lost_states.contains_key(&canonical_board) {
            0
        } else if won_states.contains_key(&inverted_canonical_board) {
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