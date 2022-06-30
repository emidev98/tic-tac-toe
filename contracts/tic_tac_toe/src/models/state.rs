use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Match {
    pub game: Vec<Vec<i32>>,
    pub prize: i32,
    pub finished: bool,
}