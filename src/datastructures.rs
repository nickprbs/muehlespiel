use std::str::FromStr;
use std::fmt;


#[derive(Clone,Copy,PartialEq)]
pub enum Player {
    Black,
    White,
}

#[derive(Clone,Copy,PartialEq)]
pub enum Phase {
    Place,
    Move, 
}

#[derive(Debug, Clone)]
pub struct InvalidFormatError;

impl fmt::Display for InvalidFormatError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid format of gameboard string")
    }
}

#[derive(Clone)]
pub struct GameBoard {
    pub   board: String,
    pub   gamephase: Phase,
    pub      white_stones: u8, 
    pub      black_stones: u8, 
    pub   total_placed_white_stones: u8,
    pub   total_placed_black_stones: u8,
} 
impl GameBoard {
    //constructs new Gameboard
    pub fn new() -> GameBoard {
        let game_board : GameBoard = GameBoard {
            board : String::from("EEEEEEEEEEEEEEEEEEEEEEEE"),
            gamephase : Phase::Place,
            white_stones : 0,
            black_stones : 0,
            total_placed_white_stones: 0,
            total_placed_black_stones: 0,
        }; 
        game_board
    }

    pub fn get_whitestones(&self)-> u8{
    self.white_stones.clone()
   }
    pub fn get_blackstones(&self) -> u8{
    self.black_stones.clone()
   }
    //returns the color at the specified index
    pub fn get_player_at(&self, index:u8) -> Option<Player> { 
        if index < 1 || index > 24 {
            panic!("Index out of bounds! Must be between 1 and 24!");
        }
        let temp_board: &String = &self.board; 
        let x: char = temp_board.chars().nth((index-1) as usize).unwrap(); 
        if x == 'B' {
            return Some(Player::Black)
        } else if x =='W' {
            return Some(Player::White)
        } else if x =='E' {
            return None 
        } else {
            panic!("Illegal Gameboard state: unexpected field token!");
        }
   }
    // true if specified field is empty
    pub fn is_free_at(&self, index: u8)-> bool { 
        matches!(self.get_player_at(index), None)
   }
   
   // returns a Vector<u8> of all free fields
   pub fn get_free_fields(&self)-> Vec<u8> {
        let mut output_vec : Vec<u8> = Vec::new();
        for i in 1..=24 {
            if self.is_free_at(i) {
                output_vec.push(i);
            }
        }
        output_vec
   }

    // changes the current stone type on the selected field. Only usable if field is empty!  
    pub fn set_stone_at(&mut self, index: u8, color: Player){
        if self.is_free_at(index) {
            self.board.replace_range((index-1)as usize ..index as usize , (decode_player(Some(color))).to_string().as_str());
        } else {
            panic!("Invalid action: there's already a stone at field {}", index);
        }
    }
    // returns a Vector<u8> of all neighbours 
    fn get_neighbours(&self, index: u8)-> Vec<u8> {
        let mut neighbours: Vec<u8> = Vec::new();
        if index % 2 == 0 {
            if index % 8 == 0 {
                neighbours.push(index-1);
                neighbours.push(index-7);
            } else {
                neighbours.push(index-1);
                neighbours.push(index+1);
            }
        } else {
            let ring = determine_ring(index); 
            if index % 8 == 1 {
                neighbours.push(index+7);
                neighbours.push(index+1);
            } else {
                neighbours.push(index-1);
                neighbours.push(index+1);
            }
            if ring == 1 {
                neighbours.push(index+8);
            } else if ring ==2 {
                neighbours.push(index-8);
                neighbours.push(index+8);
            } else if ring ==3 {
                neighbours.push(index-8);
            }
        }    
        neighbours 
    }
    pub fn set_gamephase(&mut self, phase:Phase){
         self.gamephase = match phase {
            Phase::Place => {Phase::Place},
            Phase::Move => {Phase::Move},
        };
    }
    pub fn del_stone_at(&mut self, index: u8){
        if !self.is_free_at(index){
            self.board.replace_range((index-1) as usize ..index as usize, (decode_player(None)).to_string().as_str());
        }
    }
    pub fn  increment_stone_counter (&mut self, player: Player){
        let current_phase = self.gamephase; 
        match current_phase {
            Phase::Place => {
                match player {
                    Player::Black => {
                    self.black_stones +=1;
                    self.total_placed_black_stones +=1;
                    }
                    Player::White => {
                        self.white_stones +=1;
                        self.total_placed_white_stones +=1;
                    }
                }
                if self.total_placed_black_stones > 9 || self.total_placed_white_stones > 9 {
                    panic!("Illegal State: Stone count can't be more than 9!");
                }
            }
            Phase::Move => {
                panic!("Illegal State: Can't place stone in move phase!");
                }
            }
        }
    pub fn  decrement_stone_counter (&mut self, player: Player){
        if (self.total_placed_black_stones==9 && self.black_stones < 3 )||(self.total_placed_white_stones==9 && self.white_stones < 3) {
            panic!("Illegal State: Game should be over by now!"); 
        }
        match player {
            Player::Black => {self.black_stones -=1;}
            Player::White => {self.white_stones -=1;} 
        }
    }
           
