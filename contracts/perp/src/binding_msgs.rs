use cosmwasm_std::{Uint64, Decimal, Uint128, CosmosMsg, CustomMsg};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// A number of Custom messages that can call into the Nibiru bindings
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, JsonSchema)]
pub enum NibiruMsg {
    OpenPosition { 
        pair: String, 
        side: Uint64,
        quote_asset_amount: Uint128,
        leverage: Decimal,
        base_asset_amount_limit: Uint128,
    },

    ClosePosition { pair: String },
}

impl From<NibiruMsg> for CosmosMsg<NibiruMsg> {
    fn from(msg: NibiruMsg) -> CosmosMsg<NibiruMsg> {
        CosmosMsg::Custom(msg)
    }
}

impl CustomMsg for NibiruMsg {}
