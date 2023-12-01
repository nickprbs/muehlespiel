use crate::constants::TOTAL_NUMBER_FIELDS;
use crate::iterators::MoveActionIterator;
use crate::Piece;
use crate::structs::{Location, MillGroup, Team};
use crate::types::GameBoardHistoryCounter;

pub type GameBoard = Vec<Piece>;

pub trait QueryableGameBoard {
    fn from_encoding(string: &str) -> GameBoard {
        string.chars()
            // Add numbers, so that we can use Location::from_enumeration_id later
            .enumerate()
            // Only look at those fields that are occupied
            .filter(|(_, field)| field == &'B' || field == &'W')
            // Convert to pieces
            .map(|(index, field)|
                Piece {
                    location: Location::from_enumeration_id((index) as u8),
                    owner: Team::from_encoding(field)
                }
            )
            .collect()
    }

    fn encode(&self) -> String;

    fn is_location_occupied(&self, location: &Location) -> bool;

    fn get_piece_at_location(&self, location: &Location) -> Option<Piece>;

    fn add_piece_at_location(&mut self, location: &Location, team: &Team);

    fn remove_piece_at_location(&mut self, location: Location);

    fn move_piece(&mut self, src_location: Location, target_location: &Location, team: &Team);

    fn is_in_complete_mill(&self, location: &Location, current_team: &Team) -> bool;

    fn count_pieces_of(&self, team: &Team) -> u8;

    fn print(&self) {
        let a = self.get_char_at_location(&Location::from_enumeration_id(7));
        let b = self.get_char_at_location(&Location::from_enumeration_id(0));
        let c = self.get_char_at_location(&Location::from_enumeration_id(1));
        println!("{}------------{}------------{}", a, b, c);
        println!("|            |            |");

        let a = self.get_char_at_location(&Location::from_enumeration_id(15));
        let b = self.get_char_at_location(&Location::from_enumeration_id(8));
        let c = self.get_char_at_location(&Location::from_enumeration_id(9));
        println!("|   {}--------{}--------{}   |", a, b, c);
        println!("|   |        |        |   |");

        let a = self.get_char_at_location(&Location::from_enumeration_id(23));
        let b = self.get_char_at_location(&Location::from_enumeration_id(16));
        let c = self.get_char_at_location(&Location::from_enumeration_id(17));
        println!("|   |   {}----{}----{}   |   |", a, b, c);
        println!("|   |   |         |   |   |");

        let a = self.get_char_at_location(&Location::from_enumeration_id(6));
        let b = self.get_char_at_location(&Location::from_enumeration_id(14));
        let c = self.get_char_at_location(&Location::from_enumeration_id(22));
        let d = self.get_char_at_location(&Location::from_enumeration_id(18));
        let e = self.get_char_at_location(&Location::from_enumeration_id(10));
        let f = self.get_char_at_location(&Location::from_enumeration_id(2));
        println!("{}---{}---{}         {}---{}---{}", a, b, c, d, e, f);
        println!("|   |   |         |   |   |");

        let a = self.get_char_at_location(&Location::from_enumeration_id(21));
        let b = self.get_char_at_location(&Location::from_enumeration_id(20));
        let c = self.get_char_at_location(&Location::from_enumeration_id(19));
        println!("|   |   {}----{}----{}   |   |", a, b, c);

        println!("|   |        |        |   |");
        let a = self.get_char_at_location(&Location::from_enumeration_id(13));
        let b = self.get_char_at_location(&Location::from_enumeration_id(12));
        let c = self.get_char_at_location(&Location::from_enumeration_id(11));
        println!("|   {}--------{}--------{}   |", a, b, c);

        println!("|            |            |");
        let a = self.get_char_at_location(&Location::from_enumeration_id(5));
        let b = self.get_char_at_location(&Location::from_enumeration_id(4));
        let c = self.get_char_at_location(&Location::from_enumeration_id(3));
        println!("{}------------{}------------{}", a, b, c);
    }

    fn get_char_at_location(&self, location: &Location) -> char {
        match self.get_piece_at_location(location) {
            Some(piece) => piece.owner.get_char_symbol(),
            None => '-'
        }
    }

    fn get_evaluation_for(&self, team: &Team) -> f32;

    fn get_result_for(&self, team: &Team, history: &impl GameBoardHistoryCounter) -> i8;
}

impl QueryableGameBoard for GameBoard {
    fn encode(&self) -> String {
        let mut encoded = String::from("EEEEEEEEEEEEEEEEEEEEEEE");
        self.iter()
            .for_each(|piece| {
                let location = piece.location.to_enumeration_id();
                let char_index = encoded.char_indices().nth(location as usize).unwrap().0;
                encoded.replace_range(
                    char_index..(char_index + 1),
                    String::from(piece.owner.encode()).as_str()
                );
            });
        encoded
    }

    fn is_location_occupied(&self, location: &Location) -> bool {
        self.iter().any(|&piece| &piece.location == location)
    }

    fn get_piece_at_location(&self, location: &Location) -> Option<Piece> {
        self.iter().find(|&piece| &piece.location == location).copied()
    }

    fn add_piece_at_location(&mut self, location: &Location, team: &Team) {
        self.push(Piece {
            location: *location,
            owner: *team
        });
    }

    fn remove_piece_at_location(&mut self, location: Location) {
        self.retain(|&piece| piece.location != location);
    }

    fn move_piece(&mut self, src_location: Location, target_location: &Location, team: &Team) {
        // Remove current piece
        self.remove_piece_at_location(src_location);
        // Add new piece
        self.add_piece_at_location(target_location, team);
    }

    fn is_in_complete_mill(&self, location: &Location, current_team: &Team) -> bool {
        let mill_groups = MillGroup::all_groups_that_might_contain(location);
        // The piece to remove must not be in any complete mill group
        mill_groups.iter()
            .any(|group| group.is_complete(self, current_team))
    }

    fn count_pieces_of(&self, team: &Team) -> u8 {
        self.iter()
            .filter(|piece| piece.owner == *team)
            .count() as u8
    }

    fn get_evaluation_for(&self, team: &Team) -> f32 {
        // TODO: Make it more sophisticated
        let stone_count = self.count_pieces_of(team);
        let opponent_count = self.count_pieces_of(&team.get_opponent());
        let can_fly = stone_count <= 3;

        // Look at the number of stones
        let stone_distribution = stone_count / opponent_count;
        let stone_distr_fraction = stone_distribution as f32 / 9.0;

        // Determine whether we are close to being locked in place
        // Upper limit is the most moves we could possibly make
        let upper_limit_of_moves = match can_fly {
            true => stone_count * (TOTAL_NUMBER_FIELDS - stone_count - opponent_count),
            false => stone_count * 4, // four for the number of directions in which we could possibly move. Overestimates a lot.
        } as f32;
        let actual_number_of_moves = MoveActionIterator::new(self.clone(), *team)
            .count() as f32;
        let moves_fraction = actual_number_of_moves / upper_limit_of_moves;


        let flight_bonus = if can_fly { 1.0 } else { 0.0 };

        0.75 * stone_distr_fraction + 0.1 * moves_fraction.powi(2) + 0.05 * flight_bonus
    }

    fn get_result_for(&self, team: &Team, history: &impl GameBoardHistoryCounter) -> i8 {
        return if team.is_defeated(&self) {
            -1
        } else if history.is_third_time(&self) {
            0
        } else {
            1
        };
    }
}