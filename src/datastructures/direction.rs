use crate::datastructures::direction::Direction::{Clockwise, CounterClockwise, Inward, Outward};

#[derive(Debug, Copy, Clone)]
pub enum Direction {
    Clockwise,
    CounterClockwise,
    Inward,
    Outward
}

pub struct DirectionIter {
    current_direction: Option<Direction>
}

impl Iterator for DirectionIter {
    type Item = Direction;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.current_direction;

        self.current_direction = match self.current_direction {
            None => None,
            Some(Clockwise) => Some(CounterClockwise),
            Some(CounterClockwise) => Some(Inward),
            Some(Inward) => Some(Outward),
            Some(Outward) => None
        };

        result
    }
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

    pub fn iter() -> DirectionIter {
        DirectionIter {
            current_direction: Some(Clockwise)
        }
    }
}