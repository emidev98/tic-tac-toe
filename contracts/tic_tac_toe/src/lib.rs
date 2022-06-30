use cosmwasm_std::Addr;
use cw_storage_plus::Map;
use models::state::Match;

pub mod contract;
pub mod models;
pub mod test;

pub const MATCHES: Map<(&Addr, &Addr), Match> = Map::new("matches");