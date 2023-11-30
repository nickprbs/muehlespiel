use std::fmt::Debug;
use crate::structs::{Turn, SlideOffset, Action};
use crate::structs::game_phase::*;
use crate::types::game_board::QueryableGameBoard;
use crate::types::GameContext;

pub trait Rule: Debug {
    fn applies_to_turn(&self, turn: &Turn) -> bool;
    fn is_obeyed_by(&self, turn: &Turn, context: &GameContext) -> bool;
}

// We don't want to allow placing while were in the moving phase (and the other way round)
#[derive(Debug)]
pub struct CorrectPhase;
impl Rule for CorrectPhase {
    fn applies_to_turn(&self, _turn: &Turn) -> bool { true } // All actions
    fn is_obeyed_by(&self, turn: &Turn, context: &GameContext) -> bool {
        let expected_phase: GamePhase = match turn.action {
            Action::Place { .. }                    => GamePhase::Placing,
            Action::Slide { .. } | Action::Fly { .. } => GamePhase::Moving,
        };
        context.phase == expected_phase
    }
}

// Check that we're not moving to an occupied field and that the new location is within the bounds
#[derive(Debug)]
pub struct NewLocationUnoccupiedAndValid;
impl Rule for NewLocationUnoccupiedAndValid {
    fn applies_to_turn(&self, _turn: &Turn) -> bool { true } // All actions
    fn is_obeyed_by(&self, turn: &Turn, context: &GameContext) -> bool {
        let new_location = match turn.action {
            Action::Place { new_location, .. } => new_location,
            Action::Slide { src_location, slide, .. } => {
                src_location.get_location_for(&slide)
            },
            Action::Fly { target_location, .. } => target_location
        };

        return !context.board.is_location_occupied(&new_location) && new_location.is_valid();
    }
}

// Check that player only moves their own pieces
#[derive(Debug)]
pub struct PlayerOwnsPiece;
impl Rule for PlayerOwnsPiece {
    fn applies_to_turn(&self, turn: &Turn) -> bool {
        match turn.action {
            Action::Slide { .. } | Action::Fly { .. } => true,
            Action::Place { .. }                      => false,
        }
    }
    fn is_obeyed_by(&self, turn: &Turn, context: &GameContext) -> bool {
        let location = match turn.action {
            Action::Slide { src_location, .. } | Action::Fly { src_location, .. } => {
                src_location
            }
            _ => panic!()
        };
        let actual_team = context.board.get_piece_at_location(&location)
            .expect("Piece not found")
            .owner;
        let expected_team = context.team;

        expected_team == actual_team
    }
}

// Check if the player is actually allowed to fly
#[derive(Debug)]
pub struct PlayerIsAllowedToFly;
impl Rule for PlayerIsAllowedToFly {
    fn applies_to_turn(&self, turn: &Turn) -> bool {
        match turn.action {
            Action::Fly { .. } => true,
            _                  => false
        }
    }
    fn is_obeyed_by(&self, _turn: &Turn, context: &GameContext) -> bool {
        context.team.is_allowed_to_fly(&context.board)
    }
}

// Check that we don't slide in- or outward, when there is no connection between fields
#[derive(Debug)]
pub struct NotSlidingDiagonally;
impl Rule for NotSlidingDiagonally {
    fn applies_to_turn(&self, turn: &Turn) -> bool {
        match turn.action {
            Action::Slide { .. } => true,
            _                    => false
        }
    }

    fn is_obeyed_by(&self, turn: &Turn, _context: &GameContext) -> bool {
        if let Action::Slide { slide, src_location, .. } = turn.action {
            match slide {
                // We can always do (counter)clockwise slide
                SlideOffset::Clockwise | SlideOffset::CounterClockwise => true,
                // We can only do in/out slide if alignment index is even
                SlideOffset::Inward | SlideOffset::Outward => {
                    src_location.alignment % 2 == 0
                }
            }
        } else { panic!() }
    }
}

// Check that the piece we want to take actually exists
#[derive(Debug)]
pub struct TakingExistingPiece;
impl Rule for TakingExistingPiece {
    fn applies_to_turn(&self, turn: &Turn) -> bool {
        turn.wants_to_take_piece()
    }

    fn is_obeyed_by(&self, turn: &Turn, context: &GameContext) -> bool {
        let piece_to_take = turn.piece_to_take
            .expect("Action does not want to take piece");
        context.board.is_location_occupied(&piece_to_take)
    }
}

// Check that we don't want to take our own piece
#[derive(Debug)]
pub struct NotTakingOwnPiece;
impl Rule for NotTakingOwnPiece {
    fn applies_to_turn(&self, turn: &Turn) -> bool {
        turn.wants_to_take_piece()
    }

    fn is_obeyed_by(&self, turn: &Turn, context: &GameContext) -> bool {
        let piece_to_take_pos = &turn.piece_to_take
            .expect("Action does not want to take piece");
        let piece_to_take = context.board.get_piece_at_location(piece_to_take_pos)
            .expect("Piece not found");
        piece_to_take.owner != context.team
    }
}

// Check that we're not taking a piece that's in a mill
#[derive(Debug)]
pub struct NotTakingPieceFromMill;
impl Rule for NotTakingPieceFromMill {
    fn applies_to_turn(&self, turn: &Turn) -> bool {
        turn.wants_to_take_piece()
    }

    fn is_obeyed_by(&self, turn: &Turn, context: &GameContext) -> bool {
        let piece_to_take = &turn.piece_to_take
            .expect("Action does not want to take piece");
        !context.board.is_in_complete_mill(piece_to_take, &context.team)
    }
}

// Check if removing piece due to mill is actually allowed
#[derive(Debug)]
pub struct MadeAMill;
impl Rule for MadeAMill {
    fn applies_to_turn(&self, turn: &Turn) -> bool {
        turn.wants_to_take_piece()
    }

    fn is_obeyed_by(&self, turn: &Turn, context: &GameContext) -> bool {
        turn.action.will_make_mill(&context)
    }
}