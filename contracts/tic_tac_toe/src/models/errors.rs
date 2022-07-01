use cosmwasm_std::{Addr, StdError};
use thiserror::Error;

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

    #[error("Game between {host} and {opponent} inexistent or already finished. Try starting another game.")]
    GameAlreadyCompleted { host: Addr, opponent: Addr },

    #[error("Invalid coordinate x={x} y={y}. Coordinates must be between 0 and 2")]
    InvalidCoordinates { x: u8, y: u8 },
}
