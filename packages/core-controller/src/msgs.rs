#![allow(deprecated)]
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
    #[deprecated(note = "Needs MsgServer impl added to NibiruChain/nibiru")]
    SetMarketEnabled {
        pair: String,
        enabled: bool,
    },
    #[deprecated(note = "Needs MsgServer impl added to NibiruChain/nibiru")]
    WithdrawPerpFund {
        amount: Uint256,
        to: String,
    },
    #[deprecated(note = "Needs MsgServer impl added to NibiruChain/nibiru")]
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
    #[deprecated(note = "Needs MsgServer impl added to NibiruChain/nibiru")]
    CreateMarket {
        pair: String,
        peg_mult: Decimal,
        sqrt_depth: Decimal,
        market_params: Option<MarketParams>,
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

#[cw_serde]
pub struct MarketParams {
    pub pair: String,
    pub enabled: bool,
    /// percentage that a single open or close position can alter the reserve
    /// amounts
    pub price_fluctuation_limit_ratio: Decimal,
    /// the minimum margin ratio which a user must maintain on this market
    pub maintenance_margin_ratio: Decimal,
    /// the maximum leverage a user is able to be taken on this market
    pub max_leverage: Decimal,
    /// Maximum daily funding rate, where funding_payed = size * index_price *
    /// (max_funding_rate / num_payments) In our case, payments occur every 30
    /// minutes, so num_payments is 48.
    pub max_funding_rate: Decimal,
    // Latest cumulative premium fraction for a given pair.
    // Calculated once per funding rate interval.
    // A premium fraction is the difference between mark and index, divided by the
    // number of payments per day. (mark - index) / # payments in a day
    pub latest_cpf: Decimal,
    /// the percentage of the notional given to the exchange when trading
    pub exchange_fee_ratio: Decimal,
    /// the percentage of the notional transferred to the ecosystem fund when
    /// trading
    pub ecosystem_fund_fee_ratio: Decimal,
    /// the percentage of liquidated position that will be
    /// given to out as a reward. Half of the liquidation fee is given to the
    /// liquidator, and the other half is given to the ecosystem fund.
    pub liquidation_fee_ratio: Decimal,
    /// the portion of the position size we try to liquidate if the available
    /// margin is higher than liquidation fee
    pub partial_liquidation_ratio: Decimal,
    /// specifies the interval on which the funding rate is updated
    pub funding_rate_epoch_id: String,
    /// amount of time to look back for TWAP calculations
    pub twap_lookback_window: Uint256,
}
