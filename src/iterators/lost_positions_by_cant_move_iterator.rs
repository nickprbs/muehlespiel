use crate::datastructures::{GameBoard, Team};

pub struct LostPositionsByCantMoveIterator {
    pub(crate) loosing_team: Team
}

impl LostPositionsByCantMoveIterator {
    pub fn new(loosing_team: Team) -> Self {
        todo!()
    }
}

impl Iterator for LostPositionsByCantMoveIterator {
    type Item = GameBoard;

    /**
    * We can't move anymore if all neighbouring fields of our pieces are occupied.
    * Start by having more than three stones (it's not possible to lock if can fly)
    * Idea is: For each of our stones, count how many free neighbouring fields we have.
    * Subtract that count from 9. If the result is negative, we have found a configuration, that is impossible to loose in.
    * If it's positive, place opponent pieces on all free neighbouring fields, and place excess ""randomly"" on the field.
    *
    * foreach own_num in 4..=9:
    *   ...
    **/
    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}