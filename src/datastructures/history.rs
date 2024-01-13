use fnv::FnvHashMap;
use crate::datastructures::GameBoard;

pub type BoardHistoryMap = FnvHashMap<GameBoard, u8>;

pub trait BoardHistory: Send {
    fn increment(&mut self, board: GameBoard);
    fn get_value(&self, board: GameBoard) -> u8;
    fn will_be_tie(&self, board: GameBoard) -> bool;
    // Here, we can flush the history, because we will never have the same configs again if we take a piece.
    fn took_a_piece(&mut self);
}

impl BoardHistory for BoardHistoryMap {
    fn increment(&mut self, board: GameBoard) {
        self.insert(board, self.get_value(board) + 1);
    }

    fn get_value(&self, board: GameBoard) -> u8 {
        *self.get(&board).unwrap_or(&0)
    }

    fn will_be_tie(&self, board: GameBoard) -> bool {
        self.get_value(board) >= 2
    }

    fn took_a_piece(&mut self) {
        self.clear();
    }
}