use std::mem::size_of;
use crate::datastructures::Turn;

use super::{Encodable, BoardEquivalenceClassIterator};

pub type GameBoard = [u16; 3];
pub type CanonicalGameBoard = GameBoard;

#[test]
fn test_data_structure_size() {
    let size = size_of::<GameBoard>();
    println!("Size of GameBoard: {}", size);
    assert!(size <= 7); // Requirement in exercise sheet
}

pub trait UsefulGameBoard {

    fn apply(&self, turn: Turn) -> GameBoard;
    fn unapply(&self, turn: Turn) -> Box<dyn Iterator<Item=GameBoard>>;

    fn flipped(&self) -> GameBoard;
    fn rotated(&self, increments: u8) -> GameBoard;
    fn mirrored(&self) -> GameBoard;

    // Get all 16 equivalent fields (including this one)
    fn get_equivalence_class(&self) -> Box<dyn Iterator<Item=GameBoard>>;

    // Whether this board can be represented by the other through symmetries
    fn is_equivalent_to(&self, other: GameBoard) -> bool;

    // Get a unique and constant game board that represents this game board's equivalence class (one of 16)
    // That representative is determined by comparing the gameboard arrays and getting the lowest one
    // (first comparing the number of the outer most ring, if equal: the middle, if equal: the inner)
    fn get_representative(&self) -> CanonicalGameBoard;
}

impl UsefulGameBoard for GameBoard {
    fn apply(&self, _turn: Turn) -> GameBoard {
        todo!()
    }

    fn unapply(&self, _turn: Turn) -> Box<dyn Iterator<Item=GameBoard>> {
        todo!()
    }

    // TODO: flipped, rotated, mirrored von Nick und Jan

    fn get_equivalence_class(&self) -> Box<dyn Iterator<Item=GameBoard>> {
        Box::new(
            BoardEquivalenceClassIterator::new(*self)
        )
    }

    fn is_equivalent_to(&self, other: GameBoard) -> bool {
        self.get_equivalence_class()
            .any(|equal_board| equal_board == other)
    }

    // Get an unique representative by pretending like we concatenated all three values of a game
    // board. Then, compare those in the equivalence class and return the smallest by concatenated
    // number.
    fn get_representative(&self) -> CanonicalGameBoard {
        self.get_equivalence_class()
            .min_by(|board_a, board_b| {
                // Compare the two boards by first comparing their first ring, then second, then third
                board_a[0].cmp(&board_b[0]).then(
                    board_a[1].cmp(&board_b[1]).then(
                        board_a[2].cmp(&board_b[2])
                    )
                )
            })
            .expect("None found in equivalence class")
    }

    fn flipped(&self) -> GameBoard {
        todo!()
    }

    fn rotated(&self, _increments: u8) -> GameBoard {
        todo!()
    }

    fn mirrored(&self) -> GameBoard {
        todo!()
    }
}

impl Encodable for GameBoard {
    //decodes a String into a 'game_board' type. Convention: 2bits per field: '00' <=> Empty, '01' <=> Black, '10' <=> White 
    fn decode(string: String) -> Self {
        let mut char_counter : u16 = 0;
        let mut outter_ring_num : u16 = 0;
        let mut middle_ring_num : u16 =0;
        let mut inner_ring_num: u16 =0; 
        for single_char in string.chars() {
            
            let current_exponent: u16 = 14- (char_counter % 8)*2;  
            let mut lower_bit: u16 = 0;
            let mut higher_bit: u16 =0;
            match single_char {
                'E' => {}
                'W' => {higher_bit = 1;}
                'B' => {lower_bit = 1;}
                _ => {panic!("Error parsing String: Invalid Token found.");}
            }
            if char_counter < 8 {
                outter_ring_num += lower_bit*2_u16.pow(current_exponent as u32) + higher_bit*2_u16.pow((current_exponent+1) as u32);
            } else if char_counter >= 8 && char_counter < 16 {
                middle_ring_num += lower_bit*2_u16.pow(current_exponent as u32) + higher_bit*2_u16.pow((current_exponent+1) as u32);
            } else if char_counter >= 16 && char_counter< 24 {
                inner_ring_num += lower_bit*2_u16.pow(current_exponent as u32) + higher_bit*2_u16.pow((current_exponent+1) as u32);
            } else {panic!("Invalid Format! String has to be 24 characters long!");}
            char_counter +=1;
        }
        [outter_ring_num, middle_ring_num, inner_ring_num]
    }    
    
    fn encode(&self) -> String {
        let whole_num: String =format!("{:016b}{:016b}{:016b}", self[0], self[1],self[2]);
        let mut output: String = String::new();
        for i in 0..24 {
            let higher_bit: char = whole_num.chars().nth(2*i).unwrap();
            let lower_bit: char =whole_num.chars().nth(2*i+1).unwrap();
            let field_char: char;
            if higher_bit == '0' && lower_bit == '0' {
                 field_char = 'E';
            } else if higher_bit == '0' && lower_bit == '1' {
                 field_char = 'B';
            } else if higher_bit == '1' && lower_bit == '0' {
                 field_char = 'W';
            } else {
                panic!("Error encoding Gameboard: Invalid binary representation found!");
            }
            output = output + &field_char.to_string();
        }
        output
    }
}

#[test]
fn test_decoding() {
    let cases = [
        (
            "EEEEEEEEEEEEEEEEEEEEEEEE",
            [
                0b0000000000000000,
                0b0000000000000000,
                0b0000000000000000,
            ]
        ),
        (
            "WEWWBBEWEWBWWBWWEEBWBEWE",
            [
                0b1000101001010010,
                0b0010011010011010,
                0b0000011001001000,
            ]
        ),
        (
            "BEWBEWBEWBEWBEWBEWBEWBEW",
            [
                0b0100100100100100,
                0b1001001001001001,
                0b0010010010010010
            ]
        )
    ];

    cases.iter()
        .for_each(|case| {
            assert_eq!(case.1, GameBoard::decode(String::from(case.0)));
        });
}

#[test]
fn test_encoding(){
    let cases = [
        (
            "EEEEEEEEEEEEEEEEEEEEEEEE",
            [
                0b0000000000000000,
                0b0000000000000000,
                0b0000000000000000,
            ]
        ),
        (
            "WEWWBBEWEWBWWBWWEEBWBEWE",
            [
                0b1000101001010010,
                0b0010011010011010,
                0b0000011001001000,
            ]
        ),
        (
            "BEWBEWBEWBEWBEWBEWBEWBEW",
            [
                0b0100100100100100,
                0b1001001001001001,
                0b0010010010010010
            ]
        )
    ];
    cases.iter()
        .for_each(|case| {
            assert_eq!(String::from(case.0), GameBoard::encode(&case.1));
        });
}