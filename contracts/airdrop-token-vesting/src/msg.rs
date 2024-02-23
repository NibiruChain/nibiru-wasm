use cosmwasm_schema::cw_serde;
use cosmwasm_std::{StdResult, Timestamp, Uint128, Uint64};

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
    pub cliff_amount: Option<Uint128>,
}

impl RewardUserRequest {
    pub fn validate(
        &self,
        vesting_schedule: VestingSchedule,
    ) -> Result<(), ContractError> {
        if self.vesting_amount.is_zero() {
            return Err(ContractError::Vesting(VestingError::ZeroVestingAmount));
        }

        if let VestingSchedule::LinearVestingWithCliff { .. } = vesting_schedule
        {
            if self.cliff_amount.is_none()
                || self.cliff_amount.unwrap().is_zero()
            {
                return Err(ContractError::Vesting(VestingError::Cliff(
                    CliffError::ZeroAmount,
                )));
            }

            if self.cliff_amount.unwrap() > self.vesting_amount {
                return Err(ContractError::Vesting(VestingError::Cliff(
                    CliffError::ExcessiveAmount {
                        cliff_amount: self.cliff_amount.unwrap().into(),
                        vesting_amount: self.vesting_amount.into(),
                    },
                )));
            }
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
    VestingAccount { address: String },
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

#[cw_serde]
pub enum VestingSchedule {
    /// LinearVesting is used to vest tokens linearly during a time period.
    /// The total_amount will be vested during this period.
    LinearVesting {
        start_time: Uint64,      // vesting start time in second unit
        end_time: Uint64,        // vesting end time in second unit
        vesting_amount: Uint128, // total vesting amount
    },
    LinearVestingWithCliff {
        start_time: Uint64,      // vesting start time in second unit
        end_time: Uint64,        // vesting end time in second unit
        vesting_amount: Uint128, // total vesting amount
        cliff_amount: Uint128,   // amount that will be unvested at cliff_time
        cliff_time: Uint64,      // cliff time in second unit
    },
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
        if self.amount.is_zero() {
            return Err(CliffError::ZeroAmount);
        }

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
    pub fn vested_amount(&self, block_time: u64) -> StdResult<Uint128> {
        match self {
            VestingSchedule::LinearVesting {
                start_time,
                end_time,
                vesting_amount,
            } => {
                if block_time <= start_time.u64() {
                    return Ok(Uint128::zero());
                }

                if block_time >= end_time.u64() {
                    return Ok(*vesting_amount);
                }

                let vested_token = vesting_amount
                    .checked_mul(Uint128::from(block_time - start_time.u64()))?
                    .checked_div(Uint128::from(end_time - start_time))?;

                Ok(vested_token)
            }
            VestingSchedule::LinearVestingWithCliff {
                start_time: _start_time,
                end_time,
                vesting_amount,
                cliff_amount,
                cliff_time,
            } => {
                if block_time < cliff_time.u64() {
                    return Ok(Uint128::zero());
                }

                if block_time == cliff_time.u64() {
                    return Ok(*cliff_amount);
                }

                if block_time >= end_time.u64() {
                    return Ok(*vesting_amount);
                }

                let remaining_token =
                    vesting_amount.checked_sub(*cliff_amount)?;
                let vested_token = remaining_token
                    .checked_mul(Uint128::from(block_time - cliff_time.u64()))?
                    .checked_div(Uint128::from(end_time - cliff_time))?;

                Ok(vested_token + cliff_amount)
            }
        }
    }

    ///
    /// Validates the vesting schedule.
    ///
    /// - If the VestingSchedule is LinearVesting, it checks that the vesting amount is not zero.
    /// - If the VestingSchedule is LinearVestingWithCliff, it checks:
    ///    - that the vesting amount is not zero.
    ///    - that the cliff amount is not zero.
    ///    - that the cliff amount is less than or equal to the vesting amount.
    ///
    /// Also it calls to validate_time
    ///
    pub fn validate(&self, block_time: Timestamp) -> Result<(), VestingError> {
        self.validate_time(block_time)?;
        match &self {
            VestingSchedule::LinearVesting {
                start_time: _,
                end_time: _,
                vesting_amount,
            } => {
                if vesting_amount.is_zero() {
                    return Err(VestingError::ZeroVestingAmount);
                }
                Ok(())
            }

            VestingSchedule::LinearVestingWithCliff {
                start_time: _,
                end_time: _,
                vesting_amount,
                cliff_time,
                cliff_amount,
            } => {
                if vesting_amount.is_zero() {
                    return Err(VestingError::ZeroVestingAmount);
                }
                let cliff = Cliff {
                    amount: *cliff_amount,
                    time: *cliff_time,
                };
                cliff.ok_amount(*vesting_amount)?;
                Ok(())
            }
        }
    }

    ///
    /// validate_time checks that the start_time is less than the end_time.
    /// additionally, if the vesting schedule is LinearVestingWithCliff, it checks that the cliff_time
    /// is less than the end_time.
    ///
    /// Additionally, it the vesting schedule is LinearVestingWithCliff, it checks that the cliff_time
    /// is bigger or equal to the block_time.
    ///
    pub fn validate_time(
        &self,
        block_time: Timestamp,
    ) -> Result<(), VestingError> {
        match self {
            VestingSchedule::LinearVesting {
                start_time,
                end_time,
                ..
            } => {
                if end_time <= start_time {
                    return Err(VestingError::InvalidTimeRange {
                        start_time: start_time.u64(),
                        end_time: end_time.u64(),
                    });
                }
                Ok(())
            }
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

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::contract::tests::TestResult;

    #[test]
    fn linear_vesting_vested_amount() -> TestResult {
        let schedule = VestingSchedule::LinearVesting {
            start_time: Uint64::new(100),
            end_time: Uint64::new(110),
            vesting_amount: Uint128::new(1000000u128),
        };

        assert_eq!(schedule.vested_amount(100)?, Uint128::zero());
        assert_eq!(schedule.vested_amount(105)?, Uint128::new(500000u128));
        assert_eq!(schedule.vested_amount(110)?, Uint128::new(1000000u128));
        assert_eq!(schedule.vested_amount(115)?, Uint128::new(1000000u128));

        Ok(())
    }

    #[test]
    fn linear_vesting_with_cliff_vested_amount() -> TestResult {
        let schedule = VestingSchedule::LinearVestingWithCliff {
            start_time: Uint64::new(100),
            end_time: Uint64::new(110),
            vesting_amount: Uint128::new(1_000_000_u128),
            cliff_amount: Uint128::new(100_000_u128),
            cliff_time: Uint64::new(105),
        };

        assert_eq!(schedule.vested_amount(100)?, Uint128::zero());
        assert_eq!(schedule.vested_amount(105)?, Uint128::new(100000u128)); // cliff time then the cliff amount
        assert_eq!(schedule.vested_amount(120)?, Uint128::new(1000000u128)); // complete vesting
        assert_eq!(schedule.vested_amount(104)?, Uint128::zero()); // before cliff time
        assert_eq!(schedule.vested_amount(109)?, Uint128::new(820_000)); // after cliff time but before end time

        Ok(())
    }
}