    //checks two specific fields for a certain color, returns true if they all have the same. 
    fn is_color_matching (&self, color: Player, tuple: (u8, u8)) -> bool {
        match self.get_player_at(tuple.0) {
            None => {return false}
            _ => {}
        }
        match self.get_player_at(tuple.1) {
            None => {return false}
            _ => {}
        }
        let color_0 : Player = self.get_player_at(tuple.0).unwrap();
        let color_1: Player = self.get_player_at(tuple.1).unwrap();
        match color_0 {
           c if c == color => {}
            _ => {return false}
        }
        match color_1 {
           c if c == color => {return true}
            _ => {return false} 
        }
    }
    //returns true if the index is part of a mill 
    pub fn mill_checker (&self, index: u8)->bool {
        let color: Option<Player> = self.get_player_at(index);
        let result :bool = false;
        if color.is_none() {return false}
        let color: Player = color.unwrap();
        //case 1: middle field
        if index % 2 == 1 {
        let ring: u8 = determine_ring(index);
            
            if ring == 1 && self.is_color_matching(color, ((index+8), (index+16))) {
                return true
            } else if ring == 2 && self.is_color_matching(color, ((index-8),(index+8))) {
                return true
            } else if ring == 3 && self.is_color_matching(color, ((index-8),(index-16))) {
                return true
            }
            if index % 8 ==1 {
             if  self.is_color_matching(color, ((index+1),(index+7))) {
                return true }
            } else if self.is_color_matching(color, ((index+1),(index-1))) {
                return true 
            } else {return false}
        //case 2: edge field     
        } else {
            if index % 8 == 0 {
                if self.is_color_matching(color, ((index-1),(index-2))) || self.is_color_matching(color, ((index-6),(index-7))) {
                    return true
                } else {return false }
            }  else if index % 8 == 2 {
                if self.is_color_matching(color, ((index-1),(index+6)))|| self.is_color_matching(color, ((index+1),(index+2))) {
                    return true 
                } else {return false }
            } else if self.is_color_matching(color, ((index-1),(index-2))) || self.is_color_matching(color, ((index+1),(index+2))) {
                return true 
            } else {return false }
             
        }
        result 
    }
    pub fn get_all_stones_of (&self, color: Player)-> Vec<u8> {
        let mut output: Vec<u8> = Vec::new();
        let board_string = &self.board;
        let search = decode_player(Some(color));
        for (index, character) in board_string.chars().enumerate() {
            if character == search {
                output.push((index+1) as u8); 
            }
        }
        output 
    }
    pub fn has_only_mills (&self, color: Player)-> bool {
        let result :bool = true; 
        let instances : Vec<u8> = self.get_all_stones_of(color);
        for stone in instances {
            if !self.mill_checker(stone){
                return false 
            }
        }
        result 
    }
     
     //enumerates all possible moves of one fixed position inside a Vector<MillMove>
     fn enurmerate_moves(&self, position: u8, result: &mut Vec<MillMove>){
        let mut _player= self.get_player_at(position);
        let neighbour_vec = self.get_neighbours(position);
        let free_field_vec = self.get_free_fields();
        let free_field_vec_clone = free_field_vec.clone();
        match _player {
            None=> {result.clear();}
            Some(millplayer) => {
                result.clear();
                let _player = millplayer; 
                let stone_amount = match _player {
                    Player::Black => {self.get_blackstones()}
                    Player::White => {self.get_whitestones()}
                }; 
                if stone_amount == 3 {
                    for free_field in free_field_vec {
                        let possible_move: MillMove = MillMove::new(_player, &self, position, free_field);
                        if possible_move.is_valid(self){
                            result.push(possible_move);
                        }
                    }
                } else {
                    for neighbour in neighbour_vec {
                        if self.is_free_at(neighbour) {
                            let possible_move: MillMove = MillMove::new(_player, &self, position, neighbour);
                            if possible_move.is_valid(self){
                                result.push(possible_move);
                            }
                        }
                    }
                }
            }
        }
        
    }

    //returns a Vec<MillMove> of all possible moves of the player. 
    fn possile_moves_vector(&self, player:Player) -> Vec<MillMove> {
        let mut output: Vec<MillMove> = Vec::new();
        let player_instances: Vec<u8> = self.get_all_stones_of(player);
        let mut temp_moves_vec: Vec<MillMove> = Vec::new();
            for instance in player_instances {
                self.enurmerate_moves(instance, &mut temp_moves_vec);
                output.extend(temp_moves_vec.drain(..));
                temp_moves_vec.clear();
            }
        output 
    }

