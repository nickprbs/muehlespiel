use super::{GameBoard, game_board::UsefulGameBoard};

pub struct BoardEquivalenceClassIterator {
    board: GameBoard,
    flip: bool,
    mirror: bool,
    rotations: u8
}

impl BoardEquivalenceClassIterator {
    pub fn new(board: GameBoard) -> Self {
        Self {
            board,
            flip: false,
            mirror: false,
            rotations: 0
        }
    }
}

impl Iterator for BoardEquivalenceClassIterator {
    type Item = GameBoard;

    // TODO: Remember old board, so that we don't have to compute all actions, but only one at a time
    fn next(&mut self) -> Option<Self::Item> {
        // Check if we already reached the end of all possible symmetries
        if self.flip && self.mirror && self.rotations == 3 {
            return None;
        }

        // Assemble the new board
        let board = self.board;
        if self.flip {
            board = board.flipped();
        }
        if self.mirror {
            board = board.mirrored();
        }
        board = board.rotated(self.rotations);

        // Increment the values
        if self.rotations == 3 {
            if self.flip { // When flip was true, we also had false => do with mirror now
                self.mirror = true;
            }
            self.flip = !self.flip;
        } else {
            self.rotations = (self.rotations + 1) % 4;
        }

        Some(board)
    }
}