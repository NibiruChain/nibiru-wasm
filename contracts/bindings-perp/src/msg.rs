use cosmwasm_schema::cw_serde;
use cosmwasm_std::{
    Coin, CosmosMsg, CustomMsg, Decimal, Response, StdResult, Uint128,
};

use nibiru_bindings::route::NibiruRoute;
use nibiru_macro::cw_custom;

#[cw_serde]
pub struct InstantiateMsg {}

/// NibiruExecuteMsg is an override of CosmosMsg::Custom. Using this msg
/// wrapper for the ExecuteMsg handlers show that their return values are valid
/// instances of CosmosMsg::Custom in a type-safe manner. It also shows how
/// ExecuteMsg can be extended in the contract.
#[cw_serde]
#[cw_custom]
pub struct NibiruExecuteMsg {
    pub route: NibiruRoute,
    pub msg: ExecuteMsgWithSender,
}

#[cw_serde]
pub enum ExecuteMsgWithSender {

    OpenPosition {
        sender: String,
        pair: String,
        is_long: bool,
        quote_amount: Uint128,
        leverage: Decimal,
        base_amount_limit: Uint128,
    },

    ClosePosition {
        sender: String,
        pair: String,
    },

    AddMargin {
        sender: String,
        pair: String,
        margin: Coin,
    },

    RemoveMargin {
        sender: String,
        pair: String,
        margin: Coin,
    },

    DonateToInsuranceFund {
        sender: String,
        donation: Coin,
    },

    NoOp {},
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
        sender: String,
        pair: String,
        is_long: bool,
        quote_amount: Uint128,
        leverage: Decimal,
        base_amount_limit: Uint128,
    ) -> CosmosMsg<NibiruExecuteMsg> {
        NibiruExecuteMsg {
            route: NibiruRoute::Perp,
            msg: ExecuteMsg::OpenPosition {
                sender,
                pair,
                is_long,
                quote_amount,
                leverage,
                base_amount_limit,
            },
        }
        .into()
    }

    pub fn close_position(
        sender: String,
        pair: String,
    ) -> CosmosMsg<NibiruExecuteMsg> {
        NibiruExecuteMsg {
            route: NibiruRoute::Perp,
            msg: ExecuteMsg::ClosePositionWithSender { sender, pair },
        }
        .into()
    }

    pub fn add_margin(
        sender: String,
        pair: String,
        margin: Coin,
    ) -> CosmosMsg<NibiruExecuteMsg> {
        NibiruExecuteMsg {
            route: NibiruRoute::Perp,
            msg: ExecuteMsg::AddMargin {
                sender,
                pair,
                margin,
            },
        }
        .into()
    }

    pub fn remove_margin(
        sender: String,
        pair: String,
        margin: Coin,
    ) -> CosmosMsg<NibiruExecuteMsg> {
        NibiruExecuteMsg {
            route: NibiruRoute::Perp,
            msg: ExecuteMsgWithSender::RemoveMargin {
                sender,
                pair,
                margin,
            },
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
        sender: String,
        donation: Coin,
    ) -> CosmosMsg<NibiruExecuteMsg> {
        NibiruExecuteMsg {
            route: NibiruRoute::Perp,
            msg: ExecuteMsg::DonateToInsuranceFund { sender, donation },
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
