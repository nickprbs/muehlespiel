use itertools::Itertools;
use crate::datastructures::{GameBoard, Location, Phase, Team, Turn, TurnAction};
use crate::datastructures::game_board::UsefulGameBoard;

pub struct TurnIterator {
    phase: Phase,
    occupied_locations: Vec<Location>,
    own_locations: Vec<Location>,
    opponent_locations: Vec<Location>,

    placing_current_location: Option<Location>,


}

impl TurnIterator {
    pub(crate) fn new(phase: Phase, team: Team, board: GameBoard) -> Self {
        let own_locations = board.get_piece_locations(team);
        let opponent_locations = board.get_piece_locations(team.get_opponent());

        let mut occupied_locations = own_locations.clone();
        occupied_locations.append(&mut opponent_locations.clone());

        let first_free_location: Option<Location> = (1_u16..=24_u16).collect_vec().into_iter()
            .map(|loc| loc as Location)
            .filter(|loc| { !occupied_locations.contains(loc) })
            .sorted()
            .nth(0);

        Self {
            phase,
            occupied_locations,
            own_locations,
            opponent_locations,

            placing_current_location: first_free_location
        }
    }
}

impl Iterator for TurnIterator {
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

impl TurnIterator {
    fn next_placing_turn(&mut self) -> Option<Self::Item> {
        match self.placing_current_location {
            None => None,
            Some(current_location) => {
                let turn = Turn {
                    action: TurnAction::Place { location: current_location },
                    take_from: None
                };

                self.placing_current_location = if current_location < 24 {
                    Some(current_location + 1)
                } else { None };

                Some(turn)
            }
        }
    }

    fn next_moving_turn(&mut self) -> Option<Self::Item> {
        todo!()
    }
}