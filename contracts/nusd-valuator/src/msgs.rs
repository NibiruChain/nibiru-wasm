use std::collections::BTreeSet;

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Coin, Uint128};

#[cw_ownable::cw_ownable_query]
#[cw_serde]
#[derive(cosmwasm_schema::QueryResponses)]
pub enum QueryMsg {
    /// Mintable: Returns the amount of μNUSD that can be minted in exchange
    /// for the given set of "from_coins".
    #[returns(Uint128)]
    Mintable { from_coins: BTreeSet<String> },

    /// Redeemable: Returns the amount of "to_denom"  redeemable
    /// for the given "redeem_amount" of μNUSD.
    #[returns(Uint128)]
    Redeemable {
        redeem_amount: Uint128,
        to_denom: String,
    },

    /// Returns the set of token denominations that can be used as collateral.
    #[returns(BTreeSet<String>)]
    AcceptedDenoms {},

    /// Returns the set of possible redeemable coins that could be received
    /// when redeeming the given "redeem_amount" of μNUSD.
    #[returns(BTreeSet<Coin>)]
    RedeemableChoices { redeem_amount: Uint128 },
}

#[cw_ownable::cw_ownable_execute]
#[cw_serde]
pub enum ExecuteMsg {
    /// Change one denom in the "ACCEPTED_DENOMS" set to another one in-place.
    ChangeDenom { from: String, to: String },

    /// Add a denom to the set of "ACCEPTED_DENOMS", emitting the new denom set
    /// with with the "nusd_valuator/add_denom" event
    AddDenom { denom: String },

    /// Remove a denom from the set of "ACCEPTED_DENOMS", emitting the new
    /// denom set with the "nusd_valuator/remove_denom" event
    RemoveDenom { denom: String },
}

// TODO: MigrateMsg
#[cw_serde]
pub enum MigrateMsg {}

#[cw_serde]
pub struct InstantiateMsg {
    /// The owner is the only one that can use ExecuteMsg.
    pub owner: String,
    pub accepted_denoms: BTreeSet<String>,
}
