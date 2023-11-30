use std::env;
use crate::types::game_board::QueryableGameBoard;
use crate::types::GameBoard;

pub fn print_board() {
    let args: Vec<String> = env::args().collect();
    let board = args.get(2).expect("No board string provided");
    let board = GameBoard::from_encoding(board);

    board.print();
}