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
    Invite {
        coord: Coord,
        host_symbol: PlayerSymbol,
        opponent: String
    },
    Reject {
        as_host: bool,
        opponent: String
    },
    Accept {
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
        key: Option<QueryKey>,
        status: Option<Status>
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct QueryKey {
    pub host: String,
    pub opponent: String,
}