use std::collections::HashSet;
use std::mem::size_of;
use std::ops::{BitAnd, BitOr};
use itertools::Itertools;
use crate::datastructures::turn::TurnAction;
use super::{GameBoardLocation, Location, Team, Turn, team};
use crate::iterators::{BoardEquivalenceClassIterator, NeighboursIterator};
use super::Encodable;

pub type GameBoard = [u16; 3];
pub type CanonicalGameBoard = GameBoard;

#[test]
fn test_data_structure_size() {
    let size = size_of::<GameBoard>();
    println!("Size of GameBoard: {}", size);
    assert!(size <= 7); // Requirement in exercise sheet
}

pub trait UsefulGameBoard {

    fn from_pieces(black_locations: Vec<Location>, white_locations: Vec<Location>) -> Self;
    fn get_total_stone_amount(&self) -> u8;

    fn apply(&self, turn: Turn, current_team: Team) -> GameBoard;
    fn unapply(&self, turn: Turn, current_team: Team) -> GameBoard;

    // Places two bits (and only two!) at the given location
    fn place_bits_at(&self, bits: u8, location: Location) -> GameBoard;

    fn flipped(&self) -> GameBoard;
    fn rotated(&self, increments: u8) -> GameBoard;
    fn mirrored(&self) -> GameBoard;

    // Whether this board can be represented by the other through symmetries
    fn is_equivalent_to(&self, other: GameBoard) -> bool;

    // Get a unique and constant game board that represents this game board's equivalence class (one of 16)
    // That representative is determined by comparing the gameboard arrays and getting the lowest one
    // (first comparing the number of the outer most ring, if equal: the middle, if equal: the inner)
    fn get_representative(&self) -> CanonicalGameBoard;

    // Whether the game is finished
    // Happens when:
    // 1) One team only has two pieces
    // 2) One team can't move anymore
    fn is_game_done(&self) -> bool;
    fn is_occupied(&self, location: Location) -> bool;
    // Returns true if the given Location is part of any mill. Using the location vectors as arguments is more efficient
    // in more complex tasks, becuase they then need to be calculated only once.
    fn is_mill_at(&self, location: Location, black_locations:&Vec<u8>, white_locations:&Vec<u8>) -> bool;
    fn is_mill_at2(location: Location, black_locations: &Vec<Location>, white_locations: &Vec<Location>) -> bool {
        GameBoard::from([0; 3]).is_mill_at(location, black_locations, white_locations)
    }
    fn is_mill_at3(&self, location: Location) -> bool {
        let black_locations = self.get_piece_locations(Team::BLACK);
        let white_locations = self.get_piece_locations(Team::WHITE);
        self.is_mill_at(location, &black_locations, &white_locations)
    }

    fn has_only_mills(&self, team: Team) -> bool;
    fn get_num_pieces(&self, team: Team) -> u8;
    fn get_piece_locations(&self, team: Team) -> Vec<Location>;
    fn get_team_at(&self, location: Location) -> Option<Team>;
    fn get_free_neighbours(&self, location: Location) -> Vec<Location>;


    // Calculates the amount of all possible moves that produce the current gameboard when applied. This is specific for a
    // single location, meaning in the "previous" gameboard the move leading to the current gameboard is made with the provided
    // stone in the CURRENT gameboard.
    fn calc_previous_possibilities_amount (&self, _location: Location) -> u16;
}

impl UsefulGameBoard for GameBoard {

    fn from_pieces(black_locations: Vec<Location>, white_locations: Vec<Location>) -> Self {
        let mut output : GameBoard = [0,0,0];
        for location_black in black_locations {
            let mut ring: usize = 0;
            if location_black < 17 && location_black > 8 {
                ring = 1
            } else if location_black > 16 {
                ring = 2;
            }
            let mut lower_bit = 2_u16.pow(0);
            if location_black % 8 != 0 {
                 lower_bit = 1*2_u16.pow(16-(2*(location_black %8) as u32));
            }
            output[ring] = output[ring] | lower_bit;
        }
        for location_white in white_locations {
            let mut ring: usize = 0;
            if location_white < 17 && location_white > 8 {
                ring = 1
            } else if location_white > 16 {
                ring = 2;
            }
            let mut higher_bit = 2_u16.pow(1);
            if location_white % 8 != 0 {
                 higher_bit = 1*2_u16.pow((16-(2*(location_white %8))+1) as u32);
            }
            output[ring] = output[ring] | higher_bit;
        }
        output
    }

