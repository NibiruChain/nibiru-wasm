use anyhow::anyhow;
use cosmwasm_std::{
    coin,
    testing::{self, MockApi, MockQuerier, MockStorage},
    Empty, Env, OwnedDeps, StdError, Timestamp, Uint128, Uint64,
};
use cw20::Denom;
use token_vesting::{
    contract::{execute, instantiate},
    errors::ContractError,
    msg::{ExecuteMsg, InstantiateMsg, VestingSchedule},
};

use super::helpers::TestResult;

pub fn mock_env_with_time(block_time: u64) -> Env {
    let mut env = testing::mock_env();
    env.block.time = Timestamp::from_seconds(block_time);
    env
}

/// Convenience function for instantiating the contract at and setting up
/// the env to have the given block time.
pub fn setup_with_block_time(
    block_time: u64,
) -> anyhow::Result<(OwnedDeps<MockStorage, MockApi, MockQuerier, Empty>, Env)> {
    let mut deps = testing::mock_dependencies();
    let env = mock_env_with_time(block_time);
    instantiate(
        deps.as_mut(),
        env.clone(),
        testing::mock_info("admin-sender", &[]),
        InstantiateMsg {},
    )?;
    Ok((deps, env))
}

#[test]
fn deregister_err_nonexistent_vesting_account() -> TestResult {
    let (mut deps, _env) = setup_with_block_time(0)?;

    let msg = ExecuteMsg::DeregisterVestingAccount {
        address: "nonexistent".to_string(),
        denom: Denom::Native("token".to_string()),
        vested_token_recipient: None,
        left_vesting_token_recipient: None,
    };

    let res = execute(
        deps.as_mut(),
        testing::mock_env(),
        testing::mock_info("admin-sender", &[]),
        msg,
    );

    match res {
        Ok(_) => Err(anyhow!("Unexpected result: {:#?}", res)),
        Err(ContractError::Std(StdError::GenericErr { msg, .. })) => {
            assert!(msg.contains("vesting entry is not found for denom"));
            Ok(())
        }
        Err(err) => Err(anyhow!("Unexpected error: {:#?}", err)),
    }
}

#[test]
fn deregister_err_unauthorized_vesting_account() -> TestResult {
    // Set up the environment with a block time before the vesting start time
    let (mut deps, env) = setup_with_block_time(50)?;

    let register_msg = ExecuteMsg::RegisterVestingAccount {
        master_address: Some("addr0002".to_string()),
        address: "addr0001".to_string(),
        vesting_schedule: VestingSchedule::LinearVesting {
            start_time: Uint64::new(100),
            end_time: Uint64::new(110),
            vesting_amount: Uint128::new(1000000u128),
        },
    };

    execute(
        deps.as_mut(),
        env.clone(), // Use the custom environment with the adjusted block time
        testing::mock_info("admin-sender", &[coin(1000000, "token")]),
        register_msg,
    )?;

    // Try to deregister with unauthorized sender
    let msg = ExecuteMsg::DeregisterVestingAccount {
        address: "addr0001".to_string(),
        denom: Denom::Native("token".to_string()),
        vested_token_recipient: None,
        left_vesting_token_recipient: None,
    };

    let res = execute(
        deps.as_mut(),
        env, // Use the custom environment with the adjusted block time
        testing::mock_info("addr0003", &[]),
        msg,
    );
    match res {
        Err(ContractError::Std(StdError::GenericErr { msg, .. }))
            if msg == "unauthorized" => {}
        _ => return Err(anyhow!("Unexpected result: {:?}", res)),
    }

    Ok(())
}

#[test]
fn deregister_successful() -> TestResult {
    // Set up the environment with a block time before the vesting start time
    let (mut deps, env) = setup_with_block_time(50)?;

    let register_msg = ExecuteMsg::RegisterVestingAccount {
        master_address: Some("addr0002".to_string()),
        address: "addr0001".to_string(),
        vesting_schedule: VestingSchedule::LinearVesting {
            start_time: Uint64::new(100),
            end_time: Uint64::new(110),
            vesting_amount: Uint128::new(1000000u128),
        },
    };

    execute(
        deps.as_mut(),
        env.clone(), // Use the custom environment with the adjusted block time
        testing::mock_info("admin-sender", &[coin(1000000, "token")]),
        register_msg,
    )?;

    // Deregister with the master address
    let msg = ExecuteMsg::DeregisterVestingAccount {
        address: "addr0001".to_string(),
        denom: Denom::Native("token".to_string()),
        vested_token_recipient: None,
        left_vesting_token_recipient: None,
    };

    let _res = execute(
        deps.as_mut(),
        env, // Use the custom environment with the adjusted block time
        testing::mock_info("addr0002", &[]),
        msg,
    )?;

    Ok(())
}
