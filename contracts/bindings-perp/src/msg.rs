use cosmwasm_schema::cw_serde;
use cosmwasm_std::{
    Coin, CosmosMsg, CustomMsg, CustomQuery, Decimal, Response, StdResult,
    Uint128, Uint256,
};

use nibiru_bindings::route::NibiruRoute;
use nibiru_macro::cw_custom;

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
    OpenPosition {
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
    pub fn open_position(
        pair: String,
        is_long: bool,
        quote_amount: Uint128,
        leverage: Decimal,
        base_amount_limit: Uint128,
    ) -> CosmosMsg<NibiruExecuteMsg> {
        NibiruExecuteMsg {
            route: NibiruRoute::Perp,
            msg: ExecuteMsg::OpenPosition {
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
pub enum QueryMsg {
    Sudoers {},
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

impl CustomQuery for QueryMsg {}

#[cw_serde]
pub struct SudoersQueryResponse {
    pub sudoers: Sudoers,
}
