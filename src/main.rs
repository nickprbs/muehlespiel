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
use std::sync::{Arc, Mutex};
use crate::datastructures::game_board::UsefulGameBoard;
use crate::ai::{Agent, MinimaxAgent};

fn main() {
    //ai_mode();
    MinimaxAgent::get_next_move(Phase::MOVE, Team::WHITE, GameBoard::decode(String::from("BEBWEBEEEBWEWEBEBEEEBEWE")), Arc::new(Mutex::new(BoardHistoryMap::default())));
    MinimaxAgent::get_next_move(Phase::MOVE, Team::WHITE, GameBoard::decode(String::from("BEBWEWEEEBBEEEBEBEEEEEWE")), Arc::new(Mutex::new(BoardHistoryMap::default())));
    MinimaxAgent::get_next_move(Phase::MOVE, Team::WHITE, GameBoard::decode(String::from("BEBWEBEEEBEEEEEWBEEEBEWE")), Arc::new(Mutex::new(BoardHistoryMap::default())));
}

fn ai_mode() {
    let mut history = Arc::new(Mutex::new(BoardHistoryMap::default()));

    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read line");
        let mut input_pieces: Vec<&str> = input.trim().split(" ").collect();

        let phase = match input_pieces[0] {
            "M" => Phase::MOVE,
            "P" => Phase::PLACE,
            _ => { panic!("Invalid phase") }
        };
        let team = Team::decode(String::from(input_pieces[1]));
        let board = GameBoard::decode(String::from(input_pieces[2]));

        // Also add the opponent's turn to the history
        {
            history.lock().unwrap().increment(board);
        }

        let result = MinimaxAgent::get_next_move(phase, team, board, Arc::clone(&history));

        {
            let mut history = history.lock().unwrap();

            if result.take_from.is_some() {
                {
                    history.took_a_piece();
                }
            }
            // Add the board we produced to the history
            history.increment(board.apply(result, team));
        }
    }
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

