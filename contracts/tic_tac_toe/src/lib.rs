use cosmwasm_std::Addr;
use cw_storage_plus::Map;
use models::state::Game;

pub mod contract;
pub mod models;
pub mod test;

pub const GAMES: Map<(&Addr, &Addr), Game> = Map::new("games");