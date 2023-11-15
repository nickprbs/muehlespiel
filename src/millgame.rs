use crate::datastructures::*;
use std::io; 


// compiler vorschläge testen 
//remove pub from all MillGame attributes
pub struct MillGame {
    gameboard: GameBoard,
    past_moves: Vec<String>,
    winner: Option<Player>,
    turn: Player,
    turn_counter: u32,
    game_over: bool,

}
impl MillGame {
    pub fn new() -> MillGame {
        let temp_last_move: Vec<String> = Vec::new();
        let millgame: MillGame = MillGame{
            gameboard: GameBoard::new(),
            past_moves: temp_last_move,
            winner: None,
            turn: Player::White,
            turn_counter: 0,
            game_over: false,
        };
        millgame
    }
    
    // retrieves input from player, which stone he wants to take; if possible the stone gets taken
    fn take_opponent_stone(&mut self, taker_board: &mut GameBoard) {
        let mut not_valid = true;
        let current_turn: Player = self.turn.clone(); 
        let opponent = get_other_player(current_turn);
        let mut index: u8; 
        println!("You closed a Mill! Which stone do you want to delete?");
        while not_valid {
            index = get_user_input_as_number(); 
            if index == 0 {
                self.game_over = true; 
                let temp_winner: Player = get_other_player(self.turn.clone());
                self.winner = Some(temp_winner);
                break; 
            } else {
            match self.gameboard.get_player_at(index) {
                None => {
                    println!("This field is empty! Please try again.");
                    continue;
                }
                Some(player) if player == current_turn => {
                    println!("Stone must be from the opponent! Please try again.");
                    continue;
                }
                Some(player) if player == opponent => { 
                    if self.gameboard.has_only_mills(player) {
                            not_valid = false; 
                            taker_board.del_stone_at(index);
                            taker_board.decrement_stone_counter(player);
                    } else {
                        if self.gameboard.mill_checker(index) {
                            println!("Can't take stone, because it is part of a mill! Please try again.");
                            continue
                        } else {
                            println!("correct mill case");
                            not_valid = false; 
                            taker_board.del_stone_at(index);
                            taker_board.decrement_stone_counter(player);
                        }
                    }
                }
                _ => {}  
            } 
            }
        }
    } 

    fn calc_next_gameboard(&mut self, current_move: MillMove ) -> GameBoard {
        let mut new_board : GameBoard = self.gameboard.clone();
        if current_move.is_valid(&new_board) {
            match current_move.movetype {
                Phase::Place => {
                    new_board.set_stone_at(current_move.destination, self.turn );
                    new_board.increment_stone_counter(self.turn); 
                    //mill closed?
                    if new_board.mill_checker(current_move.destination) {
                        self.take_opponent_stone(&mut new_board);
                    }
                    //all stones have been placed 
                    if new_board.total_placed_black_stones == 9 && new_board.total_placed_white_stones == 9 {
                        new_board.set_gamephase(Phase::Move);
                    }
                }
                Phase::Move => {
                    new_board.set_stone_at(current_move.destination, self.turn);
                    new_board.del_stone_at(current_move.origin);
                    if new_board.mill_checker(current_move.destination) {
                        self.take_opponent_stone(&mut new_board);
                    }
                }
            }
        } else {
            println!("Couldn't apply changes! Move not valid!");
        }
        new_board
    }

    fn retrieve_new_millmove(&mut self)-> MillMove{
        let turn: Player = get_other_player(self.turn.clone());
        let current_phase : Phase = self.gameboard.gamephase.clone();
        match current_phase {
            Phase::Place => {
                println!("Please enter where you want to put your stone.");
                let current_destination: u8 = get_user_input_as_number();
                let current_origin: u8 = 1;
                if current_destination == 0 {
                    self.game_over = true; 
                    let temp_winner: Player = turn.clone();
                    self.winner = Some(temp_winner);
                }
                return MillMove::new(get_other_player(turn), &self.gameboard, current_origin, current_destination);
            }
            Phase::Move => {
                println!("Please enter which stone you want to move.");
                let current_destination: u8 = 0;
                let current_origin: u8 = get_user_input_as_number();
                if current_origin == 0 {
                    self.game_over = true; 
                    let temp_winner: Player = turn.clone();
                    self.winner = Some(temp_winner);
                    return MillMove::new(get_other_player(turn), &self.gameboard, current_origin, current_destination);
                }
                println!("Please enter where you want to move your stone on field {}", current_origin);
                let current_destination: u8 = get_user_input_as_number();
                if current_destination == 0 || current_origin == 0 {
                    self.game_over = true; 
                    let temp_winner: Player = turn.clone();
                    self.winner = Some(temp_winner);
                }
                return MillMove::new(get_other_player(turn), &self.gameboard, current_origin, current_destination);
            }
        }

    }

    fn update_game(&mut self, millmove: MillMove) {
        self.past_moves.push(self.gameboard.board.clone());
        self.gameboard=self.calc_next_gameboard(millmove);
        self.turn = get_other_player(self.turn); 
    }

