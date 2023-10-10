use std::collections::HashMap;

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{CustomQuery, Decimal, Uint256, Uint64};

use crate::state::{
    Market, Metrics, ModuleAccountWithBalance, ModuleParams, Position,
};

#[cw_serde]
pub enum QueryPerpMsg {
    // -----------------------------------------------------------------
    // From x/perp/amm
    // -----------------------------------------------------------------
    AllMarkets {},

    Reserves {
        pair: String,
    },

    BasePrice {
        pair: String,
        is_long: bool,
        base_amount: Uint256,
    },

    // -----------------------------------------------------------------
    // From x/perp
    // -----------------------------------------------------------------
    Position {
        trader: String,
        pair: String,
    },

    Positions {
        trader: String,
    },

    ModuleParams {},

    PremiumFraction {
        pair: String,
    },

    Metrics {
        pair: String,
    },

    ModuleAccounts {},

    // -----------------------------------------------------------------
    // From x/oracle
    // -----------------------------------------------------------------
    OraclePrices {},
}

impl CustomQuery for QueryPerpMsg {}

#[cw_serde]
pub struct AllMarketsResponse {
    pub market_map: HashMap<String, Market>,
}

#[cw_serde]
pub struct ReservesResponse {
    pub pair: String,
    pub base_reserve: Decimal,
    pub quote_reserve: Decimal,
}

// #[cw_serde]
pub type OraclePricesResponse = HashMap<String, Decimal>;

#[cw_serde]
pub struct BasePriceResponse {
    pub pair: String,
    pub base_amount: Decimal,
    pub quote_amount: Decimal,
    pub is_long: bool,
}

#[cw_serde]
pub struct PositionResponse {
    pub position: Position,
    pub notional: String, // signed dec
    pub upnl: String,     // signed dec
    pub margin_ratio_mark: Decimal,
    pub margin_ratio_index: Decimal,
    pub block_number: Uint64,
}

#[cw_serde]
pub struct PositionsResponse {
    pub positions: HashMap<String, Position>,
}

#[cw_serde]
pub struct ModuleParamsResponse {
    pub module_params: ModuleParams,
}

#[cw_serde]
pub struct PremiumFractionResponse {
    pub pair: String,
    pub cpf: Decimal,
    pub estimated_next_cpf: Decimal,
}

#[cw_serde]
pub struct MetricsResponse {
    pub metrics: Metrics,
}

#[cw_serde]
pub struct ModuleAccountsResponse {
    pub module_accounts: HashMap<String, ModuleAccountWithBalance>,
}
