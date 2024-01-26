use std::sync::{Arc, Mutex, RwLock};
use crate::datastructures::{BoardHistory, CanonicalBoardSet, GameBoard, Team, Turn};

pub trait Agent {
    fn get_next_move(
        team: Team,
        board: GameBoard,
        history: Arc<Mutex<impl BoardHistory + 'static>>,
        lost_states_for_white: Arc<RwLock<CanonicalBoardSet>>,
        won_states_for_black: Arc<RwLock<CanonicalBoardSet>>,
        num_invocations: usize
    ) -> Turn;
}