use crate::datastructures::{GameBoard, Team};
use crate::iterators::lost_positions_by_cant_move_iterator::LostPositionsByCantMoveIterator;
use crate::iterators::lost_positions_by_pieces_taken_iterator::LostPositionsByPiecesTakenIterator;

pub struct LostPositionsIterator {
    lost_by_cant_move_iterator: LostPositionsByCantMoveIterator,
    lost_by_pieces_taken_iterator: LostPositionsByPiecesTakenIterator,
    // Remember that the first iterator ended, so that we don't need to call it all the time once we don't need it anymore
    is_first_iterator_done: bool
}

impl LostPositionsIterator {
    fn new(loosing_team: Team) -> Self {
        Self {
            lost_by_pieces_taken_iterator: LostPositionsByPiecesTakenIterator {
                loosing_team,
                piece_a_iterator: todo!(),
                locations_where_a_has_been: vec![],
                piece_b_iterator: todo!(),
                winner_mill_iterator: todo!(),
            },
            lost_by_cant_move_iterator: LostPositionsByCantMoveIterator { loosing_team },
            is_first_iterator_done: false
        }
    }
}

impl Iterator for LostPositionsIterator {
    type Item = GameBoard;

    fn next(&mut self) -> Option<Self::Item> {
        return if !self.is_first_iterator_done {
            self.lost_by_cant_move_iterator.next().or({
                self.is_first_iterator_done = true;
                self.lost_by_pieces_taken_iterator.next()
            })
        } else {
            self.lost_by_pieces_taken_iterator.next()
        }
    }
}