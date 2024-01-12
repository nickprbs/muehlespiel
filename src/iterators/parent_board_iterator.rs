use std::collections::HashSet;
use std::default;

use fnv::FnvHashMap;
use itertools::Itertools;
use crate::datastructures::{GameBoard, Location, Phase, Team, Turn, TurnAction, Encodable};
use crate::datastructures::game_board::{UsefulGameBoard, CanonicalGameBoard};

pub struct ParentBoardIterator {
    opponent_team: Team,
    current_board: GameBoard, 
    occupied_locations: Vec<Location>,
    own_locations: Vec<Location>,
    opponent_locations: Vec<Location>,
    unchecked_opponent_locations: Vec<Location>,
    current_stone: Option<Location>, 
    prev_move_iterator: PreviousMoveIterator, 


}

impl Iterator for ParentBoardIterator {
    //Turns as the item type may be impractical, becuase we will need to "replace" already taken stones when going back up.
    //This is not pracitical with the Turn-format especially when we are in the Move-phase. 
    type Item = CanonicalGameBoard;
    
    fn next(&mut self) -> Option<Self::Item> {
       
        match self.prev_move_iterator.next() {
            Some(board) => return Some(board),
            //we can go to the next stone
            None => {
                //we made it through all opponent stones! => finished
                if self.unchecked_opponent_locations.len() == 0 {
                    return None 
                //we need to start the PreviousMoveIterator again with the next stone    
                } else {
                self.current_stone = self.unchecked_opponent_locations.pop();     
                self.prev_move_iterator = PreviousMoveIterator::new(self.occupied_locations.clone(), self.own_locations.clone(),
                self.opponent_locations.clone(), self.current_stone.clone(),  self.current_board ); 
                return self.next() 
                } 
            }
        }
    }
}

impl ParentBoardIterator {
 pub(crate) fn new(current_team: Team, current_board: GameBoard) -> Self {
    let opponent_team = current_team.get_opponent(); 
    let own_locations = current_board.get_piece_locations(current_team); 
    let opponent_locations = current_board.get_piece_locations(current_team.get_opponent()); 
    let mut occupied_locations = own_locations.clone(); 
    occupied_locations.append(&mut opponent_locations.clone()); 
    let mut unchecked_opponent_locations: Vec<Location> = opponent_locations.clone(); 
    let current_stone = unchecked_opponent_locations.pop(); 
    let prev_move_iterator = PreviousMoveIterator::new(occupied_locations.clone(), own_locations.clone(),
    opponent_locations.clone(), current_stone.clone(), current_board );
    Self {
        opponent_team,
        current_board,
        occupied_locations,
        own_locations,
        opponent_locations,
        unchecked_opponent_locations,
        current_stone,
        prev_move_iterator,
    }

 }

 
}


//Iterates over all possible previous moves of a single location 
struct PreviousMoveIterator {
    occupied_locations: Vec<Location>,
    own_locations: Vec<Location>,
    opponent_locations: Vec<Location>,
    current_stone: Location,
    board: GameBoard,
    mill_flag: bool, 
    lookup: FnvHashMap<CanonicalGameBoard, bool> 
} 

impl Iterator for PreviousMoveIterator {
    type Item = CanonicalGameBoard;

