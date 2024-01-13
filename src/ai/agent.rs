use std::sync::{Arc, Mutex};
use crate::datastructures::{BoardHistory, GameBoard, Team, Turn};

pub trait Agent {
    fn get_next_move(
        team: Team,
        board: GameBoard,
        history: Arc<Mutex<impl BoardHistory + 'static>>,
        num_invocations: usize
    ) -> Turn;
}