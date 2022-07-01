pub mod errors;
pub mod responses;
pub mod state;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg { }

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    StartGame {
        x: u8,
        y: u8,
        host_symbol: bool,
        opponent: String
    },
    Play {
        x: u8,
        y: u8,
        opponent: String
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    Games {
        host: Option<String>,
        opponent: Option<String>,
        completed: Option<bool>
    }
}

