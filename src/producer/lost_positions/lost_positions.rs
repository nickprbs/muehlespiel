use crate::{producer::lost_positions::{lost_positions_by_cant_move, lost_positions_by_pieces_taken}, datastructures::game_board::CanonicalGameBoard};
use crate::datastructures::CanonicalBoardSet;


pub fn all_lost_positions() -> CanonicalBoardSet {
    let lost_by_cant_move_map = lost_positions_by_cant_move();
    let mut lost_by_pieces_taken_map = lost_positions_by_pieces_taken();
    for position in lost_by_cant_move_map.iter() {
        lost_by_pieces_taken_map.insert(*position); 
    }

    eprintln!("Enumerated all lost positions");

    lost_by_pieces_taken_map
}