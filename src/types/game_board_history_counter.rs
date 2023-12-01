use fnv::FnvHashMap;
use crate::types::game_board::QueryableGameBoard;
use crate::types::GameBoard;

pub type GameBoardHistoryMap = FnvHashMap<String, u8>;

pub trait GameBoardHistoryCounter {
    fn increment(&mut self, board: &GameBoard);
    fn get_value(&self, board: &GameBoard) -> u8;
    fn is_third_time(&self, board: &GameBoard) -> bool;
    // Here, we can flush the history, because we will never have the same configs again if we take a piece.
    fn removed_piece(&mut self);
}

impl GameBoardHistoryCounter for GameBoardHistoryMap {
    fn increment(&mut self, board: &GameBoard) {
        self.insert(board.encode(), self.get_value(board) + 1);
    }

    fn get_value(&self, board: &GameBoard) -> u8 {
        *self.get(&*board.encode()).unwrap_or(&0)
    }

    fn is_third_time(&self, board: &GameBoard) -> bool {
        self.get_value(board) >= 2
    }

    fn removed_piece(&mut self) {
        self.clear();
    }
}