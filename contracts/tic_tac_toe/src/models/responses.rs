
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::Match;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MatchResponse {
    pub matches: Vec<Match>,
    pub host: Option<String>,
    pub opponent: Option<String>
}