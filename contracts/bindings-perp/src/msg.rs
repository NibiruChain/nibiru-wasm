use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Coin, CosmosMsg, CustomMsg, Decimal, Uint128};

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
/// NibiruExecuteMsgWrapper is an override of CosmosMsg::Custom. Using this msg
/// wrapper for the ExecuteMsg handlers show that their return values are valid
/// instances of CosmosMsg::Custom in a type-safe manner. It also shows how
/// CosmosMsg::Custom can be extended in the contract.
/// ExecuteMsg can be extended in the contract.
pub struct NibiruExecuteMsgWrapper {
    pub route: NibiruRoute,
    pub msg: ExecuteMsg,
}

impl CustomMsg for NibiruExecuteMsgWrapper {}

/// "From" is the workforce function for returning messages as fields of the
/// CosmosMsg enum type more easily.
impl From<NibiruExecuteMsgWrapper> for CosmosMsg<NibiruExecuteMsgWrapper> {
    fn from(original: NibiruExecuteMsgWrapper) -> Self {
        CosmosMsg::Custom(original)
    }
}

#[cw_serde]
/// Routes here refer to groups of modules on Nibiru. The idea here is to add
/// information on which module or group of modules a particular execute message  
/// belongs to.
pub enum NibiruRoute {
    Perp,
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

    MultiLiquidate {
        pair: String,
        liquidations: Vec<LiquidationArgs>,
    },

    DonateToInsuranceFund {
        sender: String,
        donation: Coin,
    },
}

#[cw_serde]
pub struct LiquidationArgs {
    pub pair: String,
    pub trader: String,
}

pub fn msg_open_position(
    pair: String,
    is_long: bool,
    quote_amount: Uint128,
    leverage: Decimal,
    base_amount_limit: Uint128,
) -> CosmosMsg<NibiruExecuteMsgWrapper> {
    NibiruExecuteMsgWrapper {
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

pub fn msg_close_position(
    sender: String,
    pair: String,
) -> CosmosMsg<NibiruExecuteMsgWrapper> {
    NibiruExecuteMsgWrapper {
        route: NibiruRoute::Perp,
        msg: ExecuteMsg::ClosePosition { sender, pair },
    }
    .into()
}

pub fn msg_add_margin(
    sender: String,
    pair: String,
    margin: Coin,
) -> CosmosMsg<NibiruExecuteMsgWrapper> {
    NibiruExecuteMsgWrapper {
        route: NibiruRoute::Perp,
        msg: ExecuteMsg::AddMargin {
            sender,
            pair,
            margin,
        },
    }
    .into()
}

pub fn msg_remove_margin(
    sender: String,
    pair: String,
    margin: Coin,
) -> CosmosMsg<NibiruExecuteMsgWrapper> {
    NibiruExecuteMsgWrapper {
        route: NibiruRoute::Perp,
        msg: ExecuteMsg::RemoveMargin {
            sender,
            pair,
            margin,
        },
    }
    .into()
}

pub fn msg_multi_liquidate(
    pair: String,
    liquidations: Vec<LiquidationArgs>,
) -> CosmosMsg<NibiruExecuteMsgWrapper> {
    NibiruExecuteMsgWrapper {
        route: NibiruRoute::Perp,
        msg: ExecuteMsg::MultiLiquidate { pair, liquidations },
    }
    .into()
}

pub fn msg_donate_to_insurance_fund(
    sender: String,
    donation: Coin,
) -> CosmosMsg<NibiruExecuteMsgWrapper> {
    NibiruExecuteMsgWrapper {
        route: NibiruRoute::Perp,
        msg: ExecuteMsg::DonateToInsuranceFund { sender, donation },
    }
    .into()
}
