use std::fs;
use itertools::Itertools;
use crate::datastructures::{Direction, DirectionIter, Encodable, GameBoard, GameBoardLocation, Location, Phase, Team, Turn, TurnAction};
use crate::datastructures::game_board::UsefulGameBoard;
use crate::iterators::location_iterator::LocationIterator;

pub struct ChildTurnIterator {
    phase: Phase,
    board: GameBoard,
    team: Team,
    occupied_locations: Vec<Location>,
    own_locations: Vec<Location>,
    opponent_locations: Vec<Location>,
    takeable_opponent_locations: Vec<Location>,

    placing_current_location: Option<Location>,

    moving_current_to_be_moved_index: usize,
    moving_can_jump: bool,
    moving_jump_to_iterator: LocationIterator,
    moving_current_move_to_location: Option<Location>,
    moving_direction_iterator: DirectionIter,
    moving_current_to_be_taken_index: Option<usize>,
}

impl ChildTurnIterator {
    pub(crate) fn new(phase: Phase, team: Team, board: GameBoard) -> Self {
        let own_locations = board.get_piece_locations(team);
        let opponent_locations = board.get_piece_locations(team.get_opponent());

        let mut occupied_locations = own_locations.clone();
        occupied_locations.append(&mut opponent_locations.clone());
        let free_locations = (1_u8..=24_u8)
            .filter(|loc| { !occupied_locations.contains(loc) });

        let takeable_opponent_locations = Self::calculate_takeables(opponent_locations.clone());

        let first_free_location: Option<Location> = free_locations.into_iter()
            .sorted()
            .nth(0);

        let can_jump = own_locations.len() <= 3;
        let mut forbidden_to_jump_to = occupied_locations.clone();
        forbidden_to_jump_to.append(&mut vec![own_locations[0]]);
        let jump_to_iterator = LocationIterator::with_forbidden(forbidden_to_jump_to);

        Self {
            phase,
            board,
            team,
            occupied_locations,
            own_locations,
            opponent_locations,
            takeable_opponent_locations,

            placing_current_location: first_free_location,

            moving_current_to_be_moved_index: 0,
            moving_can_jump: can_jump,
            moving_jump_to_iterator: jump_to_iterator,
            moving_current_move_to_location: None,
            moving_direction_iterator: Direction::iter(),
            moving_current_to_be_taken_index: None
        }
    }
}

impl Iterator for ChildTurnIterator {
    type Item = Turn;

    fn next(&mut self) -> Option<Self::Item> {
        match self.phase {
            Phase::PLACE => {
                self.next_placing_turn()
            }
            Phase::MOVE => {
                self.next_moving_turn()
            }
        }
    }
}

impl ChildTurnIterator {

    fn calculate_takeables(opponent_locations: Vec<Location>) -> Vec<Location> {
        // All opponent locations that are not in a mill
        let mut takeable_opponent_locations: Vec<Location> = opponent_locations.clone().into_iter()
            .filter(|location| !GameBoard::is_mill_at2(*location, &opponent_locations, &vec![]))
            .collect();

        // It might be that there are no to take, because all are in mills. Then, all can be taken
        if takeable_opponent_locations.len() == 0 {
            takeable_opponent_locations = opponent_locations;
        };

        takeable_opponent_locations
    }

    fn next_placing_turn(&mut self) -> Option<<ChildTurnIterator as Iterator>::Item> {
        match self.placing_current_location {
            None => None,
            Some(current_location) => {
                let turn = Turn {
                    action: TurnAction::Place { location: current_location },
                    take_from: None,
                };

                self.placing_current_location = if current_location < 24 {
                    Some(current_location + 1)
                } else { None };

                Some(turn)
            }
        }
    }

    fn next_moving_turn(&mut self) -> Option<<ChildTurnIterator as Iterator>::Item> {
        // We're currently enumerating different positions to take
        if let Some(to_be_taken_index) = self.moving_current_to_be_taken_index {
            self.next_taking_move(to_be_taken_index)
        // Go in next direction/jump to next location or go to next origin location
        } else {
            if self.moving_can_jump {
                self.next_jumping_turn()
            } else {
                self.next_sliding_turn()
            }
        }
    }

