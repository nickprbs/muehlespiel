mod datastructures;
use datastructures::*; 
use datastructures::game_board::CanonicalGameBoard;
use std::{io::{Write, BufReader, BufRead, Error}, env, fs::File,collections::HashMap, ptr::hash};
use crate::datastructures::game_board::UsefulGameBoard;

fn main() -> Result<(), Error>{

  let project_directory = env::current_dir()?;
  let input_file_path = project_directory.parent().unwrap().join("input_felder.txt");
  let output_file_path = project_directory.parent().unwrap().join("output.txt");
    
  let input_file = File::open(&input_file_path)?;
  let file_reader = BufReader::new(input_file);
  let mut output_file = File::create(&output_file_path)?;
  let mut hashMap: HashMap<CanonicalGameBoard, u64> = HashMap::new();
  let mut line_counter:u64  = 0; 
  for line in file_reader.lines(){
    line_counter +=1; 
    let line_content = line?;
    let current_line_as_gameboard : GameBoard = GameBoard::decode(line_content); 
    let canonical_board: CanonicalGameBoard = current_line_as_gameboard.get_representative();
    if !hashMap.contains_key(&canonical_board){
      hashMap.insert(canonical_board, line_counter);
    }   
    let output_line_content= String::from(format!("{}", hashMap.get(&canonical_board).unwrap()));
    writeln!(output_file, "{}", output_line_content); 
    }
    Ok(())
}