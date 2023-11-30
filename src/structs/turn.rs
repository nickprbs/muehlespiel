use crate::structs::{Action, GamePhase, Location, Team};
use crate::structs::rule::*;
use crate::types::{GameBoard, GameContext};
use crate::types::game_board::QueryableGameBoard;

#[derive(Debug, Clone, Copy)]
pub struct Turn {
    pub(crate) action: Action,
    pub(crate) piece_to_take: Option<Location>
}

impl Turn {
    pub(crate) fn wants_to_take_piece(&self) -> bool {
        self.piece_to_take.is_some()
    }

    pub(crate) fn encode(&self) -> String {
        let encoded_action = self.action.encode();
        match self.piece_to_take {
            None => encoded_action,
            Some(piece_to_take) => {
                format!("{} T {}", encoded_action, piece_to_take.encode())
            }
        }
    }

    pub(crate) fn validate(&self, context: &GameContext) -> Result<(), Box<dyn Rule>> {
        let rules_to_check: Vec<Box<dyn Rule>> = vec![
            Box::new(CorrectPhase),
            Box::new(NewLocationUnoccupiedAndValid),
            Box::new(PlayerOwnsPiece),
            Box::new(PlayerIsAllowedToFly),
            Box::new(NotSlidingDiagonally),
            Box::new(TakingExistingPiece),
            Box::new(NotTakingOwnPiece),
            Box::new(NotTakingPieceFromMill),
            Box::new(MadeAMill),
        ];
        for rule in rules_to_check {
            if rule.applies_to_turn(self) {
                if !rule.is_obeyed_by(self, context) {
                    return Err(rule)
                }
            }
        }
        Ok(())
    }

    pub(crate) fn apply_safely<'lifetime>(&'lifetime self,
                                          team: &Team,
                                          phase: &GamePhase,
                                          board: &mut GameBoard
    ) -> Result<(), Box<dyn Rule>> {

        match self.validate(&GameContext {
            team: *team, phase: *phase, board: board.clone()
        }) {
            Err(broken_rule) => return Err(broken_rule),
            Ok(_) => {}
        }

        // Not unsafe, since we validated before
        self.apply_unsafely(team, board);

        return Ok(())
    }

    pub(crate) fn apply_unsafely<'lifetime>(&'lifetime self, team: &Team, board: &mut GameBoard) {
        self.action.apply(team, board);

        // Remove piece to take
        if let Some(piece_to_take) = self.piece_to_take {
            board.remove_piece_at_location(piece_to_take);
        }
    }
}