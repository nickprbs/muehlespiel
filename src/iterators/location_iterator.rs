use crate::datastructures::Location;

pub struct LocationIterator {
    current_field_number: Location,
    forbidden_fields: Vec<Location>
}

impl LocationIterator {
    fn new() -> Self {
        Self {
            current_field_number: 1,
            forbidden_fields: vec![]
        }
    }

    fn with_forbidden(forbidden_fields: Vec<Location>) -> Self {
        Self {
            current_field_number: 1,
            forbidden_fields
        }
    }
}

impl Iterator for LocationIterator {
    type Item = Location;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_field_number <= 24 {
            let result = self.current_field_number;
            self.current_field_number += 1;

            // Is this a forbidden field?
            if self.forbidden_fields.contains(&result) {
                self.next()
            } else {
                Some(result)
            }

        } else {
            None
        }
    }
}

#[test]
fn test_location_iterator() {
    let actual: Vec<Location> = LocationIterator::new().collect();
    let expected: Vec<u8> = (1..=24).collect();
    assert_eq!(actual, expected);

    let forbidden = vec![1, 5, 8, 9, 10, 24];
    let actual: Vec<Location> = LocationIterator::with_forbidden(forbidden.clone()).collect();
    let expected: Vec<Location> = (1..=24)
        .filter(|loc| !forbidden.contains(loc))
        .collect();
    assert_eq!(actual, expected);
}