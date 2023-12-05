use super::GameBoard;

pub struct BoardEquivalenceClassIterator {
    board: GameBoard
}

impl BoardEquivalenceClassIterator {
    pub fn new(board: GameBoard) -> Self {
        Self {
            board
        }
    }
}

impl Iterator for BoardEquivalenceClassIterator {
    type Item = GameBoard;

    fn next(&mut self) -> Option<Self::Item> {

    }
}