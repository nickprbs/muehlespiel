use crate::types::game_board::QueryableGameBoard;
use std::fs;
use crate::Team;
use crate::iterators::{MoveActionIterator, TakablesIterator};
use crate::structs::GamePhase;
use crate::types::{GameBoard, GameContext};

pub fn enumerate_actions_from_file() {
    let team = Team::White;

    let file_contents = fs::read_to_string("./input_felder.txt")
        .expect("File could not be read. Please provide a file ./input_felder.txt");

    let mut game_boards = file_contents.split_terminator('\n');
    let mut output: String = String::new();

    while let Some(game_board) = game_boards.next() {
        let board = GameBoard::from_encoding(game_board);
        //board.print();
        let context = GameContext {
            board,
            team,
            phase: GamePhase::Moving
        };

        let total_count = enumerate_actions(&context).into_iter().count();

        let with_mill_count = enumerate_actions(&context).into_iter()
            .filter(|action| action.will_make_mill(&context))
            .count();
        let can_make_mill = with_mill_count > 0;

        let opponent = context.team.get_opponent();
        let takeable_pieces_count: usize = if can_make_mill {
                TakablesIterator::new(
                    context.board,
                    opponent
                ).into_iter()
                    .count()
            } else { 0 };

        let output_line = format!("{total_count} {with_mill_count} {takeable_pieces_count}\n");
        //print!("{}", output_line);
        output.push_str(output_line.as_str());
    }

    fs::write("./output.txt", output.trim()).unwrap();
}

fn enumerate_actions(context: &GameContext) -> MoveActionIterator {
    MoveActionIterator::new((context.board).clone(), context.team)
}