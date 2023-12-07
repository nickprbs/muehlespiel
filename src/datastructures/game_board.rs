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
    fn apply(&self, turn: Turn) -> GameBoard {
        todo!()
    }

    fn unapply(&self, turn: Turn) -> Box<dyn Iterator<Item=GameBoard>> {
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
}

impl Encodable for GameBoard {
    // Nick
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