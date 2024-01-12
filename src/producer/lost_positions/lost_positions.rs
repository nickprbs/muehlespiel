use std::collections::HashSet;

use fnv::FnvHashSet;

use crate::{producer::lost_positions::{lost_positions_by_cant_move, lost_positions_by_pieces_taken}, datastructures::game_board::CanonicalGameBoard}; 


pub fn all_lost_positions() -> FnvHashSet<CanonicalGameBoard> {
    let lost_by_cant_move_map = lost_positions_by_cant_move();
    let mut lost_by_pieces_taken_map = lost_positions_by_pieces_taken();
    for position in lost_by_cant_move_map.iter(){
        lost_by_pieces_taken_map.insert(*position); 
    }
    lost_by_pieces_taken_map 
}