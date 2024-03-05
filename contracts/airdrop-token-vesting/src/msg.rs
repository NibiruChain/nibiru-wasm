use cosmwasm_schema::cw_serde;
use cosmwasm_std::{StdResult, Timestamp, Uint128, Uint64};
use cw20::Denom;

use crate::{
    errors::{CliffError, ContractError, VestingError},
    state::VestingAccount,
};

/// Structure for the message that instantiates the smart contract.
#[cw_serde]
pub struct InstantiateMsg {
    pub admin: String,
    pub managers: Vec<String>,
}

/// Enum respresenting message types for the execute entry point.
/// These express the different ways in which one can invoke the contract
/// and broadcast tx messages against it.
#[cw_serde]
pub enum ExecuteMsg {
    /// A creator operation that registers a vesting account
    /// address: String: Bech 32 address of the owner of the vesting account.
    /// vesting_schedule: VestingSchedule: The vesting schedule of the account.
    RewardUsers {
        rewards: Vec<RewardUserRequest>,
    },

    /// A creator operation that unregisters a vesting account.
    /// Args:
    /// - address: String: Bech 32 address of the owner of vesting account.
    /// - denom: Denom: The denomination of the tokens vested.
    /// - vested_token_recipient: Option<String>: Bech 32 address that will receive the vested
    ///   tokens after deregistration. If None, tokens are received by the owner address.
    /// - left_vesting_token_recipient: Option<String>: Bech 32 address that will receive the left
    ///   vesting tokens after deregistration.
    DeregisterVestingAccount {
        address: String,
        vested_token_recipient: Option<String>,
        left_vesting_token_recipient: Option<String>,
    },

    /// Claim is an operation that allows one to claim vested tokens.
    Claim {
        recipient: Option<String>,
    },

    // Withdraw allows the admin to withdraw the funds from the contract
    Withdraw {
        amount: Uint128,
        recipient: String,
    },
}

#[cw_serde]
pub struct RewardUserRequest {
    pub user_address: String,
    pub start_time: Uint64, // vesting start time in unix seconds
    pub cliff_time: Uint64, // cliff time in unix seconds
    pub end_time: Uint64,   // vesting end time in unix seconds
    pub cliff_amount: Uint128, // amount that will be unvested at cliff_time
    pub vesting_amount: Uint128, // total vesting amount
}

impl RewardUserRequest {
    ///
    /// Validates the vesting schedule.
    ///
    /// - It checks:
    ///    - that the vesting amount is not zero.
    ///    - that the cliff amount is less than or equal to the vesting amount.
    ///    - that the end time is greater than the start time.
    ///
    /// Also it calls to validate_time
    ///
    pub fn validate(&self, block_time: Timestamp) -> Result<(), VestingError> {
        if self.vesting_amount.is_zero() {
            return Err(VestingError::ZeroVestingAmount);
        }

        if self.cliff_amount > self.vesting_amount {
            return Err(VestingError::Cliff(CliffError::ExcessiveAmount {
                cliff_amount: self.cliff_amount.into(),
                vesting_amount: self.vesting_amount.into(),
            }));
        }

        if self.end_time <= self.start_time {
            return Err(VestingError::InvalidTimeRange {
                start_time: self.start_time.u64(),
                end_time: self.end_time.u64(),
            });
        }

        Ok(())
    }
}

#[cw_serde]
pub struct RewardUserResponse {
    pub user_address: String,
    pub success: bool,
    pub error_msg: String,
}

/// Enum representing the message types for the query entry point.
#[cw_serde]
pub enum QueryMsg {
    VestingAccount {
        address: String,
        start_after: Option<Denom>,
        limit: Option<u32>,
    },
}

#[cw_serde]
pub struct VestingAccountResponse {
    pub address: String,
    pub vesting: VestingData,
}

#[cw_serde]
pub struct VestingData {
    pub vesting_account: VestingAccount,
    pub vested_amount: Uint128,
    pub claimable_amount: Uint128,
}
