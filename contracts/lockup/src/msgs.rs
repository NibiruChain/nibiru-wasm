use cosmwasm_std::{Addr, Timestamp};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct InstantiateMsg {}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub enum ExecuteMsg {
    Lock { blocks: u64 },

    InitiateUnlock { id: u64 },

    WithdrawFunds { id: u64 },
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub enum QueryMsg {
    LocksByDenomUnlockingAfter {
        denom: String,
        unlocking_after: u64,
    },
    LocksByDenomAndAddressUnlockingAfter {
        denom: String,
        unlocking_after: u64,
        address: Addr,
    },
}
