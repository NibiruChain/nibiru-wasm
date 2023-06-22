use cosmwasm_schema::cw_serde;
use cosmwasm_std::{StdResult, Timestamp, Uint128, Uint64};
use cw20::{Cw20ReceiveMsg, Denom};

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    Receive(Cw20ReceiveMsg),

    //////////////////////////
    /// Creator Operations ///
    //////////////////////////
    RegisterVestingAccount {
        master_address: Option<String>, // if given, the vesting account can be unregistered
        address: String,
        vesting_schedule: VestingSchedule,
    },
    /// only available when master_address was set
    DeregisterVestingAccount {
        address: String,
        denom: Denom,
        vested_token_recipient: Option<String>,
        left_vesting_token_recipient: Option<String>,
    },

    ////////////////////////
    /// VestingAccount Operations ///
    ////////////////////////
    Claim {
        denoms: Vec<Denom>,
        recipient: Option<String>,
    },
}

#[cw_serde]
pub enum Cw20HookMsg {
    /// Register vesting account with token transfer
    RegisterVestingAccount {
        master_address: Option<String>, // if given, the vesting account can be unregistered
        address: String,
        vesting_schedule: VestingSchedule,
    },
}

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
    pub vested_amount: Uint128,
    pub vesting_schedule: VestingSchedule,
    pub claimable_amount: Uint128,
}

#[cw_serde]
pub enum VestingSchedule {
    /// LinearVesting is used to vest tokens linearly during a time period.
    /// The total_amount will be vested during this period.
    LinearVesting {
        start_time: Uint64,      // vesting start time in second unit
        end_time: Uint64,        // vesting end time in second unit
        vesting_amount: Uint128,  // total vesting amount
    },
    LinearVestingWithCliff {
        start_time: Uint64,      // vesting start time in second unit
        end_time: Uint64,        // vesting end time in second unit
        vesting_amount: Uint128, // total vesting amount
        cliff_amount: Uint128,   // amount that will be unvested at cliff_time
        cliff_time: Uint64,      // cliff time in second unit
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

                let remaining_token = vesting_amount.checked_sub(*cliff_amount)?;
                let vested_token = remaining_token
                    .checked_mul(Uint128::from(block_time - cliff_time.u64()))?
                    .checked_div(Uint128::from(end_time - cliff_time))?;

                Ok(vested_token+cliff_amount)
            }
        }
    }
}

#[test]
fn linear_vesting_vested_amount() {
    let schedule = VestingSchedule::LinearVesting {
        start_time: Uint64::new(100),
        end_time: Uint64::new(110),
        vesting_amount: Uint128::new(1000000u128),
    };

    println!("string: {}", "100".to_string());
    println!("timestamp: {}", Timestamp::from_seconds(100));
    println!("timestamp: {}", Timestamp::from_nanos(100));

    assert_eq!(schedule.vested_amount(100).unwrap(), Uint128::zero());
    assert_eq!(
        schedule.vested_amount(105).unwrap(),
        Uint128::new(500000u128)
    );
    assert_eq!(
        schedule.vested_amount(110).unwrap(),
        Uint128::new(1000000u128)
    );
    assert_eq!(
        schedule.vested_amount(115).unwrap(),
        Uint128::new(1000000u128)
    );
}

pub struct Cliff {
    pub amount: Uint128,
    pub time: Uint64,
}

#[test]
fn linear_vesting_with_cliff_vested_amount() {
    let schedule = VestingSchedule::LinearVestingWithCliff {
        start_time: Uint64::new(100),
        end_time: Uint64::new(110),
        vesting_amount: Uint128::new(1_000_000_u128),
        cliff_amount: Uint128::new(100_000_u128),
        cliff_time: Uint64::new(105),
    };

    assert_eq!(schedule.vested_amount(100).unwrap(), Uint128::zero());
    assert_eq!(schedule.vested_amount(105).unwrap(), Uint128::new(100000u128)); // cliff time then the cliff amount
    assert_eq!( // complete vesting
        schedule.vested_amount(120).unwrap(),
        Uint128::new(1000000u128)
    );

    // other permutations
    assert_eq!(schedule.vested_amount(104).unwrap(), Uint128::zero()); // before cliff time
    assert_eq!(schedule.vested_amount(109).unwrap(), Uint128::new(820_000)); // after cliff time but before end time
}
