use std::fmt;

use cosmwasm_std::{Coin, Uint128};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Game {
    /**
        Board is defined as following due the
        smart contract optimization process:
        - None: can be selected,
        - Some(Symbol.O): already selected by 'O',
        - Some(Symbol.X): already selected by 'X'

        None|None|None
        ----|----|----
        None|None|None
        ----|----|----
        None|None|None
    */
    pub board: Vec<Vec<Option<PlayerSymbol>>>,

    /**
        Used to determine the symbol of the player that
        started the game an the symbol of the opponent.
    */
    pub host_symbol: PlayerSymbol,

    /*
        Semaphore to determine who's current and next turn.
    */
    pub player_round: Option<PlayerSymbol>,

    /**
       Tracks the amount of coins that will have
       to be transfer to the winner of the game.
    */
    pub prize: Vec<Coin>,

    /**
        Determine the game status, where
        - INVITED: only one game can be in this status at a time per host and opponent pair. This status is achieved by creating a new game and the following possible status are PLAYING or REJECTED.
        - PLAYING: only one game can be in this status at a time per host and opponent pair. To achieve this status must mutate from INVITED.
        - COMPLETED: multiple games can be in this status but they have to mutate from PLAYING.
        - REJECTED: multiple games can be in this status but they have to mutate from INVITE. 
    */
    pub status: Status,

    /**
        Keeps track of the game winner based on the property 
        status from Game struct. When status is COMPLETED,
        winner means the following:
        - None: means that is tie
        - Some(PlayerSymbol.X): player X won,
        - Some(PlayerSymbol.O): player O won
    */
    pub winner: Option<PlayerSymbol>,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, JsonSchema)]
pub enum PlayerSymbol {
    X,
    O
}

impl fmt::Display for PlayerSymbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PlayerSymbol::X => write!(f, "X"),
            PlayerSymbol::O => write!(f, "O"),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, JsonSchema)]
pub enum Status {
    INVITED,
    PLAYING,
    COMPLETED,
    REJECTED,
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Status::INVITED => write!(f, "INVITED"),
            Status::PLAYING => write!(f, "PLAYING"),
            Status::COMPLETED => write!(f, "COMPLETED"),
            Status::REJECTED => write!(f, "REJECTED"),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, JsonSchema)]
pub struct Coord {
    pub x: u8,
    pub y: u8,
}

impl Coord {
    pub fn is_valid(self) -> bool {
        self.x < 3 && self.y < 3
    }
}

impl Game {

    pub fn new(coord: Coord, prize: Vec<Coin>, host_symbol: PlayerSymbol) -> Game {
        let mut symbol_round = PlayerSymbol::O;
        let mut board = vec![vec![None; 3]; 3];
        let row = board.get_mut(coord.y as usize).unwrap();
        row[coord.x as usize] = Some(host_symbol);

        if host_symbol == PlayerSymbol::O {
            symbol_round = PlayerSymbol::X;
        }

        Game {
            board,
            host_symbol,
            player_round: Some(symbol_round),
            prize,
            status: Status::INVITED,
            winner: None,
        }
    }

    pub fn already_played_on(&self, coord: Coord) -> bool {
        return self
            .board
            .get(coord.y as usize)
            .unwrap()
            .get(coord.x as usize)
            .unwrap()
            .is_some();
    }

    pub fn already_played(&mut self, as_host: bool) -> bool {
        match self.player_round {
            Some(current_player_symbol) => {
                if as_host { 
                    return current_player_symbol != self.host_symbol 
                } else {
                    return current_player_symbol == self.host_symbol
                }

            }
            None => return false,
        }
    }

    pub fn double_prize(&mut self) -> &mut Game {
        for coin in &mut self.prize {
            coin.amount = coin.amount.checked_mul(Uint128::new(2)).unwrap();
        }

        self
    }

    pub fn get_half_prize(&self) -> Vec<Coin> {
        self.prize.clone()
            .iter_mut()
            .map(|coin| {
                coin.amount = coin.amount.checked_div(Uint128::new(2)).unwrap();
                coin.clone()
            })
            .collect()
        
    }

    pub fn play(&mut self, coord: Coord) -> &mut Game {
        let row = self.board.get_mut(coord.y as usize).unwrap();
        row[coord.x as usize] = self.player_round;

        self
    }

    pub fn finish_round(&mut self) -> &mut Game {
        match self.player_round {
            Some(PlayerSymbol::X) => self.player_round = Some(PlayerSymbol::O),
            Some(PlayerSymbol::O) => self.player_round = Some(PlayerSymbol::X),
            None => panic!("Invalid player round"),
        };

        self
    }
    

    pub fn is_full_board(&self) -> bool {
        for row in &self.board {
            for cell in row {
                if cell.is_none() {
                    return false;
                }
            }
        }
        true
    }

    pub fn is_current_player_winner(&self) -> bool {
        macro_rules! has {
            ($y:expr, $x:expr) => {
                self.board[$y][$x] == self.player_round
            };
        }
        
        // Horizontally
        for row in 0..3 {
            if has!(row, 0) && has!(row, 1) && has!(row, 2) {
                return true;
            }
        }

        // Vertically
        for col in 0..3 {
            if has!(0, col) && has!(1, col) && has!(2, col) {
                return true;
            }
        }

        // Top-left to bottom-right
        if has!(0, 0) && has!(1, 1) && has!(2, 2) {
            return true;
        }

        // Top-right to bottom-left
        if has!(0, 2) && has!(1, 1) && has!(2, 0) {
            return true;
        }
        
        false
    }
}
