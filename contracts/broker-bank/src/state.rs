use cosmwasm_schema::cw_serde;
use cosmwasm_std::Event;
use cw_storage_plus::{Deque, Item};
use std::collections::BTreeSet;

/// TO_ADDRS: Defines the set of addresses that can receive transfers from the
/// contract.
pub const TO_ADDRS: Item<BTreeSet<String>> = Item::new("to_addrs");

/// OPERATORS: The set of accounts that can operate the broker smart contract.
/// Operators cannot add or remove other operators or withdraw funds.
pub const OPERATORS: Item<BTreeSet<String>> = Item::new("operators");

/// LOGS: Stateful `cw_storage_plus::Deque` holding transaction and event logs.
/// The "Deque" increments every time the smart contract is invoked to send
/// funds, withdraw, or change operator permissions.
pub const LOGS: Deque<Log> = Deque::new("logs");

/// IS_HALTED: An on and off switch the owner can toggle for the operators.
pub const IS_HALTED: Item<bool> = Item::new("is_halted");

/// Log: An entry in the "logs" state of the contract. Each `Log` records a
/// successful execute transaction on the broker contract.
#[cw_serde]
pub struct Log {
    pub block_height: u64,
    pub sender_addr: String,
    pub event: Event,
}