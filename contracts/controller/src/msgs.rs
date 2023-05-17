use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Decimal, Uint256, Uint64};

use crate::state::Whitelist;

/// InitMsg specifies the args for the instantiate entry point of the contract.
#[cw_serde]
pub struct InitMsg {
    pub admin: String,
}

/// ExecuteMsg specifies the args for the execute entry point of the contract.
#[cw_serde]
pub enum ExecuteMsg {
    SetMarketEnabled {
        pair: String,
        enabled: bool,
    },
    InsuranceFundWithdraw {
        amount: Uint256,
        to: String,
    },
    EditOracleParams {
        vote_period: Option<Uint64>,
        vote_threshold: Option<Decimal>,
        reward_band: Option<Decimal>,
        whitelist: Option<Vec<String>>,
        slash_fraction: Option<Decimal>,
        slash_window: Option<Uint64>,
        min_valid_per_window: Option<Decimal>,
        twap_lookback_window: Option<Uint64>,
        min_voters: Option<Uint64>,
        validator_fee_ratio: Option<Decimal>,
    },

    AddMember {
        address: String,
    },
    RemoveMember {
        address: String,
    },
    ChangeAdmin {
        address: String,
    },
}

/// QueryMsg specifies the args for the query entry point of the contract.
#[cw_serde]
pub enum QueryMsg {
    IsMember { address: String },
    Whitelist {},
}

#[cw_serde]
pub struct IsMemberResponse {
    pub is_member: bool,
    pub whitelist: Whitelist,
}

#[cw_serde]
pub struct WhitelistResponse {
    pub whitelist: Whitelist,
}
