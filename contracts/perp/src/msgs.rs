use cosmwasm_std::{Uint128, Uint64, Decimal};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct InstantiateMsg {}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub enum ExecuteMsg {
    OpenPosition { 
        pair: String, 
        side: Uint64,
        quote_asset_amount: Uint128,
        leverage: Decimal,
        base_asset_amount_limit: Uint128,
    },

    ClosePosition { pair: String },
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub enum QueryMsg {
    GetPosition {
        trader_address: String,
        pair: String,
    },

    GetPositions {
        trader_address: String,
    }
}