    fn get_total_stone_amount(&self) -> u8 {
        let mut amount: u8 = 0;
        amount += (self[0].count_ones() + self[1].count_ones() + self[2].count_ones()) as u8;
        amount
    }

    fn apply(&self, turn: Turn, current_team: Team) -> GameBoard {
        let board = match turn.action {
            TurnAction::Move { from, to } => {
                self.place_bits_at(0b00, from)
                    .place_bits_at(current_team.as_binary(), to)
            },
            TurnAction::Place { location } => {
                self.place_bits_at(current_team.as_binary(), location)
            }
        };

        return if let Some(take_from) = turn.take_from {
            board.place_bits_at(0b00, take_from)
        } else {
            board
        };
    }

    fn unapply(&self, turn: Turn, current_team: Team) -> GameBoard {
        let board = match turn.action {
            TurnAction::Move { from, to } => {
                self.place_bits_at(0b00, to)
                    .place_bits_at(current_team.as_binary(), from)
            },
            TurnAction::Place { location } => {
                self.place_bits_at(0b00, location)
            }
        };

        return if let Some(take_from) = turn.take_from {
            let opponent = current_team.get_opponent();
            board.place_bits_at(opponent.as_binary(), take_from)
        } else {
            board
        }
    }

    fn place_bits_at(&self, bits: u8, location: Location) -> GameBoard {
        let mut new_board = self.clone();

        let (ring, angle) = location.to_ring_and_angle();

        // "16 -" since we shift right, not left
        // "2 *" since each location is two bits
        // "- 2" because we want the last to bits to be at the right end of the field, not wrapped to the beginning
        let shift = 16 - (2 * angle) - 2;

        let patched_ring = new_board[ring as usize]
            .rotate_right(shift as u32)
            .bitand(0b1111111111111100)
            .bitor(bits as u16)
            .rotate_left(shift as u32);

        new_board[ring as usize] = patched_ring;

        new_board
    }

    // swaps inner and outer ring
    fn flipped(&self) -> GameBoard {
        let mut output = self.clone();
        output.swap(0, 2);
        output
    }

    // rotates 90° clockwise times 'increments'
    fn rotated(&self, increments: u8) -> GameBoard {
        let mut output = self.clone();
        let iterations = increments % 4;
        for _i in 1..=iterations {
            let old_board = output;
            let mut counter = 0;
            for elem in old_board {
                output[counter] = (elem >> 4) | (elem << (16 - 4));
                counter += 1;
            }
        }
        output
    }

    // performs a mirroring with the 90° mirror-axis.
    fn mirrored(&self) -> GameBoard {
        let old_num = self.clone();
        let mut output: [u16; 3] = [0, 0, 0];
        for i in 0..3 {
            let temp_old_num: u16 = old_num[i];
            let mut output_num: u16 = 0;
            for j in 0..8 {
                let higher_bitmask = temp_old_num & 2_u16.pow(15-2*j);
                let lower_bitmask: u16 = temp_old_num & 2_u16.pow(15-(2*j+1));
                let higher_bit: u16 = higher_bitmask.count_ones() as u16;
                let lower_bit: u16 = lower_bitmask.count_ones() as u16;
                if j == 0 {
                    output_num += higher_bitmask + lower_bitmask;
                } else if j == 4 {
                    output_num += higher_bitmask + lower_bitmask;
                } else {
                    output_num += higher_bit * 2_u16.pow(2*j-1) + lower_bit * 2_u16.pow(2*j-2);
                }
            }
            output[i] = output_num;
        }
        output
    }

