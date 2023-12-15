use itertools::Itertools;
use crate::datastructures::{GameBoard, Location, Team};
use crate::iterators::n_range_locations_iterator::NRangeLocationsIterator;
use crate::iterators::neighbours_iterator::NeighboursIterator;

pub struct LostPositionsByCantMoveIterator {
    loosing_team: Team,
    our_piece_locations: NRangeLocationsIterator,
    free_locations: Vec<Location>,
    auxilary_opponent_piece_locations: Option<NRangeLocationsIterator>,
}

impl LostPositionsByCantMoveIterator {
    pub fn new(loosing_team: Team) -> Self {
        let free_fields: Vec<Location> = (1..=24).collect();
        Self {
            loosing_team,
            our_piece_locations: NRangeLocationsIterator::new(4, 9, free_fields.clone()), // 3 pieces cant be locked down, so start at 4
            free_locations: free_fields,
            auxilary_opponent_piece_locations: None,
        }
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
     **/
    fn next(&mut self) -> Option<Self::Item> {
        // This is only the case at first start
        if let None = self.auxilary_opponent_piece_locations {
        }

        // TODO: let next_aux_configuration = self.auxilary_opponent_piece_locations.next();

        let our_next_locations = self.our_piece_locations.next();
        match our_next_locations {
            None => return None,
            Some(our_next_locations) => {
                let neighbours: Vec<Location> = NeighboursIterator::new(our_next_locations)
                    .unique()
                    .collect();
                let max_left_opponent_pieces: i8 = 9_i8 - neighbours.len();

                if max_left_opponent_pieces < 0 {
                    // Opponent does not have enough pieces to lock us. Skip this config.
                    return self.next();
                } else {
                    // Opponent does have enough pieces to lock us
                    let free_fields = (1..=24)
                        .filter(|loc| !our_next_locations.contains(loc) && !neighbours.contains(loc))
                        .collect();
                    self.auxilary_opponent_piece_locations = Some(NRangeLocationsIterator::new(0, max_left_opponent_pieces as u8, free_fields));
                    todo!();
                }
            }
        }

        // match next_aux_configuration {

        //}
        return None;
    }
}