use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Coin, Decimal, Uint128};

use crate::state::Sudoers;

/// InitMsg: message type for smart contract instantiation
#[cw_serde]
pub struct InitMsg {
    pub admin: Option<String>,
}

// ---------------------------------------------------------------------------
// Entry Point - Execute
// ---------------------------------------------------------------------------

/// ExecuteMsg: message type for invoking or executing the smart contract
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
}

#[cw_serde]
pub struct LiquidationArgs {
    pub pair: String,
    pub trader: String,
}

// ---------------------------------------------------------------------------
// Entry Point - Query
// ---------------------------------------------------------------------------

#[cw_serde]
pub enum QueryMsg {
    Sudoers {},

    /// Query perp markets
    Markets {},

    /// Query a single Nibi-Perps position
    Position {
        trader: String,
        pair: String,
    },

    /// Query all Nibi-Perps positions for the trader
    Positions {
        trader: String,
    },

    /// Query the Nibi-Perps module accounts
    ModuleAccounts {},

    /// Query prices from the Nibi-Oracle.
    OraclePrices {
        pairs: Option<Vec<String>>,
    },
}

#[cw_serde]
pub struct SudoersQueryResponse {
    pub sudoers: Sudoers,
}