    fn next_taking_move(&mut self, to_be_taken_index: usize) -> Option<<ChildTurnIterator as Iterator>::Item> {
        let to_be_moved_location = self.own_locations[self.moving_current_to_be_moved_index];

        let result = Turn {
            action: TurnAction::Move {
                from: to_be_moved_location,
                to: self.moving_current_move_to_location.unwrap(),
            },
            take_from: Some(self.takeable_opponent_locations[to_be_taken_index]),
        };

        self.moving_current_to_be_taken_index =
            if (to_be_taken_index + 1) < self.takeable_opponent_locations.len() {
                Some(to_be_taken_index + 1)
            } else {
                None
            };

        Some(result)
    }

    fn next_turn_with_or_without_taking(&mut self, next_location: Location, origin_location: Location) -> Option<<ChildTurnIterator as Iterator>::Item> {
        self.moving_current_move_to_location = Some(next_location);

        let turn_without_taking = Turn {
            action: TurnAction::Move {
                from: origin_location,
                to: next_location
            },
            take_from: None
        };

        if self.takeable_opponent_locations.len() <= 0 {
            // We cannot take a piece, but we should still return this move
            Some(turn_without_taking)
        } else {
            // We could take a piece, but have we made a mill?
            let new_board = self.board.apply(turn_without_taking.clone(), self.team);

            if new_board.is_mill_at3(next_location) {
                self.set_up_taking();
                self.moving_current_to_be_taken_index = Some(0);
                // We can take a piece, so in next recursive call, the first block (enumerating take) will execute
                self.next_moving_turn()
            } else {
                Some(turn_without_taking)
            }
        }
    }

    fn next_sliding_turn(&mut self) -> Option<<ChildTurnIterator as Iterator>::Item> {
        let next_direction = self.moving_direction_iterator.next();

        if let Some(next_direction) = next_direction {
            let origin_location = self.own_locations[self.moving_current_to_be_moved_index];
            // There is a next direction, so apply it to our origin location
            let next_location = origin_location.apply_direction(next_direction);

            // Is this a valid location?
            if let Some(next_location) = next_location {
                // Is this location occupied?
                if self.occupied_locations.contains(&next_location) {
                    return self.next_moving_turn();
                }
                self.next_turn_with_or_without_taking(next_location, origin_location)
            } else {
                self.next_moving_turn()
            }
        } else {
            self.next_origin_location_turn()
        }
    }

    fn next_jumping_turn(&mut self) -> Option<<ChildTurnIterator as Iterator>::Item> {
        if let Some(next_location) = self.moving_jump_to_iterator.next() {
            let origin_location = self.own_locations[self.moving_current_to_be_moved_index];
            self.next_turn_with_or_without_taking(next_location, origin_location)
        } else {
            self.next_origin_location_turn()
        }
    }

    fn next_origin_location_turn(&mut self) -> Option<<ChildTurnIterator as Iterator>::Item> {
        // Go to next origin location, since we did all directions
        return if self.moving_current_to_be_moved_index < self.own_locations.len() - 1 {
            self.moving_current_to_be_moved_index += 1;
            let mut origin_location = self.own_locations[self.moving_current_to_be_moved_index];
            if self.moving_can_jump {
                let mut forbidden_fields = self.occupied_locations.clone();
                forbidden_fields.append(&mut vec![origin_location]); // Don't move from field to same field
                self.moving_jump_to_iterator = LocationIterator::with_forbidden(forbidden_fields);
            } else {
                self.moving_direction_iterator = Direction::iter();
            }

            self.next_moving_turn()
        } else {
            None
        }
    }

    fn set_up_taking(&mut self) {
        if GameBoard::is_mill_at2(
            self.moving_current_move_to_location.unwrap(),
            &self.own_locations,
            &self.opponent_locations
        ) {
            self.moving_current_to_be_taken_index = Some(0)
        } else {
            self.moving_current_to_be_taken_index = None
        }
    }
}

