use crate::datastructures::Location;

pub struct Turn {
    pub action: TurnAction,
    pub take_from: Option<Location>
}

pub enum TurnAction {
    Move {
        from: Location,
        to: Location
    }
    // TODO: Add Placing
}