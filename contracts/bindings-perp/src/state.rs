use std::collections::HashSet;

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Coin, Decimal, Uint256, Uint64};
use cw_utils::Duration;

#[cw_serde]
pub struct Position {
    pub trader_addr: Addr,
    pub pair: String,
    pub size: Decimal,
    pub margin: Decimal,
    pub open_notional: Decimal,
    pub latest_cpf: Decimal,
    pub block_number: Uint64,
}

#[cw_serde]
pub struct Market {
    pub pair: String,
    pub base_reserve: Decimal,
    pub quote_reserve: Decimal,
    pub sqrt_depth: Decimal,
    pub depth: Uint256,
    pub bias: Decimal,
    pub peg_mult: Decimal,
    pub config: MarketConfig,
    pub mark_price: Decimal,
    pub index_price: String,
    pub twap_mark: String,
    pub block_number: Uint64,
}

#[cw_serde]
pub struct MarketConfig {
    pub trade_limit_ratio: Decimal,
    pub fluct_limit_ratio: Decimal,
    pub max_oracle_spread_ratio: Decimal,
    pub maintenance_margin_ratio: Decimal,
    pub max_leverage: Decimal,
}

#[cw_serde]
pub struct ModuleParams {
    pub stopped: bool,
    pub fee_pool_fee_ratio: Decimal,
    pub ecosystem_fund_fee_ratio: Decimal,
    pub liquidation_fee_ratio: Decimal,
    pub partial_liquidation_ratio: Decimal,
    pub funding_rate_interval: String,
    pub twap_lookback_window: Duration,
    pub whitelisted_liquidators: HashSet<String>,
}

#[cw_serde]
pub struct Metrics {
    pub pair: String,
    pub net_size: Decimal,
    pub volume_quote: Decimal,
    pub volume_base: Decimal,
    pub block_number: Uint64,
}

#[cw_serde]
#[derive(Eq)]
pub struct ModuleAccountWithBalance {
    pub name: String,
    pub addr: Addr,
    pub balance: Vec<Coin>,
}
