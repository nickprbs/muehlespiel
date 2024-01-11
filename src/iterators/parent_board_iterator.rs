use itertools::Itertools;
use crate::datastructures::{GameBoard, Location, Phase, Team, Turn, TurnAction};
use crate::datastructures::game_board::{UsefulGameBoard, CanonicalGameBoard};

pub struct ParentBoardIterator {
    occupied_locations: Vec<Location>,
    own_locations: Vec<Location>,
    opponent_locations: Vec<Location>,
    processed_opponent_stones: Vec<Location>,
    current_stone: Location, 
    curr_stone_previous_possibilities : u8,
    current_stone_move_vec: Vec<(Location, Location, Option<Location> )>, // Vec<(From, To, Place position)>


}

impl Iterator for ParentBoardIterator {
    //Turns as the item type may be impractical, becuase we will need to "replace" already taken stones when going back up.
    //This is not pracitical with the Turn-format especially when we are in the Move-phase. 
    type Item = CanonicalGameBoard;
    
    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

impl ParentBoardIterator {
 pub(crate) fn new(current_team: Team, current_board: GameBoard) -> Self {
    todo!()
 }

 
}