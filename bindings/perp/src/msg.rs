use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Decimal, Uint128, Coin};

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    // TODO implement
    OpenPosition {
        pair: String,
        side: u8,
        quote_asset_amount: Uint128,
        leverage: Decimal,
        base_asset_amount_limit: Uint128,
    },

    // TODO handler
    ClosePosition {
        sender: String,
        pair: String,
    },

    // TODO handler
    AddMargin {
        sender: String,
        pair: String,
        margin: Coin,
    },

    // TODO handler
    RemoveMargin {
        sender: String,
        pair: String,
        margin: Coin,
    },

    // TODO implement
    MultiLiquidate {
        pair: String,
        liquidations: Vec<LiquidationArgs>
    },

    // TODO implement
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
