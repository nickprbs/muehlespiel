use crate::datastructures::Location;



#[derive(Debug)]
pub struct LocationIterator {
    current_field_number: Location,
    forbidden_fields: Vec<Location>
}

impl LocationIterator {
    pub(crate) fn new() -> Self {
        Self {
            current_field_number: 1,
            forbidden_fields: vec![]
        }
    }

    pub(crate) fn with_forbidden(forbidden_fields: Vec<Location>) -> Self {
        Self {
            current_field_number: 1,
            forbidden_fields
        }
    }

    pub(crate) fn with_allowed(allowed_fields: Vec<Location>) -> Self {
        let forbidden_fields = (1..=24)
            .filter(|loc| !allowed_fields.contains(&loc))
            .collect();
        Self {
            current_field_number: 1,
            forbidden_fields
        }
    }
}

impl Iterator for LocationIterator {
    type Item = Location;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.current_field_number <= 24 {
                let result = self.current_field_number;
                self.current_field_number += 1;

                // Is this a forbidden field?
                if !self.forbidden_fields.contains(&result) {
                    return Some(result);
                } // else loop again
            } else {
                return None;
            }
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