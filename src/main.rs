mod datastructures;
mod iterators;
mod producer;
mod ai;

use crate::producer::lost_positions::lost_positions_by_cant_move;
use crate::producer::lost_positions::lost_positions_by_pieces_taken;

use datastructures::*;
use datastructures::game_board::CanonicalGameBoard;
use producer::complete_search::complete_search;
use std::{io::{Write, BufReader, BufRead, Error}, env, fs::File, collections::HashMap};
use crate::datastructures::game_board::UsefulGameBoard;
use crate::ai::{Agent, MinimaxAgent};

fn main() {
    // println!("{}", enumerate_lost_positions());
    let result = MinimaxAgent::get_next_move(Phase::MOVE, Team::WHITE, GameBoard::decode(String::from("EEWWWBBBEEEEEEEEEEEEEEEE")), ());
    println!("{}", result.encode());
    comeplete_search_evaluation();
}

fn comeplete_search_evaluation() -> Result<(), Error> {
    let project_directory = env::current_dir()?;
    let input_file_path = project_directory.join("input_felder.txt");
    let output_file_path = project_directory.join("output.txt");

    let input_file = File::open(&input_file_path)?;
    let file_reader = BufReader::new(input_file);
    let mut output_file = File::create(&output_file_path)?;

    let (loser, winner) = complete_search();
    eprintln!("loser:{}, winner:{}", loser.len(), winner.len());

    for line in file_reader.lines() {

        let current_gameboard = GameBoard::decode(line?);
        let canonical_board = current_gameboard.get_representative();
        let mut output_line_content;
        if loser.contains(&canonical_board){
            output_line_content = 0;
        } else if winner.contains(&canonical_board){
            output_line_content = 2;
        } else {
            output_line_content = 1;
        }
        writeln!(output_file, "{}", output_line_content)?;
    }

    Ok(())
}

