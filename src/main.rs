mod datastructures;
mod iterators;

use datastructures::*;
use datastructures::game_board::CanonicalGameBoard;
use iterators::LostPositionsByCantMoveIterator;
use std::{io::{Write, BufReader, BufRead, Error}, env, fs::File,collections::HashMap};
use std::collections::HashSet;
use crate::datastructures::game_board::UsefulGameBoard;
use crate::iterators::LostPositionsByPiecesTakenIterator;

fn main() -> Result<(), Error>{
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
        if !hash_map.contains_key(&canonical_board){
            hash_map.insert(canonical_board, line_counter);
        }
        let output_line_content = String::from(format!("{}", hash_map.get(&canonical_board).unwrap()));
        writeln!(output_file, "{}", output_line_content)?;
    }

    Ok(())
}

fn enumerate_lost_positions(loosing_team:Team) ->(u64,u64) {
    let mut canonicals: HashSet<CanonicalGameBoard> = HashSet::new();
    
    let iterator_lost_piece = LostPositionsByPiecesTakenIterator::new(loosing_team.clone());
    let iterator_lost_move = LostPositionsByCantMoveIterator::new(loosing_team.clone());
    let mut counter_lost_piece:u64=0;
    let mut counter_lost_move:u64=0;
    iterator_lost_piece.for_each(|board| {
        counter_lost_move+=1;
        let temp_repres=board.get_representative();
        canonicals.insert(temp_repres);
    });
    iterator_lost_move.for_each(|board| {
        counter_lost_move+=1;
        let temp_repres=board.get_representative();
        canonicals.insert(temp_repres);
    });
    let mut _result=(counter_lost_move,counter_lost_piece);
    _result
}