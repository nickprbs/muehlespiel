use crate::structs::SlideOffset::*;

#[derive(Debug, Copy, Clone)]
pub enum SlideOffset {
    Clockwise,
    CounterClockwise,
    Inward,
    Outward
}

impl SlideOffset {
    pub fn to_coordinate_offset(&self) -> (i8, i8) {
        match self {
            Clockwise        => (0,  1),
            CounterClockwise => (0, -1),
            Inward           => (1,  0),
            Outward          => (-1, 0),
        }
    }
}

pub struct SlideOffsetIterator {
    current_offset: Option<SlideOffset>
}

impl SlideOffsetIterator {
    pub fn new() -> Self {
        SlideOffsetIterator {
            current_offset: Some(Clockwise)
        }
    }
}

impl Iterator for SlideOffsetIterator {
    type Item = SlideOffset;

    fn next(&mut self) -> Option<Self::Item> {
        let previous_offset = self.current_offset.clone();
        self.current_offset = match self.current_offset {
            Some(Clockwise)        => Some(CounterClockwise),
            Some(CounterClockwise) => Some(Inward),
            Some(Inward)           => Some(Outward),
            Some(Outward)          => None,
            None                   => return None
        };
        previous_offset
    }
}