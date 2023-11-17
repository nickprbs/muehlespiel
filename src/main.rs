mod datastructures;
use std::str::FromStr;

use datastructures::*;
mod millgame;
use millgame::*;

use crate::datastructures::GameBoard;



fn main() {
  let s : &str = &"WWBEEEEWEWBBBEWWBWWBBEBE";
  let outter_ring= &s[0..8];
  let middle_ring = &s[8..16];
  let inner_ring = &s[16..24];
  println!("outter: {}, middle: {}, inner: {}", outter_ring, middle_ring, inner_ring);
  let new_str = format!("{}{}{}", inner_ring, middle_ring, outter_ring);
  println!("my format: {}", new_str);
  let testboard = GameBoard {
    board: String::from(new_str),
    gamephase: Phase::Move,
    white_stones: 0,
    black_stones: 0,
    total_placed_black_stones: 0,
    total_placed_white_stones: 0,
  };
  testboard.print_gameboard();
  println!("Und jetzt das fromstr board:");
  match s.parse::<GameBoard>() {
    Ok(gameboard) => {
      gameboard.print_gameboard();
      println!("now tostring: ");
      println!("{}",gameboard.to_string()); 

      println!("{}", gameboard.to_string() == *s );
    } 
    Err(err) => {
      println!("Error parsing String!");
    }
  }

  
}


fn run_new_game() {
  let mut my_game= MillGame::new();
  my_game.run();
}