    fn next(&mut self) -> Option<Self::Item> {
        let team = self.board.get_team_at(self.current_stone);
        match team {
            // no possible previous moves on an empty field
            None => return None, 
            Some(_) => {}
        }
        let team = team.unwrap(); 
        let free_neighbours = self.board.get_free_neighbours(self.current_stone);

        // if the opponent got a mill at the given location, there are more possible previous states
        if self.mill_flag {
            //locations where the current stone could have been
            for neighbour in free_neighbours {
                let valid_free_fields: Vec<Location> = (1..=24).into_iter().filter(|position| (!self.occupied_locations.contains(position))
                                                     && (*position != self.current_stone) && (*position !=neighbour)).collect_vec();
                // if closed => stone taken anywhere without mill
                 for ghost_field in valid_free_fields {
                    let temp_move = Turn { 
                        action : TurnAction::Place { location: ghost_field },
                        take_from: None 
                    };
                    let mut temp_board: GameBoard = self.board.clone().apply(temp_move, team.get_opponent());
                    let temp_opp_location_vec: Vec<Location> = temp_board.get_piece_locations(team.get_opponent()); 
                    let temp_own_location_vec: Vec<Location> = temp_board.get_piece_locations(team);
                    let (black_locations, white_locations) = match team {
                        Team::BLACK => (temp_own_location_vec.clone(), temp_opp_location_vec.clone()),
                        Team::WHITE => (temp_opp_location_vec.clone(), temp_own_location_vec.clone())
                    }; 
                    if temp_board.has_only_mills(team.get_opponent()) || 
                    //has no mills
                     !temp_opp_location_vec.into_iter().any(|position| temp_board.is_mill_at(position, &black_locations, &white_locations)) {
                        //we need to move the current stone "back" to build the valid parent gameboard
                        let second_temp_move = Turn {
                            action: TurnAction::Move { from: self.current_stone, to: neighbour },
                            take_from: None
                        }; 
                        temp_board = temp_board.apply(second_temp_move, team); 
                        let temp_board: CanonicalGameBoard = temp_board.get_representative(); 
                        if self.lookup.contains_key(&temp_board) {
                            continue;
                        } else {
                            self.lookup.insert(temp_board, true); 
                            return Some(temp_board) 
                        }
                        // case 2: other player had at least one mill but not only mills 
                        // => we cant take any stone => we cant build any parent board
                    } else {
                        if temp_board.is_mill_at(ghost_field, &black_locations, &white_locations) {
                            continue;
                        } else {
                            let second_temp_move = Turn {
                                action: TurnAction::Move { from: self.current_stone, to: neighbour },
                                take_from: None
                            }; 
                            temp_board = temp_board.apply(second_temp_move, team); 
                            let temp_board: CanonicalGameBoard = temp_board.get_representative(); 
                            if self.lookup.contains_key(&temp_board) {
                                continue;
                            } else {
                                self.lookup.insert(temp_board, true); 
                                return Some(temp_board) 
                            }
                        }
                    }
                 }
                }
        } 
        //no mill => opponent could only move in his turn
        else {
            for neighbour in free_neighbours {
                let temp_turn = Turn {
                    action: TurnAction::Move { from: self.current_stone, to: neighbour },
                    take_from: None
                }; 
                let temp_board = self.board.clone().apply(temp_turn, team); 
                let temp_board: CanonicalGameBoard = temp_board.get_representative();
                if self.lookup.contains_key(&temp_board) {
                    continue; 
                } else {
                    self.lookup.insert(temp_board, true); 
                    return Some(temp_board)
                }
            }
        }
        None
    }
}

impl PreviousMoveIterator {
    fn new (input_occupied_locations: Vec<Location>, input_own_locations: Vec<Location>, input_opponent_locations: Vec<Location>, 
            input_current_stone: Option<Location>, input_board: GameBoard) -> Self{
                let mut lookup_hash: FnvHashMap<CanonicalGameBoard, bool> = FnvHashMap::default(); 
        Self {
            occupied_locations: input_occupied_locations,
            own_locations : input_own_locations,
            opponent_locations: input_opponent_locations,
            current_stone: input_current_stone.unwrap(),
            board: input_board,
            mill_flag : input_board.is_mill_at(input_current_stone.unwrap(), &input_board.get_piece_locations(Team::BLACK), 
            &input_board.get_piece_locations(Team::WHITE)),
            lookup : lookup_hash,
        }        
    }

    
}


#[test]
fn test_parent_iterator () {
    let case1: GameBoard = GameBoard::decode(String::from("EEWWBBBEEEWWBBBWWEWBWBWE")); 
    let iter = ParentBoardIterator::new(Team::BLACK, case1); 
    let input_own_locations = case1.get_piece_locations(Team::BLACK); 
    let input_opponent_locations = case1.get_piece_locations(Team::WHITE);
    let mut input_occupied_locations = input_own_locations.clone(); 
    for elem in input_opponent_locations.iter() {
        input_occupied_locations.push(*elem); 
    }
    let smalliter= PreviousMoveIterator::new(input_occupied_locations, input_own_locations, input_opponent_locations, Some(11), case1); 
    assert_eq!(smalliter.collect_vec().len(), 5); 
}