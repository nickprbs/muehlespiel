use crate::datastructures::Location;
use itertools::Itertools;

pub struct NLocationsIterator {
    permutations_iterator: Box<dyn Iterator<Item=Vec<Location>>>
}
impl NLocationsIterator {
    pub(crate) fn new(n: usize, free_fields: Vec<Location>) -> Self {
        Self {
            permutations_iterator: Box::new(free_fields.into_iter().combinations(n).into_iter())
        }
    }
}
impl Iterator for NLocationsIterator {
    type Item = Vec<Location>;
    fn next(&mut self) -> Option<Self::Item> {
        self.permutations_iterator.next()
    }
}

#[test]
fn text_n_locations_iterator() {
    let all_free_fields: Vec<Location> = (1..=24).collect();

    // n = 1
    let expected_locations: Vec<Vec<Location>> = (1..=24).map(|n| vec![n]).collect();
    let actual_locations: Vec<Vec<Location>> = NLocationsIterator::new(1, all_free_fields.clone()).collect();
    assert_eq!(expected_locations, actual_locations);

    // n = 2
    let expected_count = 276;
    let actual_count = NLocationsIterator::new(2, all_free_fields.clone()).count();
    assert_eq!(expected_count, actual_count);

    // n = 5
    let expected_count = 42504;
    let actual_count = NLocationsIterator::new(5, all_free_fields.clone()).count();
    assert_eq!(expected_count, actual_count);
}


pub struct NRangeLocationsIterator {
    n_iterator: Box<dyn Iterator<Item=u8>>,
    n_locations_iterator: NLocationsIterator,
    free_fields: Vec<Location>
}

impl NRangeLocationsIterator {
    pub fn new(min_n: u8, max_n: u8, free_fields: Vec<Location>) -> Self {
        Self {
            n_iterator: Box::new(((min_n + 1)..=max_n).into_iter()),
            n_locations_iterator: NLocationsIterator::new(min_n as usize, free_fields.clone()),
            free_fields
        }
    }
}

impl Iterator for NRangeLocationsIterator {
    type Item = Vec<Location>;

    fn next(&mut self) -> Option<Self::Item> {
        let next_location_set = self.n_locations_iterator.next();

        return match next_location_set {
            Some(next_location_set) => {
                Some(next_location_set)
            },
            None => {
                if let Some(next_n) = self.n_iterator.next() {
                    self.n_locations_iterator = NLocationsIterator {
                        permutations_iterator: Box::new(
                            NLocationsIterator::new(next_n as usize, self.free_fields.clone())
                        ),
                    };
                    self.next()
                } else {
                    None
                }
            },
        };
    }
}

#[test]
fn test_n_range_locations_iterator() {
    let all_free_fields: Vec<Location> = (1..=24).collect();

    // n = 1
    let expected: Vec<Vec<Location>> = (1..=24).map(|n| vec![n]).collect();
    let actual: Vec<Vec<Location>> = NRangeLocationsIterator::new(1, 1, all_free_fields.clone()).collect();
    assert_eq!(expected, actual);

    // 1 <= n <= 2
    let expected_count = 24 + 276;
    let actual_count = NRangeLocationsIterator::new(1, 2, all_free_fields.clone()).count();
    assert_eq!(expected_count, actual_count);

    // 2 <= n <= 9
    let expected_count = 276 + 2024 + 10626 + 42504 + 134596 + 346104 + 735471 + 1307504;
    let actual_count = NRangeLocationsIterator::new(2, 9, all_free_fields.clone()).count();
    assert_eq!(expected_count, actual_count);
}