use crate::structs::Team::{Black, White};
use crate::types::GameBoard;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Team {
    Black,
    White
}

impl Team {

    pub fn from_encoding(character: char) -> Self {
        match character {
            'B' => Black,
            'W' => White,
            _   => panic!("Unexpected character received for team")
        }
    }

    pub fn get_char_symbol(&self) -> char {
        match self {
            Black => '●',
            White => '○'
        }
    }

    pub fn count_pieces(&self, board: &GameBoard) -> usize {
        board.iter()
            .filter(|&piece| &piece.owner == self)
            .count()
    }

    pub fn is_allowed_to_fly(&self, board: &GameBoard) -> bool {
        self.count_pieces(board) <= 3
    }

    pub fn is_defeated(&self, board: &GameBoard) -> bool {
        self.count_pieces(board) <= 2
    }

    pub fn get_opponent(&self) -> Team {
        match self {
            Black => White,
            White => Black
        }
    }
}