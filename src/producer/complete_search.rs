

use fnv::FnvHashSet;
use itertools::Itertools;
use crate::GameBoard;
use crate::datastructures::{Team, Phase, Encodable};
use crate::datastructures::game_board::{CanonicalGameBoard, UsefulGameBoard};
use crate::iterators::{ParentBoardIterator, ChildTurnIterator};

use super::lost_positions::{all_lost_positions, self};


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
    mark_lost( all_lost_positions(), Team::WHITE, &mut lost_states, &mut won_states);
    (lost_states, won_states)
}

fn mark_lost(states:  FnvHashSet<CanonicalGameBoard>, team: Team, lost_states: &mut FnvHashSet<CanonicalGameBoard>, won_states: &mut FnvHashSet<CanonicalGameBoard>) {
   if !states.is_empty() {
   let mut possible_won_states: FnvHashSet<CanonicalGameBoard> = FnvHashSet::default();
    for state in states.iter() {
        if state.get_total_stone_amount() <=6 {
            if !lost_states.contains(state) {
                lost_states.insert(*state);
                for prev_state in ParentBoardIterator::new(team, *state) {
                    //eprintln!("new parent state found"); 
                    //eprintln!("");
                    if prev_state.get_total_stone_amount() <= 6 {
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
    for state in states.iter() {
        if state.get_total_stone_amount() <= 6 {
            if !won_states.contains(state) {
                won_states.insert(*state);

                for prev_state in ParentBoardIterator::new(team, *state) {
                    let mut child_iter = ChildTurnIterator::new(Phase::MOVE, team.get_opponent(), prev_state);
                    if child_iter.all(|child_turn|
                        {
                            let mut child_board = prev_state.apply(child_turn, team.get_opponent());
                            if child_board != *state {
                                 child_board = child_board.get_representative();
                            }

                            //if won_states.contains(&GameBoard::decode(String::from("EEBEEEEWWEEEEEEBWEEEBEEE")).get_representative()){
                            //    eprintln!("won states contains 1st canonical!");
                            //} else if won_states.contains(&GameBoard::decode(String::from("WEEEEEEEEEBEBEEEWEEWEEEB")).get_representative()){
                            //    eprintln!("won states contains 2nd canonical!");
                            //} else if won_states.contains(&GameBoard::decode(String::from("WBEEEEEEEEEEEEWEWEBEEEEB")).get_representative()){
                            //    eprintln!("won states contains 3rd canonical!") 
                            //}
                            won_states.contains(&child_board)  
                        }) {
                        possible_lost_states.insert(prev_state);
                    }
                }
            }
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