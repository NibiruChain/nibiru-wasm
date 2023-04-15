use std::collections::{HashMap};

use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Decimal, Uint256, Uint64, CustomQuery};

use crate::state::{Market, Position, ModuleParams, Metrics, ModuleAccountWithBalance};

// Implement cosmwasm_std::CustomQuery interface
impl CustomQuery for NibiruQuery {} 

#[cw_serde]
#[derive(QueryResponses)]
pub enum NibiruQuery {
    // -----------------------------------------------------------------
    // From x/perp/amm
    // -----------------------------------------------------------------
    #[returns(AllMarketsResponse)]
    AllMarkets {},

    #[returns(ReservesResponse)]
    Reserves { pair: String },

    #[returns(BasePriceResponse)]
    BasePrice {
        pair: String,
        is_long: bool,
        base_amount: Uint256,
    },

    // -----------------------------------------------------------------
    // From x/perp
    // -----------------------------------------------------------------
    #[returns(PositionResponse)]
    Position { trader: String, pair: String },

    #[returns(PositionsResponse)]
    Positions { trader: String },

    #[returns(ModuleParamsResponse)]
    ModuleParams {},

    #[returns(PremiumFractionResponse)]
    PremiumFraction { pair: String },

    #[returns(MetricsResponse)]
    Metrics { pair: String },

    #[returns(ModuleAccountsResponse)]
    ModuleAccounts {},
}

#[cw_serde]
pub struct AllMarketsResponse {
    pub market_map: HashMap<String, Market>
}

#[cw_serde]
pub struct ReservesResponse {
    pub pair: String,
    pub base_reserve: Decimal,
    pub quote_reserve: Decimal,
}

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
    pub notional: Decimal,
    pub upnl: Decimal,
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
    pub module_accounts: HashMap<String, ModuleAccountWithBalance>
}