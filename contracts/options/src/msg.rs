use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use terraswap::asset::Asset;
use cosmwasm_std::{Addr};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {

}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Mint {
        ask: Asset,
        offer: Asset,
        expire: u64
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    Config {}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ConfigResponse {
    pub contract_addr: Addr,
    pub owner_addr: Addr,
    pub token_addr: Addr
}