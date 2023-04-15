use cosmwasm_std::{Addr};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct InstantiateMsg {
    pub lockup_contract_address: Addr,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub enum QueryMsg {
    ProgramFunding { program_id: u64 },
    EpochInfo { program_id: u64, epoch_number: u64 },
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub enum ExecuteMsg {
    CreateProgram {
        denom: String,
        epochs: u64,
        epoch_block_duration: u64,
        min_lockup_blocks: u64,
    },

    FundProgram {
        id: u64,
    },

    ProcessEpoch {
        id: u64,
    },

    WithdrawRewards {
        id: u64,
    },
}