    pub fn possible_moves_amount(&self, player: Player)-> u8{
        self.possile_moves_vector(player).len() as u8
    }

    pub fn possible_mill_amount(&self, player: Player) -> u8 {
        let all_moves = self.possile_moves_vector(player);
        let mut amount: u8 = 0;
        for one_move in all_moves {
            let temp_board = self.move_simulator(one_move);
            if temp_board.mill_checker(one_move.destination){
                amount +=1;
            }
        }
        amount 
    }
    //returns amount of possible opponent stones, that could be taken when closing a mill
    pub fn takeable_opponent_amount(&self, player: Player)-> u8 {
        let mut amount: u8 = 0;
        if self.possible_mill_amount(player) > 0 {
        let opponent = get_other_player(player);
        if self.has_only_mills(opponent){
            let stone_amount = match opponent {
                Player::Black => {self.get_blackstones()}
                Player::White => {self.get_whitestones()}
            }; 
            amount = stone_amount;
        } else {
            let opponent_instances = self.get_all_stones_of(opponent);
            for opponent_instance in opponent_instances {
                if !self.mill_checker(opponent_instance){
                    amount +=1;
                }
            }
        }
        }
        amount 
    }

    pub fn has_moves_left(&self, player:Player)->bool {
        if self.possible_moves_amount(player) > 0 {
            return true
        } else {
            return false 
        }
    }
        
     //simulates a single move on current gameboard an outputs a new, updated board
    pub fn move_simulator(&self, millmove: MillMove)->GameBoard{
         let mut temp_board = self.clone(); 
         if millmove.is_valid(&temp_board) {
            match millmove.movetype {
                Phase::Place => {
                    temp_board.set_stone_at(millmove.destination, millmove.turn );
                    temp_board.increment_stone_counter(millmove.turn); 
                    //mill closed?
                    if temp_board.mill_checker(millmove.destination) {
                        
                    }
                    //all stones have been placed 
                    if temp_board.total_placed_black_stones == 9 && temp_board.total_placed_white_stones == 9 {
                        temp_board.set_gamephase(Phase::Move)
                    }
                }
                Phase::Move => {
                    temp_board.set_stone_at(millmove.destination, millmove.turn);
                    temp_board.del_stone_at(millmove.origin);
                }
            }
        } else {
            println!("Couldn't apply changes! Move not valid!");
        }
        temp_board
    }   
   


    pub fn print_gameboard(&self){
        let a = decode_player(self.get_player_at(24));
        let b = decode_player(self.get_player_at(17));
        let c = decode_player(self.get_player_at(18));
        println!("{}------------{}------------{}", a, b, c);
        println!("|            |            |");

        let a = decode_player(self.get_player_at(16));
        let b = decode_player(self.get_player_at(9));
        let c = decode_player(self.get_player_at(10));
        println!("|   {}--------{}--------{}   |", a, b, c);
        println!("|   |        |        |   |");

        let a = decode_player(self.get_player_at(8));
        let b = decode_player(self.get_player_at(1));
        let c = decode_player(self.get_player_at(2));
        println!("|   |   {}----{}----{}   |   |", a, b, c);
        println!("|   |   |         |   |   |");

        let a = decode_player(self.get_player_at(23));
        let b = decode_player(self.get_player_at(15));
        let c = decode_player(self.get_player_at(7));
        let d = decode_player(self.get_player_at(3));
        let e = decode_player(self.get_player_at(11));
        let f = decode_player(self.get_player_at(19));
        println!("{}---{}---{}         {}---{}---{}", a, b, c, d, e, f);
        println!("|   |   |         |   |   |");

        let a = decode_player(self.get_player_at(6));
        let b = decode_player(self.get_player_at(5));
        let c = decode_player(self.get_player_at(4));
        println!("|   |   {}----{}----{}   |   |", a, b, c);

        println!("|   |        |        |   |");
        let a = decode_player(self.get_player_at(14));
        let b = decode_player(self.get_player_at(13));
        let c = decode_player(self.get_player_at(12));
        println!("|   {}--------{}--------{}   |", a, b, c);

        println!("|            |            |");
        let a = decode_player(self.get_player_at(22));
        let b = decode_player(self.get_player_at(21));
        let c = decode_player(self.get_player_at(20));
        println!("{}------------{}------------{}", a, b, c);
    }
}

