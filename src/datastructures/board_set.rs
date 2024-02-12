use crate::datastructures::game_board::CanonicalGameBoard;

pub type CanonicalBoardSet = hashbrown::HashSet<CanonicalGameBoard>;

pub type WonLostMap = hashbrown::HashMap<CanonicalGameBoard, u16>;