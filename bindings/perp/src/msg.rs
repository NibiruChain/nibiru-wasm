use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Decimal, Uint128};

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

    // TODO implement
    ClosePosition {
        pair: String,
    },

    // TODO implement
    AddMargin {
        pair: String,
    },

    // TODO implement
    RemoveMargin {
        pair: String,
    },

    // TODO implement
    MultiLiquidate {
        pair: String,
    },

    // TODO implement
    DonateToEcosystemFund {
        pair: String,
    },
}
