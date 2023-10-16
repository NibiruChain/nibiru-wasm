use cosmwasm_std::Coin;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct InstantiateMsg {}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub enum ExecuteMsg {
    // For x/tokenfactory
    //
    CreateDenom { subdenom: String },
    Mint { coin: Coin, mint_to: String },
    Burn { coin: Coin, burn_from: String },
    ChangeAdmin { denom: String, new_admin: String },
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub enum QueryMsg {}
