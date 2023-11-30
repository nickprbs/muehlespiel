use crate::constants::TOTAL_NUMBER_FIELDS;
use crate::Piece;
use crate::structs::{Location, Team};
use crate::types::game_board::QueryableGameBoard;
use crate::types::GameBoard;

pub struct TakablesIterator {
    board: GameBoard,
    opponent: Team,
    current_target_location_id: u8,
    all_pieces_in_mill: bool,
}

impl TakablesIterator {
    pub fn new(
        board: GameBoard,
        opponent: Team
    ) -> TakablesIterator {
        let all_pieces_in_mill = board.iter()
            .filter(|piece| piece.owner == opponent)
            .all(|piece| {
                board.is_in_complete_mill(&piece.location, &opponent)
            });

        TakablesIterator {
            board,
            opponent,
            current_target_location_id: 0,
            all_pieces_in_mill
        }
    }
}

impl Iterator for TakablesIterator {
    type Item = Piece;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let location = Location::from_enumeration_id(self.current_target_location_id);
            let piece = self.board.get_piece_at_location(&location);
            // Check that there is a piece and that it's an opponents one
            if let Some(piece) = piece {
                if piece.owner == self.opponent {
                    if self.all_pieces_in_mill {
                        self.current_target_location_id += 1;
                        return Some(piece);
                    } else {
                        // Check that it is not a part of a mill
                        let not_in_complete_mill = !self.board
                            .is_in_complete_mill(&location, &self.opponent);

                        if not_in_complete_mill {
                            self.current_target_location_id += 1;
                            return Some(piece);
                        }
                    }
                }
            } else {
                if self.current_target_location_id >= TOTAL_NUMBER_FIELDS - 1 {
                    return None;
                }
            }

            self.current_target_location_id += 1;
        }
    }
}