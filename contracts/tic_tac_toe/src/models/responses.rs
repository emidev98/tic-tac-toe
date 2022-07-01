
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::Game;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct GameResponse {
    pub games: Vec<Game>,
    pub host: Option<String>,
    pub opponent: Option<String>
}