    fn is_equivalent_to(&self, other: GameBoard) -> bool {
        BoardEquivalenceClassIterator::new(*self)
            .any(|equal_board| equal_board == other)
    }

    // Get an unique representative by pretending like we concatenated all three values of a game
    // board. Then, compare those in the equivalence class and return the smallest by concatenated
    // number.
    fn get_representative(&self) -> CanonicalGameBoard {
        BoardEquivalenceClassIterator::new(*self)
            .min_by(|board_a, board_b| {
                // Compare the two boards by first comparing their first ring, then second, then third
                board_a[0].cmp(&board_b[0]).then(
                    board_a[1].cmp(&board_b[1]).then(
                        board_a[2].cmp(&board_b[2])
                    )
                )
            })
            .expect("None found in equivalence class")
    }

    fn is_game_done(&self) -> bool {
        let by_pieces_taken = [Team::WHITE, Team::BLACK].into_iter()
            .any(|team| self.get_num_pieces(team) <= 2);

        let can_white_not_move = NeighboursIterator::new(self.get_piece_locations(Team::WHITE))
            .filter(|neighbour| { !self.is_occupied(*neighbour) })
            .dedup()
            .take(1) // Little optimization. Since we only care if there is at least one, just check if there is one.
            .count() == 0;
        let can_black_not_move = NeighboursIterator::new(self.get_piece_locations(Team::BLACK))
            .filter(|neighbour| { !self.is_occupied(*neighbour) })
            .dedup()
            .take(1)
            .count() == 0;
        let by_cant_move = can_white_not_move || can_black_not_move;

        by_pieces_taken || by_cant_move
    }

    fn is_occupied(&self, location: Location) -> bool {
        let (ring, angle) = location.to_ring_and_angle();

        let shifted: u16 = self.clone()[ring as usize] >> (16 - (2 * angle) - 2);
        let field = shifted & 0b11;

        field.count_ones() > 0
    }

    fn is_mill_at(&self, location: Location, black_locations:&Vec<u8>, white_locations:&Vec<u8>) -> bool {
        let mut output = false; 
        let (ring, angle) = location.to_ring_and_angle(); 
        let mut _team:Team = Team::WHITE;
        if black_locations.contains(&location){
            _team = Team::BLACK;
        }
        
        let lookup_vec= match _team {
            Team::BLACK =>  black_locations,
            Team::WHITE =>  white_locations
        };
        //case 1: edge-field
        if angle % 2 == 1 {
            if angle == 1 {
                output = (lookup_vec.contains(&(location-1 as u8)) && lookup_vec.contains(&(location+6 as u8)) ) || 
                         (lookup_vec.contains(&(location+1 as u8)) && lookup_vec.contains(&(location+2 as u8)) )
            } else if angle == 7 {
                output = (lookup_vec.contains(&(location-1 as u8)) && lookup_vec.contains(&(location-2 as u8)) ) || 
                         (lookup_vec.contains(&(location-6 as u8)) && lookup_vec.contains(&(location-7 as u8)) )
            } else {
                output = (lookup_vec.contains(&(location-1 as u8)) && lookup_vec.contains(&(location-2 as u8)) ) || 
                         (lookup_vec.contains(&(location+1 as u8)) && lookup_vec.contains(&(location+2 as u8)) )
            }
        }
        //case 2: middle-field
        else {
            //mill on the same ring?
            if angle == 0 {
                output = output || (lookup_vec.contains(&(location+1 as u8)) && lookup_vec.contains(&(location+7 as u8)))
            } else {
                output = output || (lookup_vec.contains(&(location+1 as u8)) && lookup_vec.contains(&(location-1 as u8)))
            }
            //mill across rings? 
            output = match ring {
                0 => output || (lookup_vec.contains(&(location+8 as u8)) && lookup_vec.contains(&(location+16 as u8))),
                1 => output || (lookup_vec.contains(&(location+8 as u8)) && lookup_vec.contains(&(location-8 as u8))),
                2 => output || (lookup_vec.contains(&(location-8 as u8)) && lookup_vec.contains(&(location-16 as u8))),
                _ => panic!("invalid location!")
            }; 
        }
        output
    }

