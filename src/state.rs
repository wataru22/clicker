use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;
use cw_storage_plus::Item;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub lucky_number: i32,
    pub owner: Addr,
    // Here's the vector!
    pub scores: Vec<(Addr, u16)>,
}

pub const STORAGE: Item<State> = Item::new("state");
