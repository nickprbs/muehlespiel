use super::{GameBoard, game_board::UsefulGameBoard};

pub struct BoardEquivalenceClassIterator {
    board: Option<GameBoard>,
    previous_board: Option<GameBoard>,
    flipped: bool,
    mirrored: bool,
    rotated: u8
}

impl BoardEquivalenceClassIterator {
    pub fn new(board: GameBoard) -> Self {
        Self {
            board: Some(board),
            previous_board: None,
            flipped: false,
            mirrored: false,
            rotated: 0
        }
    }
}

impl Iterator for BoardEquivalenceClassIterator {
    type Item = GameBoard;

    fn next(&mut self) -> Option<Self::Item> {
        self.previous_board = self.board;

        if let None = self.board {
            return None;
        }

        return match self.board {
            None => None,
            Some(board) => {
                if self.flipped {
                    if self.rotated == 3 {
                        if self.mirrored {
                            // We've reached the end of all possible symmetries
                            self.board = None;
                        } else {
                            // Not mirrored
                            self.board = Some(board.mirrored().flipped().rotated(1));
                            self.mirrored = true;
                            self.flipped = false;
                            self.rotated = 0;
                        }
                    } else {
                        // Not fully rotated
                        self.board = Some(board.flipped().rotated(1));
                        self.flipped = false;
                        self.rotated += 1;
                    }
                } else {
                    // Not flipped
                    self.board = Some(board.flipped());
                    self.flipped = true;
                }
                self.previous_board
            }
        }
    }
}