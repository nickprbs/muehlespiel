use crate::datastructures::{GameBoard, Team};
use crate::iterators::location_iterator::LocationIterator;
use crate::iterators::mill_iterator::MillIterator;

pub struct LostPositionsByPiecesTakenIterator {
    pub(crate) loosing_team: Team,
    pub(crate) piece_a_iterator: LocationIterator,
    pub(crate) locations_where_a_has_been: Vec<u8>,
    pub(crate) piece_b_iterator: LocationIterator,
    pub(crate) winner_mill_iterator: MillIterator,
}

impl LostPositionsByPiecesTakenIterator {
    pub fn new(loosing_team: Team) -> Self {
        todo!()
    }
}

impl Iterator for LostPositionsByPiecesTakenIterator {
    type Item = GameBoard;

    /**
    * select a location for a
    *   select a location for b (excluding locations_where_a_has_been)
    *     select a location for a mill
    *       foreach aux_num in 0..=6:
    *         select aux_num pieces on the field ""randomly""
    */
    fn next(&mut self) -> Option<Self::Item> {
        todo!()
        // TODO: Remember to also place 0 up to (9-3)=6 additional ;) opponent pieces in addition to mill
    }
}