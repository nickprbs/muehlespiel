use crate::structs::GamePhase;
use crate::types::game_board::QueryableGameBoard;
use crate::types::GameBoard;

pub fn player_vs_player() {
    let mut board: GameBoard = vec![];
    board.print();
    GamePhase::Placing.execute(&mut board);
    println!("-------------------------------");
    GamePhase::Moving.execute(&mut board);
}