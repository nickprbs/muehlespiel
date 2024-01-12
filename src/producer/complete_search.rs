use std::collections::HashMap;
use std::default;
use fnv::{FnvBuildHasher, FnvHashSet};
use itertools::Itertools;
use crate::datastructures::{GameBoard, Location, Team, Phase};
use crate::datastructures::game_board::{CanonicalGameBoard, UsefulGameBoard};
use crate::iterators::{ParentBoardIterator, ChildTurnIterator};

use super::lost_positions::all_lost_positions;



/**
 * Output one: LOST, Output two: WON
 */

//TODO: 
// -filter lost states : only by pieces taken, only 2 loser 3 winner stones 
// -change structure : input whole hash map in mark_lost / mark_won 
// -use profiler to determine expensive things (Flamegraph) 
//  
pub fn complete_search () -> (FnvHashSet<CanonicalGameBoard>, FnvHashSet<CanonicalGameBoard>){
    let mut lost_states: FnvHashSet<CanonicalGameBoard>= all_lost_positions(); 
    let mut won_states: FnvHashSet<CanonicalGameBoard> = FnvHashSet::default(); 
    for state in lost_states.clone() {
        mark_lost(state, Team::WHITE, &mut lost_states, &mut won_states);

    }
    for state in won_states.clone() {
        mark_won(state, Team::WHITE, &mut lost_states, &mut won_states); 
    }
    (lost_states,won_states) 
}

fn mark_lost(state: CanonicalGameBoard, team: Team, lost_states: &mut FnvHashSet<CanonicalGameBoard>, won_states: &mut FnvHashSet<CanonicalGameBoard>  ) {
    if !lost_states.contains(&state) {
        lost_states.insert(state); 
        for z in ParentBoardIterator::new(team, state) {
            mark_won(z, team.get_opponent(), lost_states, won_states);
        }
    }
} 

fn mark_won(state: CanonicalGameBoard, team:Team, lost_states: &mut FnvHashSet<CanonicalGameBoard>, won_states: &mut FnvHashSet<CanonicalGameBoard>) {
    if ! won_states.contains(&state){
        won_states.insert(state);
        for z in ParentBoardIterator::new(team, state){
            if ChildTurnIterator::new(Phase::MOVE, team.get_opponent(), z).dedup()
            .all(|child_turn| {
                let child: GameBoard = z.apply(child_turn, team.get_opponent()); 
                let canonical = child.get_representative(); 
                won_states.contains(&canonical)}
            ) {
                mark_lost(z, team, lost_states, won_states)
            }
        }
    }
}