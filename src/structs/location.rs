use crate::repeat_alignment;
use crate::SlideOffset;
use crate::constants::*;

/**
* Ring index: Rings are counted from 0 onwards from outer towards inner
* Alignment index: Counted from 0 clockwise, starting at top-center line
*/
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Location {
    pub ring: u8,
    pub alignment: u8
}

impl Location {
    pub fn to_enumeration_id(self) -> u8 {
        (self.ring * NUMBER_OF_ALIGNMENTS) + self.alignment
    }

    pub fn from_enumeration_id(id: u8) -> Self {
        let alignment = id % NUMBER_OF_ALIGNMENTS;
        Location {
            alignment,
            ring: (id - alignment) / NUMBER_OF_ALIGNMENTS,
        }
    }

    pub fn encode(&self) -> String {
        (self.to_enumeration_id() + 1).to_string()
    }

    pub fn is_valid(&self) -> bool {
        if self.ring >= NUMBER_OF_RINGS { return false; }
        if self.alignment >= NUMBER_OF_ALIGNMENTS { return false; }

        true
    }

    pub fn get_location_for(&self, slide_offset: &SlideOffset) -> Location {
        let (ring_offset, alignment_offset) = slide_offset.to_coordinate_offset();

        return Location {
            ring: ((self.ring as i8) + (ring_offset)) as u8,
            alignment: repeat_alignment(
                (self.alignment as i16) + (alignment_offset as i16)
            ),
        }
    }
}