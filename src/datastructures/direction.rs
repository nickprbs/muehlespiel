use crate::datastructures::direction::Direction::{Clockwise, CounterClockwise, Inward, Outward};

#[derive(Debug)]
pub enum Direction {
    Clockwise,
    CounterClockwise,
    Inward,
    Outward
}

impl Direction {
    pub fn get_location_offset(&self) -> i8 {
        match self {
            Clockwise => 1,
            CounterClockwise => -1,
            Inward => 8,
            Outward => -8
        }
    }

    pub fn iter() -> impl Iterator<Item=Direction> {
        [Clockwise, CounterClockwise, Inward, Outward].into_iter()
    }
}