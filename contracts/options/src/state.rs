use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{CanonicalAddr};
use cw_storage_plus::{Item, Map};
use terraswap::asset::Asset;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub owner_addr: CanonicalAddr,
    pub token_addr: CanonicalAddr
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Option {
    pub issuer_addr: CanonicalAddr,
    pub offer: Asset,
    pub ask: Asset,
    pub expire: u64 // expire in block number
}

pub const CONFIG: Item<Config> = Item::new("config");
pub const OPTIONS: Map<&[u8], Option> = Map::new("option");
