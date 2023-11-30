use crate::constants::TOTAL_NUMBER_FIELDS;
use crate::structs::{Action, GamePhase, Location, SlideOffsetIterator, Team, Turn};
use crate::types::game_board::QueryableGameBoard;
use crate::types::{GameBoard, GameContext};

pub struct MoveActionIterator {
    board: GameBoard,
    team: Team,
    current_src_location_id: u8,
    slide_offset_iterator: SlideOffsetIterator, // For sliding actions
    current_target_location_id: u8              // For flying  actions
}

impl MoveActionIterator {
    pub(crate) fn new(board: GameBoard, team: Team) -> MoveActionIterator {
        MoveActionIterator {
            board,
            team,
            current_src_location_id: 0,
            slide_offset_iterator: SlideOffsetIterator::new(),
            current_target_location_id: 0
        }
    }
}

impl Iterator for MoveActionIterator {
    type Item = Action;

    fn next(&mut self) -> Option<Self::Item> {
        let allowed_to_fly = self.team.is_allowed_to_fly(&self.board);

        loop {
            let src_location = Location::from_enumeration_id(self.current_src_location_id);

            if self.board.is_location_occupied(&src_location) &&
                self.board.get_piece_at_location(&src_location).unwrap().owner == self.team {

                if allowed_to_fly {
                    // Find next target location
                    while self.current_target_location_id < TOTAL_NUMBER_FIELDS {
                        let target_location = Location::from_enumeration_id(self.current_target_location_id);

                        self.current_target_location_id += 1;

                        if !self.board.is_location_occupied(&target_location) {
                            return Some(Action::Fly {
                                target_location,
                                src_location: Location::from_enumeration_id(self.current_src_location_id),
                            })
                        }
                    }
                } else {
                    'find_next_slide: loop {
                        let next_slide = self.slide_offset_iterator.next();

                        if let Some(slide) = next_slide {
                            let action = Action::Slide {
                                src_location: Location::from_enumeration_id(self.current_src_location_id),
                                slide,
                            };
                            let turn_without_taking = Turn {
                                action,
                                piece_to_take: None,
                            };
                            if let Ok(_) = turn_without_taking.validate(&GameContext {
                                board: self.board.clone(),
                                team: self.team,
                                phase: GamePhase::Moving
                            }) {
                                return Some(action);
                            }
                        } else { break 'find_next_slide; }
                    }
                };
            } else {
                // If we're at the end of the field
                if self.current_src_location_id >= TOTAL_NUMBER_FIELDS - 1 {
                    return None;
                }
            };

            // Advance to next src_location, since nobody returned yet, so all options are exhausted

            // Look at the next location
            self.current_src_location_id += 1;
            // Reset the slide offset iterator, as we are now on the lookout for a new piece to move
            self.slide_offset_iterator = SlideOffsetIterator::new();
            // Reset the target location, as we are now on the lookout for a new piece to move
            self.current_target_location_id = 0;
        }
    }
}

pub struct PlaceActionIterator {
    board: GameBoard,
    team: Team,
    current_place_location_id: u8,
}

impl PlaceActionIterator {
    pub(crate) fn new(board: GameBoard, team: Team) -> Self {
        Self {
            board,
            team,
            current_place_location_id: 0,
        }
    }
}

impl Iterator for PlaceActionIterator {
    type Item = Action;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            // If we're at the end of the field
            if self.current_place_location_id >= TOTAL_NUMBER_FIELDS - 1 {
                return None;
            }

            let location = Location::from_enumeration_id(self.current_place_location_id);

            if !self.board.is_location_occupied(&location) {
                self.current_place_location_id += 1;
                return Some(Action::Place {
                    new_location: location
                });
            }

            self.current_place_location_id += 1;

        }
    }
}