    fn has_only_mills(&self, team: Team) -> bool {
        let team_vector = self.get_piece_locations(team);
        let black_locations = self.get_piece_locations(Team::BLACK);
        let white_locations = self.get_piece_locations(Team::WHITE);
        if team_vector.iter().all(|location| self.is_mill_at(*location, &black_locations, &white_locations)) {
            return true
        } else {
            return false
        }
    }

    fn get_num_pieces(&self, team: Team) -> u8 {
        let offset = match team {
            Team::BLACK => 0,
            Team::WHITE => 1
        };
        let mut count = 0;

        for ring in 0..=2 {
            let mut current_ring = self[ring].clone();
            current_ring = current_ring >> offset;
            count += current_ring & 0b00000001;
            for _angle in 0..7 {
                current_ring = current_ring >> 2;
                count += current_ring & 0b00000001
            }
        }

        count as u8
    }

    fn get_piece_locations(&self, _team: Team) -> Vec<Location> {
        let mut locations: Vec<u8> = Vec::new();
        let (higher, lower) = match _team {
            Team::BLACK => (0,1),
            Team::WHITE => (1,0)
        };
        let mut counter: u8 = 25; 
        for ring in 0..=2 {
            let mut bitmask: u16 = lower + 2*higher;
            let current_ring = self[2-ring]; 
            for _angle in 0..=7 {
                counter -=1; 

                if current_ring & bitmask != 0 {
                    locations.push(counter); 
                }
                bitmask = bitmask << 2;
                
            }
        } 

        locations
    }

    fn get_team_at(&self, location: Location) -> Option<Team> {
        let (ring, angle) = location.to_ring_and_angle();
        let ( higher, lower) = ( (self[ring as usize] & (1*2_u16.pow((15-2*angle) as u32))).count_ones() ,
                                        (self[ring as usize] & (1*2_u16.pow((14-2*angle) as u32)) ).count_ones());
        let team: Option<Team> = match (higher, lower) {
            (0,1) => Some(Team::BLACK),
            (1,0) => Some(Team::WHITE),
            (0,0) => None,
            _ => panic!("Invalid team ")
        };
        team
    }

    // panics if the location is empty
    fn get_free_neighbours(&self, location: Location) -> Vec<Location> {
        let mut free_neighbours: Vec<u8> = Vec::new();
        let team = self.get_team_at(location).expect("Can't calculate neighbours on an empty field!");
        let lookup_vec = self.get_piece_locations(team);
        if lookup_vec.len() <= 3 {
             free_neighbours = (1..=24).into_iter().filter(|field| !self.is_occupied(*field)).collect_vec();
        } else {
             free_neighbours = NeighboursIterator::new(vec![location])
            .filter(|neighbour| !self.is_occupied(*neighbour)).collect_vec();
        }
        free_neighbours
    }

    fn calc_previous_possibilities_amount (&self, _location: Location) -> u16 {
        let mut output: u16 = 0;
        let _team = self.get_team_at(_location);
        match _team {
            //if the field is empty there are no possible moves
            None => {return output },
            Some(_) => {}
        }
        let _team = _team.unwrap();
        let free_neighbours = self.get_free_neighbours(_location);
        let mill_flag = self.is_mill_at(_location, &self.get_piece_locations(Team::BLACK),
            &self.get_piece_locations(Team::WHITE));
        //current stone is part of a mill => previous move could have been closing it with this stone
        if mill_flag {
            //locations where the current stone could have been
            for neighbour in free_neighbours {
                // if closed => stone taken anywhere without mill
            }
        }


        output
    }

}

#[test]
fn test_game_board_from_pieces() {
    let black_pieces = vec![1,2, 3, 7, 10];
    let white_pieces = vec![4, 24, 23, 9];
    let expected_game_board = [
        0b0101011000000100,
        0b1001000000000000,
        0b0000000000001010
    ];
    let actual_game_board = GameBoard::from_pieces(black_pieces, white_pieces);
    assert_eq!(expected_game_board, actual_game_board);
}

