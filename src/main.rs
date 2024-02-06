mod datastructures;
mod iterators;
mod producer;
mod ai;

use datastructures::*;
use producer::complete_search::complete_search;
use std::{io::{Write, BufReader, BufRead, Error}, env, fs::File, thread};
use std::sync::{Arc, Mutex, RwLock};
use std::time::SystemTime;
use crate::datastructures::game_board::UsefulGameBoard;
use crate::ai::{Agent, MinimaxAgent};

fn main() {
    ai_mode();
    //complete_search_evaluation().unwrap();
}

fn ai_mode() {
    let mut history = Arc::new(Mutex::new(BoardHistoryMap::default()));
    let mut num_invocations = 0;

    let lost_states_for_white = Arc::new(RwLock::new(CanonicalBoardSet::default()));
    let won_states_for_black = Arc::new(RwLock::new(CanonicalBoardSet::default()));
    let lost_states_ref = Arc::clone(&lost_states_for_white);
    let won_states_ref = Arc::clone(&won_states_for_black);
    thread::spawn(move|| {
        complete_search(lost_states_ref, won_states_ref);
        eprintln!("Completed complete search :)");
    });

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

        let result = MinimaxAgent::get_next_move(
            team,
            board,
            Arc::clone(&history),
            Arc::clone(&lost_states_for_white),
            Arc::clone(&won_states_for_black),
            num_invocations
        );

        {
            let mut history = history.lock().unwrap();

            if result.take_from.is_some() {
                history.took_a_piece();
            }
            // Add the board we produced to the history
            history.increment(board.apply(result, team));
        }

        num_invocations += 2; // Add two, since the opponent was queried inbetween
    }
}

fn complete_search_evaluation() -> Result<(), Error> {
    let start_time = SystemTime::now();

    let project_directory = env::current_dir()?;
    let input_file_path = project_directory.join("input_felder.txt");
    let output_file_path = project_directory.join("output.txt");

    let input_file = File::open(&input_file_path)?;
    let file_reader = BufReader::new(input_file);
    let mut output_file = File::create(&output_file_path)?;

    let lost_states = Arc::new(RwLock::new(CanonicalBoardSet::default()));
    let won_states = Arc::new(RwLock::new(CanonicalBoardSet::default()));
    complete_search(Arc::clone(&lost_states), Arc::clone(&won_states));

    let lost_states = lost_states.read().unwrap();
    let won_states = won_states.read().unwrap();

    dbg!(lost_states.len());
    dbg!(won_states.len());

    for line in file_reader.lines() {

        let current_gameboard = GameBoard::decode(line?);
        let canonical_board = current_gameboard.get_representative();
        let inverted_board = current_gameboard.invert_teams().get_representative();

        let mut output_line_content;
        if lost_states.contains(&canonical_board){
            output_line_content = 0;
        } else if won_states.contains(&inverted_board){
            output_line_content = 2;
        } else {
            output_line_content = 1;
        }
        writeln!(output_file, "{}", output_line_content)?;
    }

    println!("Took {}s", start_time.elapsed().unwrap().as_secs());

    Ok(())
}