mod by_pieces_taken;
mod by_cant_move;

pub use crate::producer::lost_positions::{
    by_cant_move::lost_positions_by_cant_move as lost_positions_by_cant_move,
    by_pieces_taken::lost_positions_by_pieces_taken as lost_positions_by_pieces_taken,
};