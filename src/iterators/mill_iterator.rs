use crate::datastructures::Location;

pub struct MillIterator {
    current_angle: u8,
    current_ring: u8,
    current_side: u8
}

impl MillIterator {
    fn new() -> Self {
        Self {
            current_angle: 0,
            current_ring: 0,
            current_side: 0
        }
    }
}

impl Iterator for MillIterator {
    type Item = [Location; 3];

    fn next(&mut self) -> Option<Self::Item> {
        let enumerate_angles = self.current_angle <= 3;
        if enumerate_angles {
            // Enumerate mills along the leading lines of the board
            let result = [
                self.current_angle,
                self.current_angle + 8,
                self.current_angle + 16
            ];
            self.current_angle += 1;
            return Some(result);
        } else {
            // Enumerate mills along the rings
            if self.current_side <= 3 {
                // Enumerate mills along this ring, but at a new side
                self.current_side += 1;
            } else if self.current_ring <= 2 {
                // Go to a new ring
                self.current_ring += 1;
                self.current_side = 0;
            } else {
                return None;
            }

            todo!()
        }

        todo!()
    }
}