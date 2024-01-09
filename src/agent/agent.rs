use crate::datastructures::{GameBoard, Phase, Team, Turn};

pub trait Agent {
    fn get_next_move(phase: Phase, team: Team, board: GameBoard, history: ()) -> Turn;
}