#[test]
fn test_apply() {
    let turn = Turn {
        action: TurnAction::Move { from: 1, to: 2 },
        take_from: Some(16)
    };
    let board    = "WEEBEEBWWWEEWEEBWBEWEEEB";
    let expected = "EWEBEEBWWWEEWEEEWBEWEEEB";
    let actual = GameBoard::decode(String::from(board))
        .apply(turn, Team::WHITE);
    assert_eq!(expected, actual.encode());
}

#[test]
fn test_bit_placing() {
    let case =     "WEEBEEBWWWEEWEEBWBEWEEEB";
    let expected = "EWEBEEBWWEEEWEEBWBEWEEBB";
    let actual = GameBoard::decode(String::from(case))
        .place_bits_at(0b00, 1)
        .place_bits_at(0b10, 2)
        .place_bits_at(0b00, 10)
        .place_bits_at(0b01, 23)
        .place_bits_at(0b01, 24);
    assert_eq!(expected, actual.encode());
}

#[test]
fn test_get_num_pieces() {
    let case = GameBoard::decode(String::from("WEEBEEBWWWEEWEEBWBEWEEEB"));
    let expected_white_count = 7;
    let expected_black_count = 5;

    assert_eq!(expected_white_count, case.get_num_pieces(Team::WHITE));
    assert_eq!(expected_black_count, case.get_num_pieces(Team::BLACK));
}

#[test]
fn test_is_game_done() {
    let case = GameBoard::decode(String::from("WEEBEEBWWWEEWEEBWBEWEEEB"));
    assert_eq!(false, case.is_game_done());

    // by pieces taken
    let case = GameBoard::decode(String::from("WWEEEEEBEEEBEBEEEBBEEEEE"));
    assert_eq!(true, case.is_game_done());

    // by cant move
    let case = GameBoard::decode(String::from("BEBEEEEBWWBWBEEBBBEBBBBB"));
    assert_eq!(true, case.is_game_done());
}


#[test]
fn test_get_team_at() {
    let string = String::from("BWEEEEEWWEEEEEWWBBWEWBBB");
    let case = GameBoard::decode(String::from("BWEEEEEWWEEEEEWWBBWEWBBB"));
    let mut counter = 0;
    for char in string.chars(){
        counter += 1;
        if char == 'B' {
            assert_eq!(case.get_team_at(counter), Some(Team::BLACK));
        } else if char == 'W' {
            assert_eq!(case.get_team_at(counter), Some(Team::WHITE));
        } else if char == 'E' {
            assert_eq!(case.get_team_at(counter), None);
        }
    }
}

#[test]
fn test_get_free_neighbours () {
    let case = GameBoard::decode(String::from("BWEEEEEWWEEEEEWWBBWEWBBB"));
    assert_eq!(case.get_free_neighbours(1), Vec::new());
    assert_eq!(case.get_free_neighbours(19), vec![20,11]);
}


