use std::collections::BTreeSet;

use broker_bank::oper_perms;
use cosmwasm_schema::cw_serde;
use cosmwasm_std::Uint128;

/// Enum respresenting message types for the execute entry point.
/// These express the different ways in which one can invoke the contract
/// and broadcast tx messages against it.
#[cw_ownable::cw_ownable_execute]
#[cw_serde]
pub enum ExecuteMsg {
    /// Toggles whether "operators" can invoke the smart contract. This acts a
    /// security feature for the contract owner to disable non-owner permissions
    /// quickly without sending multiple calls of `ExecuteMsg::EditOpers`.
    ///
    /// When the smart contract is halted, the owner can still use the everything,
    /// while operators cannot.
    ToggleHalt {},

    /// Withdraw coins from the broker smart contract balance. Only callable by
    /// the contract owner.
    Withdraw {
        to: Option<String>,
        denoms: BTreeSet<String>,
    },

    /// Withdraw all coins from the broker smart contract balance. Only callable
    /// by the contract owner.
    WithdrawAll {
        to: Option<String>,
    },

    /// Unstake allows to unstake a given amount of tokens from a set of
    /// validators. The UnstakeMsgs defines the tokens amount and address
    /// of the validator.
    Unstake {
        unstake_msgs: Vec<UnstakeMsg>,
    },

    /// Manager functions

    /// Stake allows to stake a given amount of tokens to a set of validators.
    /// The StakeMsgs defines the share of tokens distributed and the validator
    /// to which the stake is made.
    Stake {
        amount: Uint128,
        stake_msgs: Vec<StakeMsg>,
    },

    EditOpers(oper_perms::Action),
}

#[cw_serde]
pub struct UnstakeMsg {
    pub amount: Uint128,
    pub validator: String,
}

#[cw_serde]
pub struct StakeMsg {
    pub share: Uint128,
    pub validator: String,
}
