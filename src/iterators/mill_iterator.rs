use crate::datastructures::GameBoardLocation;
use crate::datastructures::Location;

// TODO: Go hardcode this

pub struct MillIterator {
    current_angle: u8,
    current_ring: u8,
    current_side: u8
}

impl MillIterator {
  pub  fn new() -> Self {
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
        return if enumerate_angles {
            // Enumerate mills along the leading lines of the board
            let angle_offset = self.current_angle * 2;
            let result = [
                angle_offset + 1, // + 0*8 + 1
                angle_offset + 9, // + 1*8 + 1
                angle_offset + 17 // + 2*8 + 1
            ];
            self.current_angle += 1;
            Some(result)
        } else {
            // We're done
            if self.current_ring == 3 {
                return None;
            }
            
            // Enumerate mills along the rings
            let center_location = (self.current_ring * 8) + (self.current_side * 2) + 1;
            let result = [
                center_location.add_wrapping_in_ring(-1),
                center_location,
                center_location.add_wrapping_in_ring(1),
            ];
            
            println!("Ring: {}, side: {}", self.current_ring, self.current_side);

            if self.current_side < 3 {
                // Enumerate mills along this ring, but at a new side
                self.current_side += 1;
            } else {
                // Go to a new ring
                self.current_ring += 1;
                self.current_side = 0;
            }

            Some(result)
        };
    }
}

#[test]
fn test_mill_iterator() {
    let expected_samples = [
        [1, 9, 17], [3, 11, 19], [5, 13, 21], [7, 15, 23],
        [8, 1, 2], [14, 15, 16], [24, 17, 18]
    ];
    let actual_all_mills: Vec<[Location; 3]> = MillIterator::new().collect();
    println!("{:?}", actual_all_mills);
    for sample in expected_samples {
        assert!(actual_all_mills.contains(&sample));
    }
    assert_eq!(actual_all_mills.len(), 4 + 3*4);
}