#[test]
fn test_is_mill_at() {
    //2 mills
    let case1 = GameBoard::decode(String::from("BWEEEEEWWEEEEEWWBBWEWBBB"));
    let case1_black = case1.get_piece_locations(Team::BLACK);
    assert!(case1_black.contains(&1));
    assert!(case1_black.contains(&17));
    assert!(case1_black.contains(&18));
    assert!(case1_black.contains(&22));
    assert!(case1_black.contains(&23));
    assert!(case1_black.contains(&24));
    let case1_white = case1.get_piece_locations(Team::WHITE);
    //0 mills
    let case2 = GameBoard::decode(String::from("WEEEEEEEBBWBWEEWWEEEEEEE"));
    let case2_black = case2.get_piece_locations(Team::BLACK);
    let case2_white = case2.get_piece_locations(Team::WHITE);
    //1 mill (other team)
    let case3 = GameBoard::decode(String::from("WWWEEEEWBBEEBEEEEEEEEEEE"));
    let case3_black = case3.get_piece_locations(Team::BLACK);
    let case3_white = case3.get_piece_locations(Team::WHITE);

    assert!(case1.is_mill_at(18, &case1_black, &case1_white));
    assert!(case1.is_mill_at(23, &case1_black, &case1_white));
    assert!(!case1.is_mill_at(15, &case1_black, &case1_white));
    assert!(case1.is_mill_at(24, &case1_black, &case1_white));

    assert!(!case2.is_mill_at(1, &case2_black, &case2_white));
    assert!(!case2.is_mill_at(5, &case2_black, &case2_white));
    assert!(!case2.is_mill_at(9, &case2_black, &case2_white));

    assert!(case3.is_mill_at(1, &case3_black, &case3_white));
    assert!(case3.is_mill_at(8, &case3_black, &case3_white));
    assert!(case3.is_mill_at(2, &case3_black, &case3_white));
    assert!(!case3.is_mill_at(24, &case3_black, &case3_white));

    let case4 = GameBoard::decode(String::from("EEEEEEEEWEBWWWWWEEEEEEEE"));
    let case4_black = case4.get_piece_locations(Team::BLACK);
    let case4_white = case4.get_piece_locations(Team::WHITE);
    assert!(!case4.is_mill_at(9, &case4_black, &case4_white));
    assert!(!case4.is_mill_at(11, &case4_black, &case4_white));
    assert!(case4.is_mill_at(12, &case4_black, &case4_white));
    assert!(case4.is_mill_at(13, &case4_black, &case4_white));
}

#[test]
fn test_has_only_mills () {
    let case1 = GameBoard::decode(String::from("EWEEEEEWWEEEEEWWBBWEWBBB"));
    assert!(case1.has_only_mills(Team::BLACK));
    assert!(!case1.has_only_mills(Team::WHITE));
}

impl Encodable for GameBoard {
    fn encode(&self) -> String {
        let whole_num: String = format!("{:016b}{:016b}{:016b}", self[0], self[1],self[2]);
        let mut output: String = String::new();
        for i in 0..24 {
            let higher_bit: char = whole_num.chars().nth(2*i).unwrap();
            let lower_bit: char = whole_num.chars().nth(2*i+1).unwrap();
            let field_char: char;
            if higher_bit == '0' && lower_bit == '0' {
                 field_char = 'E';
            } else if higher_bit == '0' && lower_bit == '1' {
                 field_char = 'B';
            } else if higher_bit == '1' && lower_bit == '0' {
                 field_char = 'W';
            } else {
                panic!("Error encoding Gameboard: Invalid binary representation found!");
            }
            output = output + &field_char.to_string();
        }
        output
    }

    //decodes a String into a 'game_board' type. Convention: 2bits per field: '00' <=> Empty, '01' <=> Black, '10' <=> White
    fn decode(string: String) -> Self {
        let mut char_counter: u16 = 0;
        let mut outer_ring_num: u16 = 0;
        let mut middle_ring_num: u16 = 0;
        let mut inner_ring_num: u16 = 0;

        for single_char in string.chars() {
            let current_exponent: u16 = 14 - (char_counter % 8) * 2;
            let mut lower_bit: u16 = 0;
            let mut higher_bit: u16 = 0;
            match single_char {
                'E' => {}
                'W' => { higher_bit = 1; }
                'B' => { lower_bit = 1; }
                _ => { panic!("Error parsing String: Invalid Token found."); }
            }
            if char_counter < 8 {
                outer_ring_num += lower_bit*2_u16.pow(current_exponent as u32) + higher_bit*2_u16.pow((current_exponent+1) as u32);
            } else if char_counter >= 8 && char_counter < 16 {
                middle_ring_num += lower_bit*2_u16.pow(current_exponent as u32) + higher_bit*2_u16.pow((current_exponent+1) as u32);
            } else if char_counter >= 16 && char_counter< 24 {
                inner_ring_num += lower_bit*2_u16.pow(current_exponent as u32) + higher_bit*2_u16.pow((current_exponent+1) as u32);
            } else {
                panic!("Invalid Format! String has to be 24 characters long!");
            }
            char_counter += 1;
        }
        [outer_ring_num, middle_ring_num, inner_ring_num]
    }
}

