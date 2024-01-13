use std::sync::{Arc, Mutex};
use crate::datastructures::{BoardHistory, GameBoard, Phase, Team, Turn};

pub trait Agent {
    fn get_next_move(phase: Phase, team: Team, board: GameBoard, history: Arc<Mutex<impl BoardHistory + 'static>>) -> Turn;
}