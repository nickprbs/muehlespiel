use std::sync::{Arc, Mutex, RwLock};
use fnv::FnvHashSet;
use crate::datastructures::{BoardHistory, GameBoard, Team, Turn};
use crate::datastructures::game_board::CanonicalGameBoard;

pub trait Agent {
    fn get_next_move(
        team: Team,
        board: GameBoard,
        history: Arc<Mutex<impl BoardHistory + 'static>>,
        lost_states_for_white: Arc<RwLock<FnvHashSet<CanonicalGameBoard>>>,
        won_states_for_black: Arc<RwLock<FnvHashSet<CanonicalGameBoard>>>,
        num_invocations: usize
    ) -> Turn;
}