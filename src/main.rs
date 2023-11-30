// Simon Wazynski (M-Nr. 3659102)
#[macro_use] extern crate scan_rules;

mod test;
mod constants;
mod structs;
mod modes;
mod types;
mod iterators;
mod agents;

use std::env;
use crate::constants::*;
use crate::modes::*;
use crate::structs::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let default_mode = String::from(DEFAULT_MODE);
    let mode = &args.get(1).unwrap_or(&default_mode);

    match mode.as_str() {
        "player-vs-player" => player_vs_player(),
        "enumerate" => enumerate_actions_from_file(),
        "ai" => ai_mode(),
        "ai-debug" => ai_debug_mode(),
        "print-board" => print_board(),
        &_ => no_mode_provided(),
    }
}

fn no_mode_provided() {
    panic!("No (valid) mode provided (please add player-vs-player or enumerate as argument")
}

fn repeat_alignment(n: i16) -> u8 {
    let result = n % (NUMBER_OF_ALIGNMENTS as i16);

    if result < 0 {
        return (NUMBER_OF_ALIGNMENTS as i16 + result) as u8
    }

    result as u8
}

// Piece
// A token (in physical game usually a small round piece) that's placed on the board by players
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Piece {
    location: Location,
    owner: Team
}