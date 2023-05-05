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

#[cfg(test)]
pub mod dummy {
    use std::{
        collections::{HashMap, HashSet},
        fs::File,
        io::Write,
        str::FromStr,
    };

    use cosmwasm_schema::cw_serde;
    use cosmwasm_std::{Addr, Coin, Decimal, Uint128};

    use crate::{
        msg::dummy::DUMMY_ADDR,
        query::{
            AllMarketsResponse, BasePriceResponse, MetricsResponse,
            ModuleAccountsResponse, ModuleParamsResponse, PositionResponse,
            PositionsResponse, PremiumFractionResponse, ReservesResponse,
        },
        state::{
            MarketConfig, Metrics, ModuleAccountWithBalance, ModuleParams,
            Position,
        },
    };

    use super::*;

    pub fn dec_420() -> Decimal {
        Decimal::from_str("420").unwrap()
    }
    pub fn dec_69() -> Decimal {
        Decimal::from_str("69").unwrap()
    }

    pub fn all_markets_response() -> AllMarketsResponse {
        let mut market_map = HashMap::new();
        market_map.insert(
            String::from("ETH:USD"),
            Market {
                pair: String::from("ETH:USD"),
                base_reserve: dec_69(),
                quote_reserve: dec_69(),
                sqrt_depth: dec_69(),
                depth: Uint256::from(69u64 * 69u64),
                bias: dec_420(),
                peg_mult: dec_420(),
                config: MarketConfig {
                    trade_limit_ratio: dec_420(),
                    fluct_limit_ratio: dec_420(),
                    max_oracle_spread_ratio: dec_420(),
                    maintenance_margin_ratio: dec_420(),
                    max_leverage: dec_420(),
                },
                mark_price: dec_420(),
                index_price: String::from("123"),
                twap_mark: String::from("456"),
                block_number: Uint64::from(42u64),
            },
        );
        AllMarketsResponse { market_map }
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

    pub fn position(pair: String) -> Position {
        Position {
            trader_addr: Addr::unchecked(DUMMY_ADDR),
            pair,
            size: "420".into(),
            margin: dec_420(),
            open_notional: dec_420(),
            latest_cpf: Decimal::zero(),
            block_number: 1u64.into(),
        }
    }

    pub fn position_response() -> PositionResponse {
        PositionResponse {
            position: position("ETH:USD".to_string()),
            notional: "420".into(), // signed dec
            upnl: "69".into(),      // signed dec
            margin_ratio_mark: Decimal::zero(),
            margin_ratio_index: Decimal::zero(),
            block_number: 0u64.into(),
        }
    }

    pub fn positions_response() -> PositionsResponse {
        let mut positions_map: HashMap<String, Position> = HashMap::new();
        let pairs: Vec<String> = vec!["ETH:USD", "BTC:USD"]
            .iter()
            .map(|&s| s.to_string())
            .collect();
        for pair in &pairs {
            positions_map.insert(pair.clone(), position(pair.clone()));
        }

        PositionsResponse {
            positions: positions_map,
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
                twap_lookback_window: Uint64::from(60u64 * 60u64), // 1 hour
                whitelisted_liquidators: HashSet::from_iter(
                    vec![
                        "nibi1ah8gqrtjllhc5ld4rxgl4uglvwl93ag0sh6e6v",
                        "nibi1zaavvzxez0elundtn32qnk9lkm8kmcsz44g7xl",
                    ]
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
                addr: Addr::unchecked(String::from(
                    "nibi1x5zknk8va44th5vjpg0fagf0lxx0rvurpmp8gs",
                )),
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

    #[cw_serde]
    pub struct ExampleNibiruQueryResponseJson {
        all_markets: AllMarketsResponse,
        reserves: ReservesResponse,
        base_price: BasePriceResponse,
        position: PositionResponse,
        positions: PositionsResponse,
        module_params: ModuleParamsResponse,
        premium_fraction: PremiumFractionResponse,
        metrics: MetricsResponse,
        module_accounts: ModuleAccountsResponse,
    }

    #[test]
    fn save_example_json() {
        let example_queries = ExampleNibiruQueryResponseJson {
            all_markets: all_markets_response(),
            reserves: reserves_response(),
            base_price: base_price_response(),
            position: position_response(),
            positions: positions_response(),
            module_params: module_params_response(),
            premium_fraction: premium_fraction_response(),
            metrics: metrics_response(),
            module_accounts: module_accounts_response(),
        };
        let json_str = serde_json::to_string_pretty(&example_queries).unwrap();
        let mut file = File::create("./query_resp.json").unwrap();
        assert!(file.write_all(json_str.as_bytes()).is_ok());
    }
}
