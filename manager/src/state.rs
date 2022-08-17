use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cw_storage_plus::Map;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub count: i32,
    pub address: String
}

pub const CONTRACTS: Map<(&str, &str), State> = Map::new("contracts");
