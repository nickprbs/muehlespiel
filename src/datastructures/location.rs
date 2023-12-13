// 1 - 24
pub type Location = u8;

pub trait GameBoardLocation {
    /**
    * Add a value to the location, but stay within the same ring. If we were to "overflow", wrap around.
    */
    fn add_wrapping_in_ring(&self, value_to_add: i16) -> Self;
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