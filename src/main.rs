mod datastructures;
mod iterators;
mod producer;
mod ai;

use crate::producer::lost_positions::lost_positions_by_cant_move;
use crate::producer::lost_positions::lost_positions_by_pieces_taken;

use datastructures::*;
use datastructures::game_board::CanonicalGameBoard;
use std::{io::{Write, BufReader, BufRead, Error}, env, fs::File, collections::HashMap};
use crate::datastructures::game_board::UsefulGameBoard;
use crate::ai::{Agent, MinimaxAgent};

fn main() {
    // println!("{}", enumerate_lost_positions());
    let result = MinimaxAgent::get_next_move(Phase::MOVE, Team::WHITE, GameBoard::decode(String::from("EEWEEWEEWEWEBBBEEEEEEBEE")), ());
    println!("{}", result.encode());
}

fn past_main() -> Result<(), Error> {
    let project_directory = env::current_dir()?;
    let input_file_path = project_directory.join("input_felder.txt");
    let output_file_path = project_directory.join("output.txt");

    let input_file = File::open(&input_file_path)?;
    let file_reader = BufReader::new(input_file);
    let mut output_file = File::create(&output_file_path)?;
    let mut hash_map: HashMap<CanonicalGameBoard, u64> = HashMap::new();
    let mut line_counter: u64 = 0;

    for line in file_reader.lines() {
        line_counter += 1;
        let current_gameboard = GameBoard::decode(line?);
        let canonical_board = current_gameboard.get_representative();
        if !hash_map.contains_key(&canonical_board) {
            hash_map.insert(canonical_board, line_counter);
        }
        let output_line_content = String::from(format!("{}", hash_map.get(&canonical_board).unwrap()));
        writeln!(output_file, "{}", output_line_content)?;
    }

    Ok(())
}

