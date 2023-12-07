use crate::iterators::TurnIterator;
use crate::structs::GamePhase::Moving;
use crate::structs::Team::{Black, White};
use crate::types::{GameBoard, GameContext};

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

    pub fn encode(&self) -> char {
        match self {
            Black => 'B',
            White => 'W',
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
        let cant_move = TurnIterator::new(&GameContext {
            board: board.clone(), team: *self, phase: Moving
        }, self.get_opponent()).count() == 0;

        self.count_pieces(board) <= 2 || cant_move
    }

    pub fn get_opponent(&self) -> Team {
        match self {
            Black => White,
            White => Black
        }
    }
}