#[test]
fn test_child_turn_iterator_moving_simple() {
    let case = GameBoard::decode(String::from(
        concat!(
        "WWWWEEEE",
        "BEBEEEEE",
        "EEEEEEEE",
        )
    ));
    assert!(list_equality(
        ChildTurnIterator::new(Phase::MOVE, Team::WHITE, case).collect::<Vec<Turn>>(),
        vec![
            Turn { action: TurnAction::Move { from: 1, to: 8 }, take_from: None },
            Turn { action: TurnAction::Move { from: 4, to: 5 }, take_from: None },
        ]
    ));

    let case = GameBoard::decode(String::from(
        concat!(
            "BBBBBBBB",
            "EWWWWWWW",
            "WBBBBBWB",
        )
    ));
    assert!(list_equality(
        ChildTurnIterator::new(Phase::MOVE, Team::WHITE, case).collect::<Vec<Turn>>(),
        vec![
            Turn { action: TurnAction::Move { from: 17, to: 9 }, take_from: Some(24) },
            Turn { action: TurnAction::Move { from: 16, to: 9 }, take_from: None },
            Turn { action: TurnAction::Move { from: 10, to: 9 }, take_from: None },
        ]
    ));

    let case = GameBoard::from([
        0b0101010101010101, // All black
        0b1001100010101010, // All white but one (only moves: 11 -> 12, 13 -> 12), and one black to take
        0b0101010101010101, // All black
    ]);
    assert!(list_equality(
        ChildTurnIterator::new(Phase::MOVE, Team::WHITE, case).collect::<Vec<Turn>>(),
        vec![
            Turn { action: TurnAction::Move { from: 11, to: 12 }, take_from: Some(10) },
            Turn { action: TurnAction::Move { from: 13, to: 12 }, take_from: None },
        ]
    ));
}

#[test]
fn test_child_turn_iterator_testset_samples() {
    let case = GameBoard::decode(String::from("BEEEEWWBBEWEWWEEBEBWEBEW"));
    let turns = ChildTurnIterator::new(Phase::MOVE, Team::WHITE, case).dedup().collect::<Vec<Turn>>();
    dbg!(turns.clone());
    assert_eq!(11, turns.len());
}

#[test]
fn test_child_turn_iterator_moving() {
    let file_contents = fs::read_to_string("./tests/child-turn-iterator/input_felder.txt")
        .expect("File could not be read");

    let mut boards = file_contents.split_terminator('\n');
    let mut actual: String = String::new();

    while let Some(board) = boards.next() {
        print!("{board}: ");
        let board = GameBoard::decode(String::from(board));

        let turns: Vec<Turn> = ChildTurnIterator::new(Phase::MOVE, Team::WHITE, board).dedup().collect();
        let count_turns_not_counting_taking = turns.clone().into_iter()
            .map(|turn| { turn.action })
            .dedup()
            .count();
        let count_turns_with_mills = turns.clone().into_iter()
            .filter(|turn| { turn.take_from.is_some() })
            .map(|turn| { turn.action })
            .dedup()
            .count();

        let opponent_locations = board.get_piece_locations(Team::BLACK);
        let count_takeables = ChildTurnIterator::calculate_takeables(opponent_locations).len();
        let actual_count_takeables = if turns.iter().any(|turn| { turn.take_from.is_some() }) {
            count_takeables
        } else {
            0
        };

        let result = format!("{count_turns_not_counting_taking} {count_turns_with_mills} {actual_count_takeables}\n");
        println!("{result}");
        actual.push_str(result.as_str());
    }

    let expected = fs::read_to_string("./tests/child-turn-iterator/output.txt")
        .expect("File could not be read");

    assert_eq!(expected.trim(), actual.trim());
}

fn list_equality<Item: PartialEq + std::fmt::Debug>(a_list: Vec<Item>, b_list: Vec<Item>) -> bool {
    for a in &a_list {
        if !&b_list.contains(a) {
            eprintln!("Item {a:?} is not in {b_list:?}");
            return false;
        }
    }
    for b in &b_list {
        if !&a_list.contains(b) {
            eprintln!("Item {b:?} is not in {a_list:?}");
            return false;
        }
    }

    return true;
}