use crate::iterators::{MoveActionIterator, PlaceActionIterator, TakablesIterator};
use crate::structs::{Action, GamePhase, Team, Turn};
use crate::types::game_board::QueryableGameBoard;
use crate::types::GameContext;

pub struct TurnIterator<'lifetime> {
    action_iterator: Box<dyn Iterator<Item=Action>>,
    takeables_iterator: TakablesIterator,
    opponents_pieces_all_in_mill: bool,
    current_action: Option<Action>,
    context: &'lifetime GameContext,
    opponent: Team
}

impl <'lifetime> TurnIterator<'lifetime> {
    pub(crate) fn new(context: &'lifetime GameContext, opponent: Team) -> Self {
        let opponents_pieces_all_in_mill = context.board.iter()
            .filter(|piece| piece.owner == opponent)
            .all(|piece| {
                context.board.is_in_complete_mill(&piece.location, &opponent)
            });

        let action_iterator: Box<dyn Iterator<Item=Action>> = match context.phase {
            GamePhase::Placing => Box::new(
                PlaceActionIterator::new(context.board.clone(), context.team)
            ),
            GamePhase::Moving => Box::new(
                MoveActionIterator::new(context.board.clone(), context.team)
            ),
        };

        Self {
            action_iterator: Box::new(action_iterator),
            takeables_iterator: TakablesIterator::new(context.board.clone(), opponent),
            opponents_pieces_all_in_mill,
            current_action: None,
            context,
            opponent
        }
    }
}

impl Iterator for TurnIterator<'_> {
    type Item = Turn;

    fn next(&mut self) -> Option<Self::Item> {
        if let None = self.current_action {
            if let Some(next_action) = self.action_iterator.next() {
                self.current_action = Some(next_action);
            }
        }

        return if let Some(action) = self.current_action {
            if action.will_make_mill(self.context) {
                let takeable = self.takeables_iterator.next();

                match takeable {
                    None => {
                        // Go take the next action
                        self.current_action = self.action_iterator.next();
                        // Reset the takeables iterator, as we switched to a new action
                        self.takeables_iterator = TakablesIterator::new(
                            self.context.board.clone(),
                            self.opponent
                        );
                        // Execute again, since we jumped to a new action
                        self.next()
                    },
                    Some(takeable) => {
                        Some(Turn {
                            action,
                            piece_to_take: Some(takeable.location),
                        })
                    }
                }
            } else {
                let result = Some(Turn {
                    action,
                    piece_to_take: None
                });
                self.current_action = self.action_iterator.next();
                result
            }
        } else {
            None
        }
    }
}