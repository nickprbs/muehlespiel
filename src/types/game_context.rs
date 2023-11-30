use crate::structs::{GamePhase, Rule, Team, Turn};
use crate::types::game_board::QueryableGameBoard;
use crate::types::GameBoard;

#[derive(Clone)]
pub struct GameContext {
    pub(crate) team: Team,
    pub(crate) board: GameBoard,
    pub(crate) phase: GamePhase,
}

impl GameContext {
    pub fn from_encoding(string: &str) -> Self {
        let mut split = string.split_whitespace().into_iter();
        GameContext {
            phase: GamePhase::from_encoding(split.next().unwrap().chars().next().unwrap()),
            team:  Team::from_encoding(split.next().unwrap().chars().next().unwrap()),
            board: GameBoard::from_encoding(split.next().unwrap()),
        }
    }

    pub(crate) fn is_done(&self) -> bool {
        Team::Black.is_defeated(&self.board) || Team::White.is_defeated(&self.board)
    }

    pub fn toggle_team(&mut self) {
        self.team = self.team.get_opponent();
    }

    pub fn apply<'lifetime>(&mut self, turn: Turn) -> Result<(), Box<dyn Rule>> {
        turn.apply_safely(
            &self.team,
            &self.phase,
            &mut self.board
        )?;
        Ok(())
    }

    pub fn apply_unsafely<'lifetime>(&mut self, turn: Turn) {
        turn.apply_unsafely(
            &self.team,
            &mut self.board
        )
    }

    pub fn apply_unsafely_copied<'lifetime>(&self, turn: Turn) -> Self {
        let mut copied_context = self.clone();
        copied_context.apply_unsafely(turn);
        copied_context
    }
}