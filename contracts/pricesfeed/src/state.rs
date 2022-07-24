use cosmwasm_std::{Addr, Coin, Decimal, Empty, Timestamp, Uint128};
use cw_storage_plus::{
    Index, IndexList, IndexedMap, Item, Map, MultiIndex, SnapshotItem, SnapshotMap,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::ops::Mul;

pub const ORACLE_PAIR_WHITELIST: Map<(String, &Addr), Empty> = Map::new("oracle_pair_whitelist");
pub const ACTIVE_PAIRS: Map<String, Empty> = Map::new("active_pairs");
pub const RAW_PRICES: Map<(String, &Addr), PostedPrice> = Map::new("raw_prices");
pub const CURRENT_PRICES: Map<String, Decimal> = Map::new("current_prices");
pub const CURRENT_TWAP: Map<String, CurrentTWAP> = Map::new("current_twap");

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct PostedPrice {
    pub pair_id: String,
    pub oracle: Addr,
    pub price: Decimal,
    pub expiry: Timestamp,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct CurrentTWAP {
    pub pair_id: String,
    pub numerator: Decimal,
    pub denominator: Decimal, // could be int?
    pub price: Decimal,
}
