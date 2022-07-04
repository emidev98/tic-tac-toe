pub mod errors;
pub mod responses;
pub mod state;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use self::state::{PlayerSymbol, Status, Coord};


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg { }

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    CreateGame {
        coord: Coord,
        host_symbol: PlayerSymbol,
        opponent: String
    },
    AcceptGame {
        coord: Coord,
        host: String
    },
    Play {
        as_host: bool,
        coord: Coord,
        opponent: String
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    Games {
        host: Option<String>,
        opponent: Option<String>,
        status: Option<Status>
    }
}

