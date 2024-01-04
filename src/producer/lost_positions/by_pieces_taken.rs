use std::collections::HashSet;
use std::hash::BuildHasherDefault;

use fnv::FnvHashSet;
use itertools::Itertools;
use crate::datastructures::{Encodable, GameBoard, game_board::CanonicalGameBoard, game_board::UsefulGameBoard,};
use crate::iterators::{NRangeLocationsIterator, MillIterator};

pub fn lost_positions_by_pieces_taken() -> FnvHashSet<GameBoard> {
    let mut output = FnvHashSet::default(); 
    let initial_looser_positions_hash = all_initial_looser_positions(); 
    let with_just_mills_hash = with_just_mills(initial_looser_positions_hash); 
    //first set of outputs: 2 looser stones and one mill from the winner
    with_just_mills_hash.clone().into_iter().for_each(|board: CanonicalGameBoard|{
        output.insert(board); 
    });
    //for each output board: all combinations of possible boards where looser has 2 stones and winner has at least one mill and 
    // 9 stones at most
    for mill_board in with_just_mills_hash.iter(){
        //get all empty fields on specific board (there are already stones placed)
        let mut free_fields_vec: Vec<u8> = Vec::new();
        for i in 1..=24 {
            if is_canonical_free_at(*mill_board, i as u8){
                free_fields_vec.push(i as u8); 
            }
        }
        //creation of NRangeLocationIterator
        let _all_other_stones_iter = NRangeLocationsIterator::new(1,6, free_fields_vec); 
        _all_other_stones_iter.for_each(|additional_winner_stones|{
            //buildup of new possible canonical gameboard
            let temp_gameboard: GameBoard = GameBoard::from_pieces(additional_winner_stones, Vec::new());
            let new_board: GameBoard = [(temp_gameboard[0] | mill_board[0] ),(temp_gameboard[1] | mill_board[1] ),
            (temp_gameboard[2] | mill_board[2] ) ]; 
            let new_canonical: CanonicalGameBoard = new_board.get_representative();
            //input in final hash if not present there
            if !output.contains(&new_canonical){
                output.insert(new_canonical); 
            }
        });
    }
    output  
}

//initial buildup of all possible !canonical! boards where loser has two stones and winner has at 
//least one mill 
fn all_initial_looser_positions () -> FnvHashSet<CanonicalGameBoard> {
    let mut second_temp_hash: HashSet<CanonicalGameBoard, BuildHasherDefault<fnv::FnvHasher>> = FnvHashSet::default(); 
    let free_fields_vec = (1..=24).collect_vec();
    let second_stone_iter = NRangeLocationsIterator::new(2,2, free_fields_vec);
    second_stone_iter.for_each(|looser_locations|{
            let configuration: GameBoard = GameBoard::from_pieces(Vec::new(), looser_locations); 
            let canonical: CanonicalGameBoard = configuration.get_representative(); 
            if !second_temp_hash.contains(&canonical) {
                second_temp_hash.insert(canonical); 
            }
        });
    second_temp_hash
}

fn with_just_mills (two_stones_hash: FnvHashSet<CanonicalGameBoard>) -> FnvHashSet<CanonicalGameBoard> {
    let mut with_just_mills_hash: HashSet<CanonicalGameBoard, BuildHasherDefault<fnv::FnvHasher>> = FnvHashSet::default(); 
    for _two_stone_board in two_stones_hash.iter() {
        let mut mill_iter = MillIterator::new();
        while let Some(single_mill_position) = mill_iter.next() {
           if is_canonical_free_at(*_two_stone_board, single_mill_position[0]) && 
            is_canonical_free_at(*_two_stone_board, single_mill_position[1]) &&
            is_canonical_free_at(*_two_stone_board, single_mill_position[2]) {
                let mill_board: GameBoard = GameBoard::from_pieces(single_mill_position.to_vec(),Vec::new());
                let new_board: GameBoard = [(_two_stone_board[0] | mill_board[0] ),(_two_stone_board[1] | mill_board[1] ),
                    (_two_stone_board[2] | mill_board[2] ) ]; 
                let canonical_mill: CanonicalGameBoard = new_board.get_representative(); 
                if !with_just_mills_hash.contains(&canonical_mill) {
                    with_just_mills_hash.insert(canonical_mill); 
                }
            }
        }
    }
    with_just_mills_hash 
} 


fn is_canonical_free_at(canonical_board: CanonicalGameBoard, position:u8) -> bool {
    let mut output = false; 
    if position > 24 || position < 1 {
        panic!("incorrect index");
    } else {
        let gameboard_string = GameBoard::encode(&canonical_board); 
        if gameboard_string.chars().nth((position-1) as usize).unwrap() == 'E' {
            output = true; 
        }
    }
    output 
}


#[test] 
fn test_first_two_patterns() {
    let test_hash = all_initial_looser_positions();
    assert_eq!(30, test_hash.len());
}

#[test]
fn test_with_just_mills() {
    let test_hash = all_initial_looser_positions();
    let mill_hash = with_just_mills(test_hash); 
    println!("{}", mill_hash.len());
    let mut second_temp_hash: HashSet<CanonicalGameBoard, BuildHasherDefault<fnv::FnvHasher>> = FnvHashSet::default(); 
    let free_fields_vec = (1..=24).collect_vec();
    let second_stone_iter = NRangeLocationsIterator::new(5,5, free_fields_vec);
    second_stone_iter.for_each(|looser_locations|{
            let configuration: GameBoard = GameBoard::from_pieces(Vec::new(), looser_locations); 
            let canonical: CanonicalGameBoard = configuration.get_representative(); 
            if !second_temp_hash.contains(&canonical) {
                second_temp_hash.insert(canonical); 
            }
        });
    println!("{}", second_temp_hash.len()); 
    assert!(second_temp_hash.len() >= mill_hash.len());
}

#[test]
pub fn test_lost_positions_by_piece_taken() {
    let final_hash = lost_positions_by_pieces_taken();
    println!("{}", final_hash.len()); 
}