use std::fs;
use fnv::FnvHashSet;
use itertools::Itertools;
use crate::GameBoard;
use crate::datastructures::{Team, Phase, Encodable};
use crate::datastructures::game_board::{CanonicalGameBoard, UsefulGameBoard};
use crate::iterators::{ParentBoardIterator, ChildTurnIterator};

use super::lost_positions::all_lost_positions;

const MAX_NUM_PIECES_PER_TEAM: u8 = 3;


/**
 * Output one: LOST, Output two: WON
 */

//TODO: 
// -terminate after all 3v3 positions 
// -filter lost states : only by pieces taken, only 2 loser 3 winner stones 
// -change structure : input whole hash map in mark_lost / mark_won 
// -use profiler to determine expensive things (Flamegraph) 
//  
pub fn complete_search() -> (FnvHashSet<CanonicalGameBoard>, FnvHashSet<CanonicalGameBoard>) {
    let mut lost_states: FnvHashSet<CanonicalGameBoard> = FnvHashSet::default();
    let mut won_states: FnvHashSet<CanonicalGameBoard> = FnvHashSet::default();
    mark_lost(all_lost_positions(), Team::WHITE, &mut lost_states, &mut won_states);
    (lost_states, won_states)
}

fn mark_lost(states:  FnvHashSet<CanonicalGameBoard>, team: Team, lost_states: &mut FnvHashSet<CanonicalGameBoard>, won_states: &mut FnvHashSet<CanonicalGameBoard>) {
   if !states.is_empty() {
   let mut possible_won_states: FnvHashSet<CanonicalGameBoard> = FnvHashSet::default();
    for state in states.iter() {
        if state.get_num_pieces(Team::WHITE) <= MAX_NUM_PIECES_PER_TEAM && state.get_num_pieces(Team::BLACK) <= MAX_NUM_PIECES_PER_TEAM {
            if !lost_states.contains(state) {
                lost_states.insert(*state);
                for prev_state in ParentBoardIterator::new(team, *state) {
                    if prev_state.get_num_pieces(Team::WHITE) <= MAX_NUM_PIECES_PER_TEAM && prev_state.get_num_pieces(Team::BLACK) <= MAX_NUM_PIECES_PER_TEAM {
                        possible_won_states.insert(prev_state);
                    }
                }
            }
        }
    }
    eprintln!("executing mark_won, len of input hash:{}", possible_won_states.len());
    mark_won( possible_won_states, team.get_opponent(), lost_states, won_states);
    }
}

fn mark_won(states:  FnvHashSet<CanonicalGameBoard>, team: Team, lost_states: &mut FnvHashSet<CanonicalGameBoard>, won_states: &mut FnvHashSet<CanonicalGameBoard>) {
    if !states.is_empty() {
        let mut possible_lost_states: FnvHashSet<CanonicalGameBoard> = FnvHashSet::default();
        let mut prev_states = FnvHashSet::default();

        for state in states.iter() {
            if state.get_num_pieces(Team::WHITE) <= MAX_NUM_PIECES_PER_TEAM && state.get_num_pieces(Team::BLACK) <= MAX_NUM_PIECES_PER_TEAM {
                if !won_states.contains(state) {
                    won_states.insert(*state);

                    ParentBoardIterator::new(team, *state)
                        .for_each(|prev_state| { prev_states.insert(prev_state); });
                }
            }
        }

        for prev_state in prev_states {
            let mut child_iter = ChildTurnIterator::new(Phase::MOVE, team.get_opponent(), prev_state);
            if child_iter.all(|child_turn|
                {
                    let child_board = prev_state.apply(child_turn, team.get_opponent()).get_representative();
                    won_states.contains(&child_board)
                }) {
                possible_lost_states.insert(prev_state);
            }
        }

        eprintln!("executing mark_lost, input hash len:{}", possible_lost_states.len());
        mark_lost( possible_lost_states, team.get_opponent(), lost_states, won_states);
    }
}






#[test]

fn test_logic() {
   let case1 = GameBoard::decode(String::from("EEBEEEEWWEEEEEEBWEEEBEEE"));
   let case2 = GameBoard::decode(String::from("WEEEEEEEEEBEBEEEWEEWEEEB"));
   let case3 = GameBoard::decode(String::from("WBEEEEEEEEEEEEWEWEBEEEEB"));

    case1.print_board();
    case2.print_board();
    case3.print_board();

    let case1_can = case1.get_representative();
    let case2_can = case2.get_representative();
    let case3_can = case3.get_representative();


}

#[test]
fn test_3vs3() {
    let file_contents = fs::read_to_string("./tests/complete-search/3vs3/input_felder.txt")
        .expect("File could not be read");

    let mut boards = file_contents.split_terminator('\n');
    let mut actual: String = String::new();

    let (lost_states, won_states) = complete_search();

    while let Some(board) = boards.next() {
        let canonical_board = GameBoard::decode(String::from(board)).get_representative();
        let output_line = if lost_states.contains(&canonical_board){
            0
        } else if won_states.contains(&canonical_board){
            2
        } else {
            1
        };
        actual = format!("{actual}\n{output_line}");
    }

    let expected = fs::read_to_string("./tests/complete-search/3vs3/output.txt")
        .expect("File could not be read");
    assert_eq!(expected.trim(), actual.trim());
}