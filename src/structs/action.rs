use scan_rules::input::ScanInput;
use crate::{Location, SlideOffset};
use crate::structs::Team;
use crate::types::{GameBoard, GameContext};
use crate::types::game_board::QueryableGameBoard;

// Something that should be done during a turn
#[derive(Debug, Copy, Clone)]
pub enum Action {
    Place {
        new_location: Location,
    },
    Slide {
        src_location: Location,
        slide: SlideOffset,
    },
    Fly {
        src_location: Location,
        target_location: Location,
    }
}

impl Action {

    pub(crate) fn apply(&self, team: &Team, board: &mut GameBoard) {
        match self {
            Action::Place { new_location, .. } => {
                board.add_piece_at_location(new_location, team)
            },
            Action::Slide { src_location, slide, .. } => {
                board.move_piece(*src_location, &src_location.get_location_for(slide), team)
            }
            Action::Fly { src_location, target_location, .. } => {
                board.move_piece(*src_location, target_location, team);
            }
        }
    }

    pub fn get_new_location(&self) -> Location {
        match self {
            Action::Place { new_location, .. } => *new_location,
            Action::Slide { src_location, slide, .. } => src_location.get_location_for(slide),
            Action::Fly { target_location, .. } => *target_location
        }
    }

    pub fn will_make_mill(
        &self,
        context: &GameContext
    ) -> bool {
        // Copy the board
        let mut future_board = context.board.clone();
        self.apply(&context.team, &mut future_board);

        future_board.is_in_complete_mill(&self.get_new_location(), &context.team)
    }

    pub fn encode(&self) -> String {
        let (action_type, first_location, second_location) = match self {
            Action::Place { new_location, .. } => ('P', new_location, None),
            Action::Slide {
                src_location,
                slide
            } => ('M', src_location, Some(src_location.get_location_for(slide))),
            Action::Fly {
                src_location,
                target_location
            } => ('M', src_location, Some(*target_location)),
        };

        let result = format!(
            "{} {} {}",
            action_type,
            first_location.encode(),
            match &second_location {
                None => String::new(),
                Some(second_location) => {
                    second_location.encode()
                }
            }
        );
        String::from(result.trim())
    }
}