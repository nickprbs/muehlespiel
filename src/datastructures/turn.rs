use crate::datastructures::{Encodable, Location};

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct Turn {
    pub action: TurnAction,
    pub take_from: Option<Location>
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub enum TurnAction {
    Move {
        from: Location,
        to: Location
    },
    Place {
        location: Location
    }
}

impl Encodable for Turn {
    fn encode(&self) -> String {
        return if let Some(take_from) = self.take_from {
            format!("{} T {}", self.action.encode(), take_from)
        } else {
            self.action.encode()
        }
    }

    fn decode(string: String) -> Self {
        todo!()
    }
}

impl Encodable for TurnAction {
    fn encode(&self) -> String {
        match self {
            TurnAction::Move { from, to } => format!("M {from} {to}"),
            TurnAction::Place { location } => format!("P {location}"),
        }
    }

    fn decode(string: String) -> Self {
        todo!()
    }
}