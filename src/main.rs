mod datastructures;

mod millgame;
use millgame::*;



fn main() {
  let mut my_game= MillGame::new();
  my_game.run();
}
