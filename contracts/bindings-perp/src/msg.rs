use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{
    Coin, CosmosMsg, CustomMsg, CustomQuery, Decimal, Response, StdResult,
    Uint128, Uint256,
};

use nibiru_macro::cw_custom;
use nibiru_std::bindings::msg::NibiruRoute;
use nibiru_std::bindings::query as bindings_query;

use crate::state::Sudoers;

// ---------------------------------------------------------------------------
// Entry Point - Instantiate
// ---------------------------------------------------------------------------

#[cw_serde]
pub struct InitMsg {
    pub admin: Option<String>,
}

// ---------------------------------------------------------------------------
// Entry Point - Execute
// ---------------------------------------------------------------------------

/// NibiruExecuteMsg is an override of CosmosMsg::Custom. Using this msg
/// wrapper for the ExecuteMsg handlers show that their return values are valid
/// instances of CosmosMsg::Custom in a type-safe manner. It also shows how
/// ExecuteMsg can be extended in the contract.
#[cw_serde]
#[cw_custom]
pub struct NibiruExecuteMsg {
    pub route: NibiruRoute,
    pub msg: ExecuteMsg,
}

#[cw_serde]
pub enum ExecuteMsg {
    MarketOrder {
        pair: String,
        is_long: bool,
        quote_amount: Uint128,
        leverage: Decimal,
        base_amount_limit: Uint128,
    },

    ClosePosition {
        pair: String,
    },

    AddMargin {
        pair: String,
        margin: Coin,
    },

    RemoveMargin {
        pair: String,
        margin: Coin,
    },

    MultiLiquidate {
        pair: String,
        liquidations: Vec<LiquidationArgs>,
    },

    DonateToInsuranceFund {
        donation: Coin,
    },

    Claim {
        funds: Option<Coin>,
        claim_all: Option<bool>,
        to: String,
    },

    NoOp {},
}

#[cw_serde]
pub struct LiquidationArgs {
    pub pair: String,
    pub trader: String,
}

/// nibiru_msg_to_cw_response: Converts a CosmosMsg to the response type
/// expected by the execute entry point of smart contract's .
pub fn nibiru_msg_to_cw_response(
    cw_msg: CosmosMsg<NibiruExecuteMsg>,
) -> StdResult<Response<NibiruExecuteMsg>> {
    Ok(Response::new().add_message(cw_msg))
}

impl NibiruExecuteMsg {
    pub fn market_order(
        pair: String,
        is_long: bool,
        quote_amount: Uint128,
        leverage: Decimal,
        base_amount_limit: Uint128,
    ) -> CosmosMsg<NibiruExecuteMsg> {
        NibiruExecuteMsg {
            route: NibiruRoute::Perp,
            msg: ExecuteMsg::MarketOrder {
                pair,
                is_long,
                quote_amount,
                leverage,
                base_amount_limit,
            },
        }
        .into()
    }

    pub fn close_position(pair: String) -> CosmosMsg<NibiruExecuteMsg> {
        NibiruExecuteMsg {
            route: NibiruRoute::Perp,
            msg: ExecuteMsg::ClosePosition { pair },
        }
        .into()
    }

    pub fn add_margin(
        pair: String,
        margin: Coin,
    ) -> CosmosMsg<NibiruExecuteMsg> {
        NibiruExecuteMsg {
            route: NibiruRoute::Perp,
            msg: ExecuteMsg::AddMargin { pair, margin },
        }
        .into()
    }

    pub fn remove_margin(
        pair: String,
        margin: Coin,
    ) -> CosmosMsg<NibiruExecuteMsg> {
        NibiruExecuteMsg {
            route: NibiruRoute::Perp,
            msg: ExecuteMsg::RemoveMargin { pair, margin },
        }
        .into()
    }

    pub fn multi_liquidate(
        pair: String,
        liquidations: Vec<LiquidationArgs>,
    ) -> CosmosMsg<NibiruExecuteMsg> {
        NibiruExecuteMsg {
            route: NibiruRoute::Perp,
            msg: ExecuteMsg::MultiLiquidate { pair, liquidations },
        }
        .into()
    }

    pub fn donate_to_insurance_fund(
        donation: Coin,
    ) -> CosmosMsg<NibiruExecuteMsg> {
        NibiruExecuteMsg {
            route: NibiruRoute::Perp,
            msg: ExecuteMsg::DonateToInsuranceFund { donation },
        }
        .into()
    }

    pub fn no_op() -> CosmosMsg<NibiruExecuteMsg> {
        NibiruExecuteMsg {
            route: NibiruRoute::NoOp,
            msg: ExecuteMsg::NoOp {},
        }
        .into()
    }
}

// ---------------------------------------------------------------------------
// Entry Point - Query
// ---------------------------------------------------------------------------

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(SudoersQueryResponse)]
    Sudoers {},
    // -----------------------------------------------------------------
    // From x/perp/amm
    // -----------------------------------------------------------------
    #[returns(bindings_query::AllMarketsResponse)]
    AllMarkets {},

    #[returns(bindings_query::ReservesResponse)]
    Reserves { pair: String },

    #[returns(bindings_query::BasePriceResponse)]
    BasePrice {
        pair: String,
        is_long: bool,
        base_amount: Uint256,
    },

    // -----------------------------------------------------------------
    // From x/perp
    // -----------------------------------------------------------------
    #[returns(bindings_query::PositionResponse)]
    Position { trader: String, pair: String },

    #[returns(bindings_query::PositionsResponse)]
    Positions { trader: String },

    #[returns(bindings_query::ModuleParamsResponse)]
    ModuleParams {},

    #[returns(bindings_query::PremiumFractionResponse)]
    PremiumFraction { pair: String },

    #[returns(bindings_query::MetricsResponse)]
    Metrics { pair: String },

    #[returns(bindings_query::ModuleAccountsResponse)]
    ModuleAccounts {},

    // -----------------------------------------------------------------
    // From x/oracle
    // -----------------------------------------------------------------
    #[returns(bindings_query::OraclePricesResponse)]
    OraclePrices { pairs: Option<Vec<String>> },
}

impl CustomQuery for QueryMsg {}

#[cw_serde]
pub struct SudoersQueryResponse {
    pub sudoers: Sudoers,
}
