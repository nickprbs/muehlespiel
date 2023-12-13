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
}