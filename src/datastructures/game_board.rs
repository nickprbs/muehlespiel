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
    // swaps inner and outer ring 
    fn flipped(&self) -> GameBoard {
        let mut output = self.clone();
        output.swap(0, 2);
        output 
    }
    // rotates 90° clockwise times '_increments' 
    fn rotated(&self, _increments: u8) -> GameBoard {
           let mut output = self.clone();  
        let iterations = _increments % 4; 
        for _i in 1..=iterations{
            let old_board = output;
            let mut counter=0;
            for elem in old_board{
                output[counter]=(elem >> 4) | (elem << (16 - 4));
                counter +=1
            }
        }
        output
    }
    // performs a mirroring with the 90° mirror-axis. 
    fn mirrored(&self) -> GameBoard {
        let old_num = self.clone(); 
        let mut output: [u16 ; 3] = [0, 0, 0 ];
        for i in 0..3 {
            let temp_old_num: u16 = old_num[i]; 
            let mut output_num: u16 =0;
            for j in 0..8 {
                let higher_bitmask = temp_old_num & 2_u16.pow(15-2*j); 
                let lower_bitmask: u16 = temp_old_num & 2_u16.pow(15-(2*j+1));
                let higher_bit: u16 = higher_bitmask.count_ones() as u16; 
                let lower_bit: u16 = lower_bitmask.count_ones() as u16; 
                if j == 0 {
                    output_num += higher_bitmask + lower_bitmask; 
                } else if j == 4 {
                    output_num += higher_bitmask + lower_bitmask; 
                } else {
                    output_num += higher_bit * 2_u16.pow(2*j-1) + lower_bit* 2_u16.pow(2*j-2);
                }
            }
            output[i]=output_num;
        }
        output 
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

#[test]
fn test_mirroring() {
    let _case0 =        [
                0b0000000000000000,
                0b0000000000000000,
                0b0000000000000000
            ];
    let _case1 =             [
        0b1000101001010010,
        0b0010011010011010,
        0b0000011001001000
    ];

    let _case2 =             [
        0b0100100100100100,
        0b1001001001001001,
        0b0010010010010010
    ];

    assert_eq!(_case0.mirrored() , [0b0000000000000000,0b0000000000000000,0b0000000000000000]);
    assert_eq!(_case1.mirrored(), [0b1010000101101000,0b0010100110100110,0b0000100001100100]);
    assert_eq!(_case2.mirrored(), [0b0100011000011000,0b1001100001100001, 0b0010000110000110]);

}

#[test]
fn test_rotating() {
    let _case0 =[
                0b0000000000000000,
                0b0000000000000000,
                0b0000000000000000
            ];
    let _case1 =[
        0b1000101001010010,
        0b0010011010011010,
        0b0000011001001000
    ];

    let _case2 =             [
        0b0100100100100100,
        0b1001001001001001,
        0b0010010010010010
    ];
    assert_eq!(_case0.rotated(1) , [0b0000000000000000,0b0000000000000000,0b0000000000000000]);
    assert_eq!(_case1.rotated(1), [0b0010100010100101, 0b1010001001101001, 0b1000000001100100]);
    assert_eq!(_case2.rotated(1), [0b0100010010010010, 0b1001100100100100, 0b0010001001001001]);

    assert_eq!(_case0.rotated(2) , [0b0000000000000000,0b0000000000000000,0b0000000000000000]);
    assert_eq!(_case1.rotated(2), [0b0101001010001010, 0b1001101000100110, 0b0100100000000110]);
    assert_eq!(_case2.rotated(2), [0b0010010001001001, 0b0100100110010010, 0b1001001000100100]);

    assert_eq!(_case0.rotated(3) , [0b0000000000000000,0b0000000000000000,0b0000000000000000]);
    assert_eq!(_case1.rotated(3), [0b1010010100101000, 0b0110100110100010, 0b0110010010000000]);
    assert_eq!(_case2.rotated(3), [0b1001001001000100, 0b0010010010011001, 0b0100100100100010]);

    assert_eq!(_case0.rotated(4) , _case0); 
    assert_eq!(_case1.rotated(4), _case1);
    assert_eq!(_case2.rotated(4), _case2 );
}