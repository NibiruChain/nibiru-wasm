use std::collections::HashSet;

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Uint64, Decimal, Uint256, Coin};
use cw_utils::Duration;

#[cw_serde]
pub struct Position {
    trader_addr: Addr,
    pair: String,
    size: Decimal,
    margin: Decimal,
    open_notional: Decimal,
    latest_cpf: Decimal,
    block_number: Uint64,
}


#[cw_serde]
pub struct Market {
    pair: String,
    base_reserve: Decimal,
    quote_reserve: Decimal,
    sqrt_depth: Decimal,
    depth: Uint256,
    bias: Decimal,
    peg_mult: Decimal,
    config: MarketConfig,
    mark_price: Decimal,
    index_price: String,
    twap_mark: String,
    block_number: Uint64,
}

#[cw_serde]
pub struct MarketConfig {
    trade_limit_ratio: Decimal,
    fluct_limit_ratio: Decimal,
    max_oracle_spread_ratio: Decimal,
    maintenance_margin_ratio: Decimal,
    max_leverage: Decimal,
}

#[cw_serde]
pub struct ModuleParams {
    stopped: bool,
    fee_pool_fee_ratio: Decimal,
    ecosystem_fund_fee_ratio: Decimal,
    liquidation_fee_ratio: Decimal,
    partial_liquidation_ratio: Decimal,
    funding_rate_interval: String,
    twap_lookback_window: Duration,
    whitelisted_liquidators: HashSet<String>,
}

#[cw_serde]
pub struct Metrics {
    pair: String,
    net_size: Decimal,
    volume_quote: Decimal,
    volume_base: Decimal,
    block_number: Uint64,
}

#[cw_serde]
#[derive(Eq)]
pub struct ModuleAccountWithBalance {
    name: String,
    addr: Addr,
    balance: Vec<Coin>,
}