#[test]
fn test_decoding() {
    let cases = [
        (
            "EEEEEEEEEEEEEEEEEEEEEEEE",
            [
                0b0000000000000000,
                0b0000000000000000,
                0b0000000000000000,
            ]
        ),
        (
            "WEWWBBEWEWBWWBWWEEBWBEWE",
            [
                0b1000101001010010,
                0b0010011010011010,
                0b0000011001001000,
            ]
        ),
        (
            "BEWBEWBEWBEWBEWBEWBEWBEW",
            [
                0b0100100100100100,
                0b1001001001001001,
                0b0010010010010010
            ]
        )
    ];

    cases.iter()
        .for_each(|case| {
            assert_eq!(case.1, GameBoard::decode(String::from(case.0)));
        });
}

#[test]
fn test_encoding(){
    let cases = [
        (
            "EEEEEEEEEEEEEEEEEEEEEEEE",
            [
                0b0000000000000000,
                0b0000000000000000,
                0b0000000000000000,
            ]
        ),
        (
            "WEWWBBEWEWBWWBWWEEBWBEWE",
            [
                0b1000101001010010,
                0b0010011010011010,
                0b0000011001001000,
            ]
        ),
        (
            "BEWBEWBEWBEWBEWBEWBEWBEW",
            [
                0b0100100100100100,
                0b1001001001001001,
                0b0010010010010010
            ]
        )
    ];
    cases.iter()
        .for_each(|case| {
            assert_eq!(String::from(case.0), GameBoard::encode(&case.1));
        });
}

#[test]
fn test_mirroring() {
    let case0 = [
        0b0000000000000000,
        0b0000000000000000,
        0b0000000000000000
    ];
    let case1 = [
        0b1000101001010010,
        0b0010011010011010,
        0b0000011001001000
    ];
    let case2 = [
        0b0100100100100100,
        0b1001001001001001,
        0b0010010010010010
    ];

    assert_eq!(case0.mirrored(), [0b0000000000000000,0b0000000000000000,0b0000000000000000]);
    assert_eq!(case1.mirrored(), [0b1010000101101000,0b0010100110100110,0b0000100001100100]);
    assert_eq!(case2.mirrored(), [0b0100011000011000,0b1001100001100001,0b0010000110000110]);

}

#[test]
fn test_rotating() {
    let case0 = [
        0b0000000000000000,
        0b0000000000000000,
        0b0000000000000000
    ];
    let case1 = [
        0b1000101001010010,
        0b0010011010011010,
        0b0000011001001000
    ];

    let case2 = [
        0b0100100100100100,
        0b1001001001001001,
        0b0010010010010010
    ];

    assert_eq!(case0.rotated(1), [0b0000000000000000, 0b0000000000000000, 0b0000000000000000]);
    assert_eq!(case1.rotated(1), [0b0010100010100101, 0b1010001001101001, 0b1000000001100100]);
    assert_eq!(case2.rotated(1), [0b0100010010010010, 0b1001100100100100, 0b0010001001001001]);

    assert_eq!(case0.rotated(2), [0b0000000000000000, 0b0000000000000000, 0b0000000000000000]);
    assert_eq!(case1.rotated(2), [0b0101001010001010, 0b1001101000100110, 0b0100100000000110]);
    assert_eq!(case2.rotated(2), [0b0010010001001001, 0b0100100110010010, 0b1001001000100100]);

    assert_eq!(case0.rotated(3), [0b0000000000000000, 0b0000000000000000, 0b0000000000000000]);
    assert_eq!(case1.rotated(3), [0b1010010100101000, 0b0110100110100010, 0b0110010010000000]);
    assert_eq!(case2.rotated(3), [0b1001001001000100, 0b0010010010011001, 0b0100100100100010]);

    assert_eq!(case0.rotated(4), case0);
    assert_eq!(case1.rotated(4), case1);
    assert_eq!(case2.rotated(4), case2);
}