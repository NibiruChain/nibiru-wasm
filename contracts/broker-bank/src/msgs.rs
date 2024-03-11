use std::collections::BTreeSet;

use cosmwasm_schema::cw_serde;
use cosmwasm_std as cw;

use crate::oper_perms;

#[cw_ownable::cw_ownable_execute]
#[cw_serde]
pub enum ExecuteMsg {
    /// Send coins to an account the set of "TO_ADDRS", appending transaction
    /// info to the "LOGS". This tx msg emits a "broker/bank/send" event
    BankSend { coins: Vec<cw::Coin>, to: String },

    /// ToggleHalt: Toggles on or off the ability of the operators to use the
    /// smart contract. Only callable by the contract owner.
    ToggleHalt {},

    /// Withdraw coins from the broker smart contract balance. Only callable by
    /// the contract owner.
    Withdraw {
        to: Option<String>,
        denoms: BTreeSet<String>,
    },

    /// Withdraw all coins from the broker smart contract balance. Only callable
    /// by the contract owner.
    WithdrawAll { to: Option<String> },

    /// TODO: owner
    EditOpers(oper_perms::Action),
    // TODO: feat(broker-bank): Clear logs tx
}

#[cw_ownable::cw_ownable_query]
#[cw_serde]
#[derive(cosmwasm_schema::QueryResponses)]
pub enum QueryMsg {
    /// Perms: Query the smart contract owner, set of operators, and whether
    /// operator set is "halted".
    #[returns(PermsStatus)]
    Perms {},
    // TODO: feat(broker-bank): Logs query
}

#[cw_serde]
pub struct PermsStatus {
    pub is_halted: bool,
    pub perms: oper_perms::Permissions,
}

#[cw_serde]
pub struct InstantiateMsg {
    /// The owner is the only one that can use ExecuteMsg.
    pub owner: String,
    pub to_addrs: BTreeSet<String>,
    pub opers: BTreeSet<String>,
}