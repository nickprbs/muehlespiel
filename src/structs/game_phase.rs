use rand::Rng;
use scan_rules::scanner::Word;
use crate::constants::*;
use crate::{Turn, Location, Piece, SlideOffset, Team};
use crate::structs::Action;
use crate::structs::GamePhase::{Moving, Placing};
use crate::types::game_board::QueryableGameBoard;
use crate::types::{GameBoard, GameContext};

#[derive(Copy, Clone, PartialEq)]
pub enum GamePhase {
    Placing,
    Moving
}

impl GamePhase {

    pub(crate) fn from_encoding(character: char) -> Self {
        match character {
            'P' => Placing,
            'M' => Moving,
            _   => panic!("Unexpected character received for game phase")
        }
    }

    pub(crate) fn execute(self, board: &mut GameBoard) {
        match self {
            Placing => self.execute_placing_phase(board),
            Moving => self.execute_moving_phase(board)
        }
    }

    fn execute_placing_phase(self, board: &mut GameBoard) {
        println!("Welcome to this game of mill!");
        println!("Let's start by placing pieces, one at a time.");
        println!("-------------------------------");
        println!("Type 'start' and press enter to start the game (or type 'random' to start with random positions)");

        let_readln!(let mode: Word<String>);
        match mode.as_str() {
            "random" => randomly_place_pieces(board),
            "start"  => manually_place_pieces(board),
            _        => panic!("Not one of two modes (start/random) selected!")
        }

        println!("All pieces placed. Let's go!");
    }

    fn execute_moving_phase(self, board: &mut GameBoard) {let mut current_team = Team::White;

        while !Team::White.is_defeated(&board) &&
            !Team::Black.is_defeated(&board) {
            println!("It's {:?}'s turn to move a piece.", current_team);
            'input_loop: loop {
                println!("Please input the location of a piece to move:");
                let piece_to_modify = let_user_select_piece(&board);

                let turn: Turn = if current_team.is_allowed_to_fly(&board) {
                    println!("Please input the new location for this piece:");
                    let target_location = read_location();

                    let action = Action::Fly {
                        src_location: piece_to_modify.location,
                        target_location
                    };

                    // Ask the user for piece to remove
                    let piece_to_take = if action.will_make_mill(
                        &GameContext {
                            team: current_team,
                            phase: self,
                            board: board.clone()
                        }
                    ) {
                        println!("A mill has formed. Enter an opponent's piece to remove.");
                        let piece_to_remove = let_user_select_piece(&board);
                        Some(piece_to_remove.location)
                    } else {
                        None
                    };

                    Turn {
                        action,
                        piece_to_take
                    }
                } else {
                    println!("Please say where you want to move this piece (cw: clockwise, ccw: counterclockwise, in, out):");
                    let_readln!(let slide_offset: Word<String>);

                    let slide = match slide_offset.as_str() {
                        "cw"  => SlideOffset::Clockwise,
                        "ccw" => SlideOffset::CounterClockwise,
                        "in"  => SlideOffset::Inward,
                        "out" => SlideOffset::Outward,
                        _ => panic!()
                    };

                    let action = Action::Slide {
                        src_location: piece_to_modify.location,
                        slide: slide.clone()
                    };

                    // Ask the user for piece to remove
                    let piece_to_take = if action.will_make_mill(
                        &GameContext {
                            team: current_team,
                            phase: self,
                            board: board.clone()
                        }
                    ) {
                        println!("A mill has formed. Enter an opponent's piece to remove.");
                        let piece_to_remove = let_user_select_piece(&board);
                        Some(piece_to_remove.location)
                    } else {
                        None
                    };

                    Turn {
                        action,
                        piece_to_take
                    }
                };

                match turn.apply_safely(&current_team, &self, board) {
                    Err(problem) => {
                        println!("Action is invalid (Violated rule: {:?}). Try another position.", problem);
                    },
                    Ok(_) => {
                        board.print();
                        // Toggle currently active team
                        current_team = current_team.get_opponent();
                        // The action was valid, so continue
                        break 'input_loop;
                    }
                };
            };
        }

        println!("The game has ended.");

        let winner = match Team::White.is_defeated(&board) {
            true  => Team::Black,
            false => Team::White
        };

        println!("And the winner is: {:?}!!! Congrats!", winner);
    }
}

fn randomly_place_pieces(board: &mut GameBoard) {
    for i in 0..(PIECES_PER_TEAM * 2) {
        let current_team = match i % 2 {
            0 => Team::White,
            1 => Team::Black,
            _ => panic!()
        };

        'find_unoccupied_location: loop {
            let ring = rand::thread_rng().gen_range(0..NUMBER_OF_RINGS);
            let alignment = rand::thread_rng().gen_range(0..NUMBER_OF_ALIGNMENTS);

            let turn = Turn {
                action: Action::Place {
                    new_location: Location { ring, alignment },
                },
                piece_to_take: None
            };

            match turn.apply_safely(&current_team, &GamePhase::Placing, board) {
                Ok(_)  => break 'find_unoccupied_location,
                Err(_) => {} // Try again
            }
        }
    }

    board.print();
}

fn manually_place_pieces( board: &mut GameBoard) {
    for i in 0..(PIECES_PER_TEAM * 2) {
        let current_team = match i % 2 {
            0 => Team::White,
            1 => Team::Black,
            _ => panic!()
        };

        println!("It's {:?}'s turn to place a piece.", current_team);

        'input_loop: loop {
            println!("Enter the ring (0-2) and alignment on which you'd like to place the piece:");
            let new_location = read_location();

            let turn = Turn {
                action: Action::Place {
                    new_location
                },
                piece_to_take: None // TODO: Let user select this
            };

            match turn.apply_safely(&current_team, &GamePhase::Placing, board) {
                Err(problem) => {
                    println!("Action is invalid (Violated rule: {:?}). Try another position.", problem);
                },
                Ok(_) => {
                    board.print();
                    break 'input_loop;
                }
            };
        };
    }
}

fn read_location() -> Location {
    let_readln!(let ring: u8, ",", let alignment: u8);
    Location { ring, alignment }
}

fn let_user_select_piece(board: &GameBoard) -> &Piece {
    loop {
        let location = read_location();

        let piece = board.iter()
            .find(|piece| piece.location == location);

        match piece {
            Some(piece) => { return piece },
            None => {
                println!("Invalid location: No piece found. Try again:");
            }
        }
    }
}