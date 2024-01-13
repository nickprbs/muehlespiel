use fnv::FnvHashSet;
use itertools::Itertools;
use crate::datastructures::{Team, Phase};
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
<<<<<<< HEAD
pub fn complete_search() -> (FnvHashSet<CanonicalGameBoard>, FnvHashSet<CanonicalGameBoard>) {
    let mut lost_states: FnvHashSet<CanonicalGameBoard> = FnvHashSet::default();
    let mut won_states: FnvHashSet<CanonicalGameBoard> = FnvHashSet::default();
    mark_lost(&mut all_lost_positions(), Team::WHITE, &mut lost_states, &mut won_states);
    (lost_states, won_states)
}

fn mark_lost(states: &mut FnvHashSet<CanonicalGameBoard>, team: Team, lost_states: &mut FnvHashSet<CanonicalGameBoard>, won_states: &mut FnvHashSet<CanonicalGameBoard>) {
    for state in states.clone() {
        if !lost_states.contains(&state) {
            lost_states.insert(state);
            if lost_states.len() > 10 {
                for lost in lost_states.iter() {
                    //dbg!(lost.encode());
                }
                // break;
            }
            let mut possible_won_states: FnvHashSet<CanonicalGameBoard> = FnvHashSet::default();
            for prev_state in ParentBoardIterator::new(team, state) {
=======
pub fn complete_search () -> (FnvHashSet<CanonicalGameBoard>, FnvHashSet<CanonicalGameBoard>){
    let mut lost_states: FnvHashSet<CanonicalGameBoard>= FnvHashSet::default(); 
    let mut won_states: FnvHashSet<CanonicalGameBoard> = FnvHashSet::default(); 
    
    let mut lost_white_positions: FnvHashSet<CanonicalGameBoard> = all_lost_positions(); 
    let mut lost_black_positions: FnvHashSet<CanonicalGameBoard> = FnvHashSet::default(); 
    for lost_state in lost_white_positions.iter() {
        let inverted_board = lost_state.invert_teams(); 
        lost_black_positions.insert(inverted_board); 
    }
    eprintln!("lost black finished!");
    mark_lost(&mut lost_white_positions, Team::WHITE, &mut lost_states, &mut won_states);
    mark_won(&mut lost_black_positions, Team::WHITE, &mut lost_states, &mut won_states); 
    (lost_states,won_states) 
}

fn mark_lost(states: &mut FnvHashSet<CanonicalGameBoard>, team: Team, lost_states: &mut FnvHashSet<CanonicalGameBoard>, won_states: &mut FnvHashSet<CanonicalGameBoard>  ) {
    let mut possible_won_states: FnvHashSet<CanonicalGameBoard> = FnvHashSet::default(); 
    for state in states.clone() {
        if !lost_states.contains(&state) {
            lost_states.insert(state); 
            for prev_state in ParentBoardIterator::new(team, state){
>>>>>>> 5a5a316 (reverse search updates)
                if prev_state.get_total_stone_amount() <= 6 {
                    possible_won_states.insert(prev_state);
                }
            }
<<<<<<< HEAD
            mark_won(&mut possible_won_states, team.get_opponent(), lost_states, won_states);
        }
    }
}
=======
        }
    }
    mark_won(&mut possible_won_states, team.get_opponent(), lost_states, won_states); 
} 
>>>>>>> 5a5a316 (reverse search updates)

fn mark_won(states: &mut FnvHashSet<CanonicalGameBoard>, team: Team, lost_states: &mut FnvHashSet<CanonicalGameBoard>, won_states: &mut FnvHashSet<CanonicalGameBoard>) {
    let mut possible_lost_states: FnvHashSet<CanonicalGameBoard> = FnvHashSet::default();
    for state in states.clone() {
<<<<<<< HEAD
        if state.get_total_stone_amount() <= 6 {
            if !won_states.contains(&state) {
                won_states.insert(state);
                if won_states.len() > 10 {
                    for won in won_states.iter() {
                        //dbg!(won.encode());
                    }
                    // break;
                }

                let mut previous_states: FnvHashSet<CanonicalGameBoard> = FnvHashSet::default();

                ParentBoardIterator::new(team, state).for_each(|parent_board| {
                    previous_states.insert(parent_board);
                });

                for prev_state in previous_states {
                    if ChildTurnIterator::new(Phase::MOVE, team.get_opponent(), prev_state).all(|child_turn|
                        {
                            let child_board = prev_state.apply(child_turn, team.get_opponent());
                            let child_board = child_board.get_representative();
                            won_states.contains(&child_board)
                        }) {
                        possible_lost_states.insert(prev_state);
                    }
                }
                mark_lost(&mut possible_lost_states, team.get_opponent(), lost_states, won_states);
            }
        }
    }
=======
        if state.get_total_stone_amount() <=6 {
        if !won_states.contains(&state){
            won_states.insert(state); 
            for prev_state in ParentBoardIterator::new(team, state) {
                
               if ChildTurnIterator::new(Phase::MOVE, team.get_opponent(), prev_state).all(|child_turn|
                {
                    let child_board = prev_state.apply(child_turn, team.get_opponent()); 
                    let child_board = child_board.get_representative(); 
                    won_states.contains(&child_board)
                }) {
                    
                    possible_lost_states.insert(prev_state); 
                } 
            }
        } 
        }
    }
    mark_lost(&mut possible_lost_states, team.get_opponent(), lost_states, won_states); 
>>>>>>> 5a5a316 (reverse search updates)
}