    fn is_game_over(&mut self)->bool {
        let temp_board = self.gameboard.clone();
        //first condition: any player with less than 3 stones after place-phase?
        if (temp_board.total_placed_black_stones + temp_board.total_placed_white_stones) == 18{
            if temp_board.get_blackstones() < 3 {
                self.winner = Some(Player::White);
                return true 
            } else if temp_board.get_whitestones() < 3 {
                self.winner = Some(Player::Black);
                return true
            } 
        }
         //second condition: any player unable to move? 
         match temp_board.gamephase {
            Phase::Place => {}
            Phase::Move => {
                if !temp_board.has_moves_left(Player::White) {
                    self.winner = Some(Player::Black);
                    println!("Player White has no legal moves left!");   
                     return true  
                    } else if !temp_board.has_moves_left(Player::Black){
                      self.winner = Some(Player::White);
                      println!("Player Black has no legal moves left!");
                      return true 
                    }
            }
         }

        //third condition: was the current board repeated 3 times?
        let mut counter: u64 = 0; 
        let board_string: String = temp_board.board.clone();  
        for element in self.past_moves.iter() {
            if *element == board_string {
                counter+=1;
            }
            if counter >= 2 {
                break;
            }
        } 
        if counter >= 2 {
            println!("This position was repeated 3 times! ");
            return true 
        } else {
            return false 
        }

    }

    fn print_winner(&self)-> String {
        match self.winner {
            None => {return String::from("Its a draw! Well played on both sides.")},
            Some(Player::Black) => {return String::from("Player Black has won the game! GGWP.")},
            Some(Player::White) => {return String::from("Player White has won the game! GGWP.")},
        }
    }

    


    pub fn run(&mut self){
        println!("Welcome to this mill game. The fields are indexed the following way:");
        println!("Starting from the upper, middle field on the innermorst ring with 1, the indexes increment clockwise and outwarts.");
        println!("Note that after one ring is finished, the next ring starts again at the upper middle field. Here's a visual:");
        print_tutorial_board();
        println!("type in anything and press 'enter' to start"); 
        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).expect("Error parsing user input");
        self.gameboard.print_gameboard();
        while !self.game_over {
            println!("Current gamephase: {}, Current turn: {}", decode_phase(self.gameboard.gamephase), decode_player(Some(self.turn)));
           //update and validate upcoming move
           let mut new_move:MillMove = self.retrieve_new_millmove();
           if self.game_over {
            break; 
            }
           while !new_move.is_valid(&self.gameboard){
            new_move = self.retrieve_new_millmove();
            if self.game_over {
                break; 
            }
           }
           if self.game_over {
            break; 
           }
           //process new move
           self.update_game(new_move); 
           if self.game_over {
            self.gameboard.print_gameboard();
            break; 
            }
           self.gameboard.print_gameboard();
           println!("white stones: {}, black stones: {}",self.gameboard.get_whitestones(), self.gameboard.get_blackstones());
           self.game_over = self.is_game_over();
           self.turn_counter +=1;
        }
        println!("{} The game went on for {} rounds!", self.print_winner(), self.turn_counter);
    }
}

pub fn get_user_input_as_number() -> u8 {
    loop {
        println!("Please enter a valid field number (between 1 and 24) to continue or type either 'exit' or '0' to give up and end the game.");
        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).expect("Error parsing user input");
        let cleaned_input = user_input.trim();
        if cleaned_input.eq_ignore_ascii_case("exit") {
            return 0
        }
        match cleaned_input.parse::<u8>() {
            Ok(number) => {
                return number
            }
            Err(_) => {
                println!("Invalid Input. Please try again.");
                continue;
            }
        }
    }
} 

 fn print_tutorial_board(){
    let a = 24;
    let b = 17;
    let c =  18;
    println!("{}-----------{}-----------{}", a, b, c);
    println!("|            |            |");

    let a = 16;
    let b =  9;
    let c =  10;
    println!("|   {}-------{}-------{}   |", a, b, c);
    println!("|   |        |        |   |");

    let a = 8;
    let b = 1;
    let c = 2;
    println!("|   |   {}----{}----{}   |   |", a, b, c);
    println!("|   |   |         |   |   |");

    let a = 23;
    let b = 15;
    let c = 7;
    let d = 3;
    let e = 11;
    let f = 19;
    println!("{}--{}--{}         {}---{}--{}", a, b, c, d, e, f);
    println!("|   |   |         |   |   |");

    let a = 6;
    let b = 5;
    let c = 4;
    println!("|   |   {}----{}----{}   |   |", a, b, c);

    println!("|   |        |        |   |");
    let a = 14;
    let b =  13;
    let c = 12;
    println!("|   {}-------{}-------{}  |", a, b, c);

    println!("|            |            |");
    let a = 22;
    let b = 21;
    let c = 20;
    println!("{}-----------{}-----------{}", a, b, c);
}

