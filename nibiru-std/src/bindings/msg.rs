use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Coin, CosmosMsg, CustomMsg, Decimal, Uint128};
use nibiru_macro::cw_custom;

/// NibiruMsg is an override of CosmosMsg::Custom. Using this msg
/// wrapper for the NibiruMsg handlers show that their return values are valid
/// instances of CosmosMsg::Custom in a type-safe manner. It also shows how
/// NibiruMsg can be extended in the contract.
#[cw_serde]
#[cw_custom]
pub struct NibiruMsgWrapper {
    pub route: NibiruRoute,
    pub msg: NibiruMsg,
}

/// Routes here refer to groups of operations that will be interpreted in
/// the x/wasmbinding package. The idea here is to add
/// information on which module or group of modules a particular execute message
/// belongs to.
#[cw_serde]
pub enum NibiruRoute {
    /// "oracle" is the route corresponding to custom bindings for the x/oracle module.
    Oracle,

    /// "no_op" is a valid route that doesn't do anything. It's necessary for
    /// formatting in the custom Wasm execute handler.
    NoOp,
}

#[cw_serde]
pub enum NibiruMsg {
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

    NoOp {},
}

#[cw_serde]
pub struct LiquidationArgs {
    pub pair: String,
    pub trader: String,
}
