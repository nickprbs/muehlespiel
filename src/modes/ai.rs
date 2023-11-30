use std::env;
use crate::agents::Agent;
use crate::types::GameContext;
use crate::constants::AGENT;
use crate::iterators::TurnIterator;
use crate::types::game_board::QueryableGameBoard;

pub fn ai_mode() {
    let agent = AGENT {};

    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read line");

        let context = GameContext::from_encoding(input.as_str());

        let next_turn = agent.get_next_turn(&context);

        println!("{}", next_turn.encode());
    }
}

pub fn ai_debug_mode() {
    let agent = AGENT {};

    let args: Vec<String> = env::args().collect();
    let input = args.get(2).expect("No command string provided");

    let context = GameContext::from_encoding(input.as_str());
    context.board.print();

    println!("------------");
    println!("Possible moves (zero depth)");
    println!("------------");

    TurnIterator::new(
        &context, context.team.get_opponent()
    ).for_each(|turn| println!("{}", turn.encode()));

    println!("------------");
    println!("Chosen turn");
    println!("------------");

    let next_turn = agent.get_next_turn(&context);
    println!("{}", next_turn.encode());
}