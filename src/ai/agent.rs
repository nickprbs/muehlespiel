use std::sync::{Arc, Mutex, RwLock};
use crate::datastructures::{BoardHistory, CanonicalBoardSet, GameBoard, Team, Turn, WonLostMap};

pub trait Agent {
    fn get_next_move(
        team: Team,
        board: GameBoard,
        history: Arc<Mutex<impl BoardHistory + 'static>>,
        lost_states_for_white: Arc<RwLock<WonLostMap>>,
        won_states_for_black: Arc<RwLock<WonLostMap>>,
        num_invocations: usize
    ) -> Turn;
}