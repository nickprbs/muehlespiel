use std::fs;
use itertools::Itertools;
use crate::datastructures::game_board::{CanonicalGameBoard, UsefulGameBoard};
use crate::datastructures::{Encodable, GameBoard, Location, Phase, Team};
use crate::iterators::location_iterator::LocationIterator;
use crate::iterators::{ChildTurnIterator, NeighboursIterator};

pub struct ParentBoardIterator {
    before_team: Team,
    after_team: Team,
    can_fly: bool,

    post_board: GameBoard,
    all_locations_occupied_post: Vec<Location>,

    locations_of_before_team_post: Vec<Location>,
    post_position_iter: LocationIterator,
    current_post_position: Option<Location>,
    post_position_in_mill: bool,

    locations_of_after_team_post: Vec<Location>,
    pre_position_iter: Box<dyn Iterator<Item=Location>>,
    current_pre_position: Option<Location>,

    possible_locations_taken_from: Vec<Location>,
    taken_from_iter: LocationIterator,
}

impl ParentBoardIterator {
    pub(crate) fn new(after_team: Team, board: GameBoard) -> Self {
        let before_team = after_team.get_opponent();
        let can_fly = board.get_num_pieces(before_team) <= 3;

        let locations_of_before_team = board.get_piece_locations(before_team);
        let locations_of_after_team = board.get_piece_locations(after_team);

        let mut all_locations_occupied_post = locations_of_after_team.clone();
        all_locations_occupied_post.append(&mut locations_of_before_team.clone());

        let possible_locations_taken_from: Vec<Location> = (1..=24)
            // The taken piece cannot be at a location that's occupied
            .filter(|loc| { !all_locations_occupied_post.contains(loc) })
            .filter(|loc| {
                let board_with_that_piece = board.place_bits_at(after_team.as_binary(), *loc);

                // Determine whether with this piece present, all of after-team's pieces are in a mill
                // If so, we can take from this location, since we can take from all locations
                let all_in_mill = locations_of_after_team.iter()
                    .all(|loc| board_with_that_piece.is_mill_for_team_at(after_team, *loc));
                if all_in_mill { return true; }

                // Not all pieces are in a mill, so now we need to check whether this single piece
                // is in a mill. If so, return false, since we can't take it.
                let this_in_mill = board_with_that_piece.is_mill_for_team_at(after_team, *loc);
                return !this_in_mill;
            })
            .collect();
        // Note: There is one more location from where we could not have taken: The before-team's
        // moved piece pre position. We need to check that in the Iterator itself however,
        // since the moved piece changes throughout the run.

        let pre_position_iter = Box::new(LocationIterator::with_allowed(vec![]));

        Self {
            before_team,
            after_team,
            can_fly,

            post_board: board,
            all_locations_occupied_post,

            locations_of_before_team_post: locations_of_before_team.clone(),
            post_position_iter: LocationIterator::with_allowed(locations_of_before_team),
            current_post_position: None,
            post_position_in_mill: false, // Value shouldn't matter, since we're setting it anyway

            locations_of_after_team_post: locations_of_after_team,
            pre_position_iter,
            current_pre_position: None,

            possible_locations_taken_from,
            taken_from_iter: LocationIterator::with_allowed(vec![]),
        }
    }

    fn set_up_taken_from_iter(&mut self, pre_location: Location) {
        // Important: We need to remove the pre location from the possible taken iter
        self.taken_from_iter = if self.post_position_in_mill {
            let mut taken_from_locations = self.possible_locations_taken_from.clone();
            let index_of_pre_position = taken_from_locations.iter()
                .position(|x| *x == pre_location);
            if let Some(index) = index_of_pre_position {
                taken_from_locations.remove(index);
            } // else: it might be that the pre location is already ignored (for example for being in a mill)

            LocationIterator::with_allowed(taken_from_locations)
        } else {
            // We cannot take anything
            LocationIterator::with_allowed(vec![])
        };
    }

