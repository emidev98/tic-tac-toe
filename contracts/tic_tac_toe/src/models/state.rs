use cosmwasm_std::Coin;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::errors::ContractError;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Game {
/**
    Board is defined as following due the 
    smart contract optimization process:

    - None: this space can be selected,
    - Some(true): this space has been selected with 'O',
    - Some(false): this space has been selected 'X' 

    None|None|None
    ----|----|----
    None|None|None
    ----|----|----
    None|None|None
*/
    pub board: Vec<Vec<Option<bool>>>,
    pub host_symbol: bool,
    pub prize: Vec<Coin>,
    pub completed: bool,
}

impl Game {
    pub fn new(
        x: u8,
        y: u8,
        prize: Vec<Coin>,
        host_symbol: bool,
    ) -> Result<Game, ContractError> {
        if x > 2 || y > 2 {
            return Err(ContractError::InvalidCoordinates { x, y });
        }
        
        let mut board = vec![vec![None; 3]; 3];
        let x_row = board.get_mut(x as usize).unwrap();
        x_row[y as usize] = Some(host_symbol);

        Ok(Game {
            board,
            host_symbol,
            prize,
            completed: false,
        })
    }
}