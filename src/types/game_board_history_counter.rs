use std::collections::HashMap;
use crate::types::GameBoard;

// TODO: Use hashmap with fast hashing function
pub type GameBoardHistoryCounter = HashMap<GameBoard, u16>;

/*impl GameBoardHistoryCounter {
    pub fn increment(&mut self, board: &GameBoard) {
        self.insert(board, self.get_value(board) + 1);
    }

    pub fn get_value(&self, board: &GameBoard) -> u16 {
        self.entry(board).unwrap_or(0)
    }
}*/