mod datastructures;
mod iterators;
mod producer;
mod ai;

use datastructures::*;
use producer::complete_search::complete_search;
use std::{io::{Write, BufReader, BufRead, Error}, env, fs::File};
use std::sync::{Arc, Mutex};
use crate::datastructures::game_board::UsefulGameBoard;
use crate::ai::{Agent, MinimaxAgent};

fn main() {
    //ai_mode();
    complete_search_evaluation().unwrap();
}

fn ai_mode() {
    let mut history = Arc::new(Mutex::new(BoardHistoryMap::default()));
    let mut num_invocations = 0;

    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read line");
        let mut input_pieces: Vec<&str> = input.trim().split(" ").collect();

        let phase = match input_pieces[0] {
            "M" => Phase::MOVE,
            "P" => Phase::PLACE,
            _ => { panic!("Invalid phase") }
        };
        if phase == Phase::MOVE && num_invocations < 18 {
            num_invocations = 18
        }
        let team = Team::decode(String::from(input_pieces[1]));
        let board = GameBoard::decode(String::from(input_pieces[2]));

        // Also add the opponent's turn to the history
        {
            history.lock().unwrap().increment(board);
        }

        let result = MinimaxAgent::get_next_move(team, board, Arc::clone(&history), num_invocations);

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

        num_invocations += 2; // Add two, since the opponent was queried inbetween
    }
}

fn complete_search_evaluation() -> Result<(), Error> {
    let project_directory = env::current_dir()?;
    let input_file_path = project_directory.join("input_felder.txt");
    let output_file_path = project_directory.join("output.txt");

    let input_file = File::open(&input_file_path)?;
    let file_reader = BufReader::new(input_file);
    let mut output_file = File::create(&output_file_path)?;

    let (lost_states, won_states) = complete_search();

    dbg!(lost_states.len());

    for line in file_reader.lines() {

        let current_gameboard = GameBoard::decode(line?);
        let canonical_board = current_gameboard.get_representative();
        let mut output_line_content;
        if lost_states.contains(&canonical_board){
            output_line_content = 0;
        } else if won_states.contains(&canonical_board){
            output_line_content = 2;
        } else {
            output_line_content = 1;
        }
        writeln!(output_file, "{}", output_line_content)?;
    }

    Ok(())
}