    fn build_canonical_board_option(&self,
                                    move_from: Location,
                                    move_to: Location,
                                    un_take_from: Option<Location>,
    ) -> Option<CanonicalGameBoard> {
        let mut pre_board = self.post_board
            .place_bits_at(self.before_team.as_binary(), move_from)
            .place_bits_at(0b00, move_to);

        if let Some(un_take_from) = un_take_from {
            pre_board = pre_board.place_bits_at(self.after_team.as_binary(), un_take_from);
        }

        return Some(pre_board.get_representative());
    }
}

impl Iterator for ParentBoardIterator {
    type Item = CanonicalGameBoard;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(taken_from) = self.taken_from_iter.next() {
                // PHASE c (>>>): The before-team moved into a mill, so now we're iterating all pieces that can
                // be taken
                return self.build_canonical_board_option(
                    self.current_pre_position.unwrap(),
                    self.current_post_position.unwrap(),
                    Some(taken_from),
                );
            } else {
                // PHASES: The before-team did not move into a mill. We now need to check whether we
                // 1) b (>>): can un-move the piece at the post position from another pre position
                // 2) a (>):  or we have visited all pre positions with that post position and need the
                //            use the next post position
                if let Some(past_pre_location) = self.current_pre_position {
                    let next_pre_location = self.pre_position_iter.next();
                    self.current_pre_position = next_pre_location;

                    if let Some(next_pre_location) = next_pre_location {
                        // Reset the taken_from_iter to start over
                        self.set_up_taken_from_iter(next_pre_location);
                    }

                    // It may be that we can't take one, so the PHASE c above won't execute
                    // Therefore, return here if the pre_location isn't None
                    if !self.post_position_in_mill {
                        return self.build_canonical_board_option(
                            past_pre_location,
                            self.current_post_position.unwrap(),
                            None,
                        );
                    }

                    // We have taken some pieces, so we do another loop iteration so that we end up in PHASE c
                } else {
                    // Go to next post position
                    if let Some(next_post_location) = self.post_position_iter.next() {
                        self.current_post_position = Some(next_post_location);
                        self.post_position_in_mill = self.post_board.is_mill_for_team_at(self.before_team, next_post_location);

                        // Reset the pre_position_iter
                        self.pre_position_iter = if self.can_fly {
                            Box::new(LocationIterator::with_forbidden(
                                self.all_locations_occupied_post.clone()
                            ))
                        } else {
                            Box::new(NeighboursIterator::new_with_forbidden(
                                vec![next_post_location],
                                self.all_locations_occupied_post.clone(),
                            ))
                        };

                        self.current_pre_position = self.pre_position_iter.next();
                        if let Some(next_pre_location) = self.current_pre_position {
                            // Reset the taken_from_iter to start over
                            self.set_up_taken_from_iter(next_pre_location);
                        }

                        // Let's jump directly into PHASE a by doing another loop
                    } else {
                        // There's nothing we need to look at anymore, since we looked at all post positions
                        return None;
                    }
                }
            }
        }
    }
}

#[test]
fn test_parent_board_iter2() {
    let mut file_content = fs::read_to_string("./tests/complete-search/3vs3/input_felder.txt")
        .expect("File could not be read");
    file_content.push_str(
        fs::read_to_string("./tests/complete-search/5vs5/input_felder.txt").unwrap().as_str()
    );

    let parent_cases = file_content
        .split_terminator("\n")
        .map(|case| {
            GameBoard::decode(String::from(case))
        });

    parent_cases.into_iter().for_each(|parent| {
        let children = ChildTurnIterator::new(Phase::MOVE, Team::WHITE, parent);

        children.for_each(|turn| {
            let child_board = parent.apply(turn, Team::WHITE);
            let mut parents_of_child: Vec<CanonicalGameBoard> = ParentBoardIterator::new(Team::BLACK, child_board).collect();

            assert!(
                parents_of_child.contains(&parent.get_representative()),
                "Child {} does not include parent {}. Either ChildTurnIterator or ParentBoardIterator is wrong. Number of supposed parents: {:?}",
                child_board.encode(), parent.encode(), parents_of_child.len()
            );
        })
    })
}

