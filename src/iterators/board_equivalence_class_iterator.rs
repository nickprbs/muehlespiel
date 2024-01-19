use crate::datastructures::game_board::UsefulGameBoard;
use crate::datastructures::GameBoard;

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
                            self.board = Some(board.mirrored().flipped().rotated());
                            self.mirrored = true;
                            self.flipped = false;
                            self.rotated = 0;
                        }
                    } else {
                        // Not fully rotated
                        self.board = Some(board.flipped().rotated());
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

#[test]
fn test_equivalence_count() {
    let board1: GameBoard = [0, 0, 0];
    let count1 = BoardEquivalenceClassIterator::new(board1)
        .count();
    let board2: GameBoard = [0b0110001001100010, 0b0110001001100010, 0b1001001010100010];
    let count2 = BoardEquivalenceClassIterator::new(board2)
        .count();
    assert_eq!(count1, 16);
    assert_eq!(count2, 16);
}