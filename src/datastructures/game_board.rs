use std::str::FromStr;

type GameBoard = [u16; 3];

trait UsefulGameBoard {

    fn apply(&self, turn: Turn) -> GameBoard;
    fn unapply(&self, turn: Turn) -> Iter<GameBoard>;

    fn flipped(&self) -> GameBoard;
    fn rotated(&self, increments: u8) -> GameBoard;
    fn mirrored(&self) -> GameBoard;

    // Get all 16 equivalent fields (including this one)
    fn get_equivalence_class(&self) -> Iter<GameBoard>;

    // Whether this board can be represented by the other through symmetries
    fn is_equivalent_to(&self, other: GameBoard) -> bool;

    // Get a unique and constant game board that represents this game board's equivalence class (one of 16)
    // That representative is determined by comparing the gameboard arrays and getting the lowest one
    // (first comparing the number of the outer most ring, if equal: the middle, if equal: the inner)
    fn get_representative(&self) -> GameBoard;
}

impl UsefulGameBoard for GameBoard {
    fn apply(&self, turn: Turn) -> GameBoard {
        todo!()
    }

    fn unapply(&self, turn: Turn) -> GameBoardIterator {
        todo!()
    }

    fn get_representative(&self) -> GameBoard {
        self.get_equivalence_class()
            .iter()
            .min_by(|board_a, board_b| {
                todo!()
            })
    }
}