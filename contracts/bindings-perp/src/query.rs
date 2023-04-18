use std::collections::HashMap;

use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{CustomQuery, Decimal, Uint256, Uint64};

use crate::state::{
    Market, Metrics, ModuleAccountWithBalance, ModuleParams, Position,
};

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
    pub market_map: HashMap<String, Market>,
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
    pub module_accounts: HashMap<String, ModuleAccountWithBalance>,
}

#[cfg(test)]
pub mod dummy {
    use std::{
        collections::{HashMap, HashSet},
        str::FromStr,
    };

    use cosmwasm_std::{Addr, Coin, Decimal, Uint128};
    use cw_utils::Duration;

    use crate::{
        query::{
            AllMarketsResponse, BasePriceResponse, MetricsResponse,
            ModuleAccountsResponse, ModuleParamsResponse, PositionResponse,
            PositionsResponse, PremiumFractionResponse, ReservesResponse,
        },
        state::{Metrics, ModuleAccountWithBalance, ModuleParams, Position},
    };

    pub fn dec_420() -> Decimal {
        Decimal::from_str("420").unwrap()
    }
    pub fn dec_69() -> Decimal {
        Decimal::from_str("69").unwrap()
    }

    pub fn all_markets_response() -> AllMarketsResponse {
        AllMarketsResponse {
            market_map: HashMap::new(),
        }
    }

    pub fn reserves_response() -> ReservesResponse {
        ReservesResponse {
            pair: "ETH:USD".to_string(),
            base_reserve: dec_420(),
            quote_reserve: dec_69(),
        }
    }

    pub fn base_price_response() -> BasePriceResponse {
        BasePriceResponse {
            pair: "ETH:USD".to_string(),
            base_amount: Decimal::one(),
            quote_amount: dec_420(),
            is_long: false,
        }
    }

    pub fn position_response() -> PositionResponse {
        let addr_str: &str = "nibi1kqg3q3v8pjd3epktg9h0azwk56j5v5r5lu5eq2";
        PositionResponse {
            position: Position {
                trader_addr: Addr::unchecked(String::from(addr_str)),
                pair: "ETH:USD".to_string(),
                size: Decimal::zero(),
                margin: Decimal::zero(),
                open_notional: Decimal::zero(),
                latest_cpf: Decimal::zero(),
                block_number: 0u64.into(),
            },
            notional: Decimal::zero(),
            upnl: Decimal::zero(),
            margin_ratio_mark: Decimal::zero(),
            margin_ratio_index: Decimal::zero(),
            block_number: 0u64.into(),
        }
    }

    pub fn positions_response() -> PositionsResponse {
        PositionsResponse {
            positions: HashMap::new(),
        }
    }

    pub fn module_params_response() -> ModuleParamsResponse {
        ModuleParamsResponse {
            module_params: ModuleParams {
                stopped: false,
                fee_pool_fee_ratio: dec_420(),
                ecosystem_fund_fee_ratio: dec_69(),
                liquidation_fee_ratio: dec_69(),
                partial_liquidation_ratio: Decimal::zero(),
                funding_rate_interval: "1h".to_string(),
                twap_lookback_window: Duration::Time(60 * 60), // 1 hour
                whitelisted_liquidators: HashSet::from_iter(
                    vec!["nibi123", "nibiabc"]
                        .iter()
                        .map(|s_ptr| s_ptr.to_string()),
                ),
            },
        }
    }

    pub fn premium_fraction_response() -> PremiumFractionResponse {
        PremiumFractionResponse {
            pair: "ETH:USD".to_string(),
            cpf: Decimal::zero(),
            estimated_next_cpf: dec_69(),
        }
    }

    pub fn metrics_response() -> MetricsResponse {
        MetricsResponse {
            metrics: Metrics {
                pair: "ETH:USD".to_string(),
                net_size: dec_420(),
                volume_quote: Decimal::one(),
                volume_base: dec_420(),
                block_number: 42u64.into(),
            },
        }
    }

    pub fn module_accounts_response() -> ModuleAccountsResponse {
        let name = "acc1";
        let mut accounts_map = HashMap::new();
        accounts_map.insert(
            name.to_string(),
            ModuleAccountWithBalance {
                name: name.to_string(),
                addr: Addr::unchecked(String::from("nibiacc1")),
                balance: vec![Coin {
                    denom: "foocoin".to_string(),
                    amount: Uint128::new(420),
                }],
            },
        );
        ModuleAccountsResponse {
            module_accounts: accounts_map,
        }
    }
}
