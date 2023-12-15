use itertools::Itertools;
use crate::datastructures::{Direction, GameBoardLocation, Location};

/**
* Enumerates all neighbours of all locations given
* If two locations have the same neighbour, this neighbour is returned twice!
*/
pub struct NeighboursIterator {
    locations: Vec<Location>,
    locations_iter: Box<dyn Iterator<Item=Location>>,
    current_location: Option<Location>,
    neighbour_directions: Box<dyn Iterator<Item=Direction>>
}

impl NeighboursIterator {
    pub(crate) fn new(locations: Vec<Location>) -> Self {
        Self {
            locations: locations.clone(),
            locations_iter: Box::new(locations.into_iter()),
            current_location: None,
            neighbour_directions: Box::new(Direction::iter()),
        }
    }
}

impl Iterator for NeighboursIterator {
    type Item = Location;

    fn next(&mut self) -> Option<Self::Item> {
        if let None = self.current_location {
            self.current_location = self.locations_iter.next();
            return self.next();
        }

        let next_neighbour_direction = self.neighbour_directions.next();

        return match next_neighbour_direction {
            None => {
                self.current_location = self.locations_iter.next();
                self.neighbour_directions = Box::new(Direction::iter());
                match self.current_location {
                    None => None,
                    Some(_) => self.next(),
                }
            },
            Some(neighbour_direction) => {
                let mut neighbour_location = self.current_location?.apply_direction(neighbour_direction);

                return match neighbour_location {
                    None => {
                        self.next()
                    },
                    Some(neighbour_location) => {
                        // Test if that field actually exists
                        if !neighbour_location.exists() {
                            return self.next();
                        }

                        // Test if that field is actually free
                        if self.locations.contains(&neighbour_location) {
                            return self.next();
                        }

                        // Return it
                        Some(neighbour_location)
                    }
                }
            }
        };
    }
}

#[test]
fn test_neighbours_iterator() {
    let locations: Vec<Location> = vec![1];
    println!("Checking locations: {:?}", locations);
    let expected_neighbours: Vec<Location> = vec![8, 2, 9].into_iter().unique().sorted().collect();
    let actual_neighbours: Vec<Location> = NeighboursIterator::new(locations)
        .unique()
        .sorted()
        .collect();
    assert_eq!(expected_neighbours, actual_neighbours);

    let locations: Vec<Location> = vec![1, 2, 17, 24];
    println!("Checking locations: {:?}", locations);
    let expected_neighbours: Vec<Location> = vec![8, 9, 3, 18, 23].into_iter().unique().sorted().collect();
    let actual_neighbours: Vec<Location> = NeighboursIterator::new(locations)
        .unique()
        .sorted()
        .collect();
    assert_eq!(expected_neighbours, actual_neighbours);
}