impl FromStr for GameBoard {
    type Err = InvalidFormatError;  

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() == 24 {
        let mut white: u8 = 0;
        let mut black: u8 = 0;
        let boardstring = String::from(format!("{}{}{}", &s[16..24], &s[8..16], &s[0..8]));
        let gameboard: GameBoard; 
        for character in &mut s.chars() {
            if character == 'B' {
                black +=1;
            } else if character == 'W' {
                white +=1;
            } else if character == 'E' {
            } else {
               return Err(InvalidFormatError)
            }
        }
         gameboard = GameBoard{
            board: boardstring,
            gamephase: Phase::Move,
            white_stones: white,
            black_stones: black,
            total_placed_black_stones: 9,
            total_placed_white_stones: 9,
        };
            return Ok(gameboard)
        } else {
           return Err(InvalidFormatError)
        }
        
    }
}

impl ToString for GameBoard {
    fn to_string(&self) -> String {
        String::from(format!("{}{}{}", &self.board[16..24], &self.board[8..16], &self.board[0..8] ))
    }
}

#[derive(Clone,Copy)]
pub struct MillMove {
   pub movetype : Phase,
   pub turn: Player,
   pub origin: u8,
   pub destination: u8,
}
impl MillMove {
    pub fn new(current_turn: Player, game_board: &GameBoard, origin: u8, destination: u8)-> MillMove{
        let millmove: MillMove = MillMove{
            movetype : game_board.gamephase.clone(),
            turn: current_turn,
            origin: origin,
            destination: destination,
        }; 
        millmove 
    }
    pub fn is_valid (&self, gameboard: &GameBoard)->bool {
        if self.destination < 1 || self.destination > 24 || self.origin < 1 || self.origin > 24 {
            println!("OutOfBoundsError!: Origin or destination is not a field on the board!"); 
            return false    
        }
        if gameboard.total_placed_black_stones >9 {
            panic!("Invalid State: too many black stones have been placed! This should not be possible!");
        }else if gameboard.total_placed_white_stones > 9 {
            panic!("Invalid State: too many white stines have been placed! This should not be possible!");
        }
        let _valid: bool = false;
        let turn = self.turn;
        let stone_amount = match turn{
            Player::Black => {gameboard.get_blackstones()}
            Player::White => {gameboard.get_whitestones()}
        };
        match &self.movetype {
            Phase::Place => {
                if gameboard.is_free_at(self.destination){
                    if gameboard.total_placed_black_stones + gameboard.total_placed_white_stones < 18 {
                        if stone_amount < 10 {
                            return true 
                        } else { println!("InvalidStateError: Already placed all possible stones for this player!");
                                    return false 
                                }
                    } else {
                        println!("InvalidStateError: Already placed all possible stones! Phase should be Move by now!");
                        return false 
                    }
                } else {
                    println!("InvalidActionError: Can't place stone on occupied field!");
                    return false 
                }
            }
            Phase::Move => {
                //checks if player wants to move his own color
                let _opponent: Player = get_other_player(self.turn);
                match gameboard.get_player_at(self.origin){
                    None => {
                        println!("InvalidActionError: There's no stone at the selected field!");
                        return false}
                    Some(player) if player == _opponent => {
                        println!("Invalid Action! Can't move stone of the opponent!");
                        return false }    
                    _ => {}
                }
                let _origin_color = gameboard.get_player_at(self.origin).unwrap();
                    if stone_amount == 3 {
                        if gameboard.is_free_at(self.destination){
                            return true 
                        } else {
                            println!("Invalid Action! Can't jump to occupied field!");
                            return false 
                        }
                    } else {
                        let neighbours: Vec<u8> = gameboard.get_neighbours(self.origin);
                        if neighbours.contains(&self.destination) && gameboard.is_free_at(self.destination){
                            return true 
                        } else {
                            println!("InvalidActionError: Either the destination can't be reached in 1 move or the destination is occupied!");
                            return false 
                        }
                    }
            }
        }
    }

}

// produces a char based on the color of the player
pub fn decode_player (color: Option<Player>)-> char{
    let output = match color {
        Some(Player::Black) => { 'B' },
        Some(Player::White) => {'W'},
        None => {'E'}
    };
    output 
}

pub fn decode_phase (phase: Phase)->String {
    let output: String = match phase {
        Phase::Place => {String::from("Place")},
        Phase::Move => {String::from("Move")},
    };
    output
}

//determines the ring which the input index lays upon
 fn determine_ring (index:u8)-> u8 {
    let mut clone: i8 = index.clone() as i8;
    let mut output : u8 =0;
    while clone > 0 {
        for _i in 0..8 {
            clone -=1; 
        }
        output+=1; 
    }
    output
}

 pub fn get_other_player (color: Player)->Player {
    let output: Player = match color {
        Player::Black => {Player::White },
        Player::White => {Player::Black },
    };
    output
    
 }




