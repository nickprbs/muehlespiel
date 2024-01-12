use crate::datastructures::Direction;

// 1 - 24
pub type Location = u8;

pub trait GameBoardLocation {
    /**
     * Add a value to the location, but stay within the same ring. If we were to "overflow", wrap around.
     */
    fn add_wrapping_in_ring(&self, value_to_add: i16) -> Self;
    fn apply_direction(&self, direction: Direction) -> Option<Location>;
    fn exists(&self) -> bool;

    fn to_ring_and_angle(self) -> (u8, u8);
}

impl GameBoardLocation for Location {
    fn add_wrapping_in_ring(&self, value_to_add: i16) -> Self {
        let self_normed_to_zero = *self - 1;

        // When we do mod later, the info about the ring gets lost. We need to know in which ring we are.
        let mut counting_down = self_normed_to_zero;
        let mut ring = 0;
        while counting_down >= 8 {
            ring += 1;
            counting_down -= 8;
        }

        (((self_normed_to_zero as i16 + value_to_add).rem_euclid(8)) + ring * 8 + 1) as Location
    }

    fn apply_direction(&self, direction: Direction) -> Option<Self> {
        let result: Option<i8> = match direction {
            Direction::Clockwise | Direction::CounterClockwise => {
                Some(
                    self.add_wrapping_in_ring(direction.get_location_offset() as i16) as i8
                )
            }
            _ => {
                // We can only move in-/outward, if there is a connection between those.
                // That's only the case if the location number is uneven
                if self % 2 == 1 {
                    Some(
                        *self as i8 + direction.get_location_offset()
                    )
                } else {
                    None
                }
            }
        };
        return if !(1..=24).contains(&result.unwrap_or(-1)) {
            None
        } else {
            match result {
                None => None,
                Some(result) => Some(result as Self)
            }
        };
    }

    fn exists(&self) -> bool {
        self >= &1 && self <= &24
    }

    fn to_ring_and_angle(self) -> (u8, u8) {
        let ring = match self {
            1..=8   => 0,
            9..=16  => 1,
            17..=24 => 2,
            _       => panic!("Invalid location")
        };

        let angle = (self - 1) % 8;

        (ring, angle)
    }
}

#[test]
fn test_to_ring_and_angle() {
    for location in 1..=24 {
        let test_loc: Location = location as u8;
        let (x,y) = test_loc.to_ring_and_angle();
        if test_loc < 9 {
            assert_eq!(x, 0);
        } else if test_loc >8 && test_loc < 17 {
            assert_eq!(x,1);
        } else if test_loc > 16 && test_loc < 25 {
            assert_eq!(x,2);
        }
        assert_eq!(y, (test_loc-1)%8 as u8 )
    }
}


#[test]
fn test_location_wrapping() {
    let expected: Location = 1;
    let actual = 8.add_wrapping_in_ring(1);
    assert_eq!(expected, actual);

    let expected: Location = 8;
    let actual = 1.add_wrapping_in_ring(-1);
    assert_eq!(expected, actual);

    let expected: Location = 24;
    let actual = 19.add_wrapping_in_ring(13);
    assert_eq!(expected, actual);

    let expected: Location = 15;
    let actual = 12.add_wrapping_in_ring(-5);
    assert_eq!(expected, actual);
}

#[test]
fn test_apply_direction() {
    assert_eq!(
        1,
        8.apply_direction(Direction::Clockwise).unwrap()
    );
    assert_eq!(
        7,
        8.apply_direction(Direction::CounterClockwise).unwrap()
    );
    assert_eq!(
        None,
        8.apply_direction(Direction::Inward)
    );
    assert_eq!(
        None,
        1.apply_direction(Direction::Outward)
    );
}