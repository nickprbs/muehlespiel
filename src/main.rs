mod datastructures;
use std::str::FromStr;
use std::fs::File;
use std::io::{Write, BufReader, BufRead, Error};
use std::io::prelude::*;
use std::env;
use datastructures::*;
mod millgame;
use millgame::*;

use crate::datastructures::GameBoard;



fn main() {
  read_and_write_move_information();
}


fn run_new_game() {
  let mut my_game= MillGame::new();
  my_game.run();
}

fn read_and_write_move_information() -> Result<(), Error> {
  let project_directory = env::current_dir()?;
  let input_file_path = project_directory.parent().unwrap().join("input_felder.txt");
  let output_file_path = project_directory.parent().unwrap().join("output_felder.txt");

 let input_file = File::open(&input_file_path)?;
 let file_reader = BufReader::new(input_file);

 let mut output_file = File::create(&output_file_path)?;
 for line in file_reader.lines(){
  let line_content = line?;
  let mut white_moves=0;
  let mut white_mills=0;
  let mut takeable_stones=0;
    match line_content.parse::<GameBoard>() {
      Ok(gameboard) => {
         white_moves= gameboard.possible_moves_amount(Player::White);
         white_mills= gameboard.possible_mill_amount(Player::White);
         takeable_stones= gameboard.takeable_opponent_amount(Player::White);
      }
      Err(err) => {println!("Error parsing string to gameboard!")}
    }
   let output_line_content= String::from(format!("{} {} {}", white_moves, white_mills, takeable_stones));
   writeln!(output_file, "{}", output_line_content);
}
Ok(())
}