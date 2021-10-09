use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;
use cw_storage_plus::Item;

// This defines a Database for the Cosmwasm SC which uses a Key-Value Store to save the state of
// the current contract
// Here we create a struct and then we Store it as an Item type to help save the state on the
// contract
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub count: i32,
    pub owner: Addr,
}
pub const STATE: Item<State> = Item::new("state");
