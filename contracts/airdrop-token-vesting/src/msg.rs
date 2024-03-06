use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Timestamp, Uint128, Uint64};
use cw20::Denom;

use crate::errors::{CliffError, ContractError, VestingError};

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
        vesting_schedule: VestingSchedule,
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
    pub vesting_amount: Uint128,
    pub cliff_amount: Uint128,
}

impl RewardUserRequest {
    pub fn validate(&self) -> Result<(), ContractError> {
        if self.vesting_amount.is_zero() {
            return Err(ContractError::Vesting(VestingError::ZeroVestingAmount));
        }

        if self.cliff_amount > self.vesting_amount {
            return Err(ContractError::Vesting(VestingError::Cliff(
                CliffError::ExcessiveAmount {
                    cliff_amount: self.cliff_amount.into(),
                    vesting_amount: self.vesting_amount.into(),
                },
            )));
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
    pub vestings: Vec<VestingData>,
}

#[cw_serde]
pub struct VestingData {
    pub master_address: Option<String>,
    pub vesting_denom: Denom,
    pub vesting_amount: Uint128,
    pub vesting_schedule: VestingScheduleQueryOutput,

    pub vested_amount: Uint128,
    pub claimable_amount: Uint128,
}

#[cw_serde]
pub enum VestingSchedule {
    LinearVestingWithCliff {
        start_time: Uint64, // vesting start time in second unit
        end_time: Uint64,   // vesting end time in second unit
        cliff_time: Uint64, // cliff time in second unit
    },
}

/// For legacy, we need the query to return the schedule with the vesting amount and cliff amount
#[cw_serde]
pub enum VestingScheduleQueryOutput {
    LinearVestingWithCliff {
        start_time: Uint64, // vesting start time in second unit
        end_time: Uint64,   // vesting end time in second unit
        cliff_time: Uint64, // cliff time in second unit
        vesting_amount: Uint128,
        cliff_amount: Uint128,
    },
}

pub fn from_vesting_to_query_output(
    vesting: &VestingSchedule,
    vesting_amount: Uint128,
    cliff_amount: Uint128,
) -> VestingScheduleQueryOutput {
    match vesting {
        VestingSchedule::LinearVestingWithCliff {
            start_time,
            end_time,
            cliff_time,
        } => VestingScheduleQueryOutput::LinearVestingWithCliff {
            start_time: *start_time,
            end_time: *end_time,
            cliff_time: *cliff_time,
            vesting_amount,
            cliff_amount,
        },
    }
}

pub struct Cliff {
    pub amount: Uint128,
    pub time: Uint64,
}

impl Cliff {
    pub fn ok_time(&self, block_time: Timestamp) -> Result<(), CliffError> {
        let cliff_time_seconds = self.time.u64();
        if cliff_time_seconds < block_time.seconds() {
            return Err(CliffError::InvalidTime {
                cliff_time: cliff_time_seconds,
                block_time: block_time.seconds(),
            });
        }
        Ok(())
    }

    pub fn ok_amount(&self, vesting_amount: Uint128) -> Result<(), CliffError> {
        let cliff_amount = self.amount.u128();
        if cliff_amount > vesting_amount.u128() {
            return Err(CliffError::ExcessiveAmount {
                cliff_amount,
                vesting_amount: vesting_amount.u128(),
            });
        }
        Ok(())
    }
}

impl VestingSchedule {
    ///
    /// validate_time checks that the start_time is less than the end_time.
    /// additionally, if the vesting schedule is LinearVestingWithCliff, it checks that the cliff_time
    /// is less than the end_time.
    ///
    /// Additionally, it the vesting schedule is LinearVestingWithCliff, it checks that the cliff_time
    /// is bigger or equal to the block_time.
    ///
    pub fn validate(&self, block_time: Timestamp) -> Result<(), VestingError> {
        match self {
            VestingSchedule::LinearVestingWithCliff {
                start_time,
                end_time,
                cliff_time,
                ..
            } => {
                if end_time <= start_time {
                    return Err(VestingError::InvalidTimeRange {
                        start_time: start_time.u64(),
                        end_time: end_time.u64(),
                    });
                }
                let cliff = Cliff {
                    amount: Uint128::zero(),
                    time: *cliff_time,
                };
                cliff.ok_time(block_time)?;
                Ok(())
            }
        }
    }
}
