use std::collections::HashMap;
use fnv::{FnvBuildHasher, FnvHashMap, FnvHashSet};
use itertools::Itertools;
use crate::datastructures::{GameBoard, Location};
use crate::datastructures::game_board::{CanonicalGameBoard, UsefulGameBoard};
use crate::iterators::{NeighboursIterator, NRangeLocationsIterator};

pub fn lost_positions_by_cant_move() -> FnvHashSet<CanonicalGameBoard> {
    let our_positions = calc_our_positions();
    let with_neighbours_boards = calc_with_neighbours_boards(our_positions);
    let with_auxiliaries_boards = calc_with_auxiliaries_boards(with_neighbours_boards);
    with_auxiliaries_boards
}

#[inline]
fn calc_our_positions() -> FnvHashMap<CanonicalGameBoard, Vec<Location>> {
    let mut our_positions_board: HashMap<CanonicalGameBoard, Vec<Location>, FnvBuildHasher> = FnvHashMap::default();

    // TODO: Use NLocationsIterator nine times and prune in between n's
    let iterator = NRangeLocationsIterator::new(4, 9, (1..=24).collect());
    iterator
        .for_each(|white_locations| {
            let canonical = GameBoard::from_pieces(vec![], white_locations.clone()).get_representative();
            if !our_positions_board.contains_key(&canonical) {
                our_positions_board.insert(canonical, white_locations.clone());
            } // else just ignore it
        });

    our_positions_board
}

#[test]
fn test_calc_our_positions() {
    let our_positions = calc_our_positions();
    println!("{:?}", our_positions.len());
}


#[inline]
fn calc_with_neighbours_boards(originals: FnvHashMap<CanonicalGameBoard, Vec<Location>>) -> FnvHashMap<CanonicalGameBoard, (Vec<Location>, Vec<Location>)> {
    let mut with_neighbours_boards: HashMap<CanonicalGameBoard, (Vec<Location>, Vec<Location>), FnvBuildHasher>
        = FnvHashMap::default();

    originals.into_iter()
        .for_each(|(original_board, white_locations)| {
            let neighbours: Vec<Location> = NeighboursIterator::new(white_locations.clone()).unique().collect();

            if neighbours.len() <= 9 {
                let canonical = GameBoard::from_pieces(neighbours.clone(), white_locations.clone()).get_representative();
                if !with_neighbours_boards.contains_key(&canonical) {
                    with_neighbours_boards.insert(canonical, (white_locations, neighbours));
                }
            } // else ignore, since black can't lock white
        });

    with_neighbours_boards
}

#[test]
fn test_calc_neighbours() {
    let expected_size = 0;
    let actual_size = calc_with_neighbours_boards(FnvHashMap::default()).len();
    assert_eq!(expected_size, actual_size);

    let expected_size = 1;
    let mut map: FnvHashMap<CanonicalGameBoard, Vec<Location>> = FnvHashMap::default();
    map.insert(
        GameBoard::from_pieces(vec![], vec![1]).get_representative(),
        vec![1],
    );
    map.insert(
        GameBoard::from_pieces(vec![], vec![17]).get_representative(),
        vec![17],
    );
    let actual_size = calc_with_neighbours_boards(map).len();
    assert_eq!(expected_size, actual_size);

    let expected_size = 2;
    let mut map: FnvHashMap<CanonicalGameBoard, Vec<Location>> = FnvHashMap::default();
    map.insert(
        GameBoard::from_pieces(vec![], vec![2]).get_representative(),
        vec![2],
    );
    map.insert(
        GameBoard::from_pieces(vec![], vec![10]).get_representative(),
        vec![10],
    );
    let actual_size = calc_with_neighbours_boards(map).len();
    assert_eq!(expected_size, actual_size);

    let expected_size = 2;
    let mut map: FnvHashMap<CanonicalGameBoard, Vec<Location>> = FnvHashMap::default();
    map.insert(
        GameBoard::from_pieces(vec![], vec![2]).get_representative(),
        vec![2],
    );
    map.insert(
        GameBoard::from_pieces(vec![], vec![15]).get_representative(),
        vec![15],
    );
    let actual_size = calc_with_neighbours_boards(map).len();
    assert_eq!(expected_size, actual_size);
}


#[inline]
fn calc_with_auxiliaries_boards(originals: FnvHashMap<CanonicalGameBoard, (Vec<Location>, Vec<Location>)>) -> FnvHashSet<CanonicalGameBoard> {
    let mut final_game_boards: FnvHashSet<CanonicalGameBoard> = FnvHashSet::default();

    originals.iter()
        .for_each(|(orig_canonical, (orig_white, orig_black))| {
            let num_auxilary_items = 9_u8.saturating_sub(orig_black.len() as u8);

                let mut forbidden_fields: Vec<Location> = orig_white.clone();
                forbidden_fields.append(&mut orig_black.clone());
                let free_fields = (1..=24)
                    .filter(|loc| !forbidden_fields.contains(loc))
                    .collect();

                let auxiliaries = NRangeLocationsIterator::new(0, num_auxilary_items, free_fields);
                auxiliaries.into_iter()
                    .map(|mut auxiliaries| {
                        let mut black_locations = orig_black.clone();
                        black_locations.append(&mut auxiliaries);
                        GameBoard::from_pieces(black_locations, orig_white.clone())
                    })
                    .map(|board| board.get_representative())
                    .for_each(|representative| {
                        final_game_boards.insert(representative);
                    });
        });

    final_game_boards
}

#[test]
fn test_auxiliaries() {
    let mut both_nine: FnvHashMap<CanonicalGameBoard, (Vec<Location>, Vec<Location>)> = FnvHashMap::default();
    let black: Vec<Location> = vec![7,13,14,16,9,11,19,21,23];
    let white: Vec<Location> = vec![1,2,3,4,5,6,8,15,20];
    both_nine.insert(
        GameBoard::from_pieces(black.clone(), white.clone()).get_representative(),
        (white, black)
    );
    let expected_size = 1;
    let actual_size = calc_with_auxiliaries_boards(both_nine).len();
    assert_eq!(expected_size, actual_size);

    let mut black_can_place_one: FnvHashMap<CanonicalGameBoard, (Vec<Location>, Vec<Location>)> = FnvHashMap::default();
    let black: Vec<Location> = vec![7,13,14,16,9,11,23];
    let white: Vec<Location> = vec![1,2,3,4,5,6,8,15];
    black_can_place_one.insert(
        GameBoard::from_pieces(black.clone(), white.clone()).get_representative(),
        (white, black)
    );
    let expected_size = 36 - 4 - 10;
    let actual_size = calc_with_auxiliaries_boards(black_can_place_one).len();
    assert_eq!(expected_size, actual_size);
}

#[test]
fn test_by_cant_move() {
    let boards = lost_positions_by_cant_move();
    let count = boards.len();
    println!("Count of Lost by can't move positions: {}", count);
    println!("First boards:");
    for board in boards.iter().take(10) {
        println!("{:?}", board);
    }
    assert_eq!(count, 567794);
}