use cosmwasm_std::{Addr, StdError};
use thiserror::Error;

use super::state::Coord;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Game against yourself cannot be started")]
    CannotStartGame {},

    #[error("Game between {host} and {opponent} already exists. Complete the previous game to start a new one")]
    GameAlreadyInProgress { host: Addr, opponent: Addr },

    #[error("Game between {host} and {opponent} is invalid. Try starting another game.")]
    InvalidGame { host: Addr, opponent: Addr },

    #[error("x={{coord.x}} and y={{coord.y}} already contain symbol. Try using another coordinate.")]
    CoordinateAlreadyPlayed { coord: Coord },

    #[error("You already played this turn. Wait for '{{second_player}}' to play its turn.")]
    TurnAlreadyPlayed { second_player: String},

    #[error("Invalid coordinate x={{coord.x}} y={{coord.y}}. Coordinates must be between 0 and 2")]
    InvalidCoord { coord: Coord },

    #[error("The funds you send must be equal to the prize of the game")]
    InvalidReceivedFunds {},
}