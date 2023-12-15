use fnv::FnvHashSet;
use crate::datastructures::{GameBoard, Team};

pub fn lost_positions_by_pieces_taken(loosing_team: Team) -> FnvHashSet<GameBoard> {
    let output = FnvHashSet::default(); 
    let initial_board: GameBoard = [0,0,0]; 
    //initial buildup of all possible !canonical! boards where loser has two stones and winner has at 
    //least one mill 
    

    output  
}