#[test]
fn test_parent_board_equivalence_1_and_2() {
    let mut file_content = fs::read_to_string("./tests/complete-search/3vs3/input_felder.txt")
        .expect("File could not be read");

    let parent_cases = file_content
        .split_terminator("\n")
        .map(|case| {
            GameBoard::decode(String::from(case))
        });

    parent_cases.into_iter().for_each(|parent| {
        let children = ChildTurnIterator::new(Phase::MOVE, Team::WHITE, parent);

        children.for_each(|turn| {
            let child_board = parent.apply(turn, Team::WHITE);
            let parents_of_child_1: Vec<CanonicalGameBoard> = crate::iterators::parent_board_iterator::ParentBoardIterator::new(Team::BLACK, child_board).collect();
            let parents_of_child_2: Vec<CanonicalGameBoard> = crate::iterators::parent_board_iterator2::ParentBoardIterator::new(Team::BLACK, child_board).collect();

            parents_of_child_1.iter().for_each(|parent_1| {
                assert!(parents_of_child_2.contains(parent_1), "Parent from Iterator 1 '{}' is not in Parents from Iterator 2. Post-Board: {}", parent_1.encode(), child_board.encode())
            });
            parents_of_child_2.iter().for_each(|parent_2| {
                assert!(parents_of_child_1.contains(parent_2), "Parent from Iterator 2 '{}' is not in Parents from Iterator 1. Post-Board: {}", parent_2.encode(), child_board.encode())
            });
        })
    })
}

#[test]
fn test_parent_board_iter2_minimality() {
    // 4 White, 3 Black
    let child = [
        0b_10_10_00_10_10_00_00_00,
        0b_00_01_00_01_00_01_00_00,
        0b_00_00_00_00_00_00_00_00,
    ];
    let non_parents = [
        [
            0b_00_10_10_10_10_00_00_00,
            0b_00_01_00_01_00_01_00_00,
            0b_00_00_00_00_00_00_00_00,
        ],
        [
            0b_10_10_10_10_00_00_00_00,
            0b_00_01_00_01_00_01_00_00,
            0b_00_00_00_00_00_00_00_00,
        ],
        [
            0b_00_10_00_10_10_00_10_00,
            0b_00_01_00_01_00_01_00_00,
            0b_00_00_00_00_00_00_00_00,
        ],
        [
            0b_00_00_10_10_10_00_00_10,
            0b_00_01_00_01_00_01_00_00,
            0b_00_00_00_00_00_00_00_00,
        ],
        [
            0b_00_10_00_10_10_00_00_00,
            0b_00_01_00_01_00_01_00_00,
            0b_10_00_00_00_00_00_00_00,
        ],
        [
            0b_10_10_10_00_00_00_00_00,
            0b_00_01_00_01_00_01_00_00,
            0b_00_00_00_00_10_00_00_00,
        ],
    ]
        .iter()
        .map(|non_parent| non_parent.get_representative());

    let mut actual_parents: Vec<CanonicalGameBoard> = ParentBoardIterator::new(Team::WHITE.get_opponent(), child).collect();
    dbg!(&child.get_representative().encode());
    dbg!(&actual_parents.iter().map(|parent| parent.encode()).collect_vec());
    dbg!(&non_parents.clone().map(|parent| parent.encode()).collect_vec());

    for non_parent in non_parents {
        assert!(
            !actual_parents.contains(&non_parent),
            "Parents wrongly contained {}",
            non_parent.encode()
        );
    }

    assert_eq!(actual_parents.len(), 6);
}