use cosmwasm_std::{Decimal, Uint128, CosmosMsg, CustomMsg, CustomQuery};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_schema::{QueryResponses};

/// A number of Custom messages that can call into the Nibiru bindings
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum NibiruMsg {
    OpenPosition { 
        pair: String, 
        side: u8,
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


/// A number of Custom queries that can call into the Nibiru bindings
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, JsonSchema)]
#[derive(QueryResponses)]
#[serde(rename_all = "snake_case")]
pub enum NibiruQuery {
    #[returns(PositionResponse)]
    Position { 
        trader: String,
        pair: String,
    },

    #[returns(PositionsResponse)]
    Positions { trader: String },
}

impl CustomQuery for NibiruQuery {}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, JsonSchema)]
pub struct Position {
    pub trader: String,
    pub pair: String,
    pub size: String,
    pub margin: String,
    pub open_notional: String,
    pub latest_cumulative_premium_fraction: String,
    pub block_number: Uint128,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, JsonSchema)]
pub struct PositionResponse {
    pub position: Position,
    pub position_notinoal: String,
    pub unrealized_pnl: String,
    pub margin_ratio_mark: String,
    pub margin_ratio_index: String,
    pub block_number: Uint128,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, JsonSchema)]
pub struct PositionsResponse {
    pub positions: Vec<PositionResponse>
}