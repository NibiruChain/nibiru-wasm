use crate::state::PostedPrice;
use cosmwasm_std::{Addr, Decimal, Timestamp};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct InstantiateMsg {}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub enum ExecuteMsg {
    PostPrice {
        token0: String,
        token1: String,
        price: Decimal,
        expiry: Timestamp,
    },
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub enum SudoMsg {
    BeginBlock {},
    AddOracleProposal {
        oracles: Vec<Addr>,
        pairs: Vec<String>,
    },
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub enum QueryMsg {
    Price { pair_id: String },
    Prices {},
    RawPrices { pair_id: String },
    Oracles {},
    Markets {},
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct QueryPriceResponse {
    pub current_price: CurrentPrice,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct QueryPricesResponse {
    pub current_prices: Vec<CurrentPrice>,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct QueryRawPriceResponse {
    pub raw_prices: Vec<PostedPrice>,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct QueryOraclesResponse {
    pub oracles: Vec<Addr>,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct QueryMarketsResponse {
    pub markets: Vec<Market>,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct Market {
    pub pair_id: String,
    pub oracles: Vec<Addr>,
    pub active: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct CurrentPrice {
    pub pair_id: String,
    pub price: Decimal,
}
