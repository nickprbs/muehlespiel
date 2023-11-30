use crate::{Location, Piece, repeat_alignment};
use crate::structs::Team;
use crate::types::GameBoard;

pub struct MillGroup([Location; 3]);

impl MillGroup {

    pub fn all_groups_that_might_contain(location: &Location) -> Vec<MillGroup> {
        let mut groups: Vec<MillGroup> = Vec::new();

        let Location { ring: ring_index, alignment: alignment_index } = *location;

        if location.alignment % 2 == 0 {
            // Location is on a leading line
            // Add the inward/outward mill
            groups.push(MillGroup([
                Location { ring: 0, alignment: alignment_index },
                Location { ring: 1, alignment: alignment_index },
                Location { ring: 2, alignment: alignment_index },
            ]));
            // Add the across-ring mill
            groups.push(MillGroup([
                Location { ring: ring_index, alignment: repeat_alignment(alignment_index as i16 - 1) },
                *location,
                Location { ring: ring_index, alignment: repeat_alignment(alignment_index as i16 + 1) },
            ]));
        } else {
            // Location is in a corner
            groups.push(MillGroup([
                Location { ring: ring_index, alignment: repeat_alignment(alignment_index as i16 - 2) },
                Location { ring: ring_index, alignment: repeat_alignment(alignment_index as i16 - 1) },
                *location,
            ]));
            groups.push(MillGroup([
                *location,
                Location { ring: ring_index, alignment: repeat_alignment(alignment_index as i16 + 1) },
                Location { ring: ring_index, alignment: repeat_alignment(alignment_index as i16 + 2) },
            ]));
        }

        groups
    }

    // Returns all pieces that are in this mill group
    fn intersecting_pieces(&self, board: &GameBoard) -> Vec<Piece> {
        board.iter()
            .filter(|piece| self.0.contains(&piece.location))
            .copied()
            .collect()
    }

    // Counts all pieces in this group and returns if enough
    pub fn is_complete(&self, board: &GameBoard, current_team: &Team) -> bool {
        let intersecting_pieces = self.intersecting_pieces(board);

        let number_of_pieces = intersecting_pieces.iter()
            .filter(|piece| piece.owner == *current_team)
            .count();
        
        number_of_pieces == 3
    }
}