use crate::contract::tests::TestResult;
use crate::contract::{execute, instantiate, query};
use crate::errors::{CliffError, ContractError, VestingError};
use crate::msg::{
    ExecuteMsg, InstantiateMsg, QueryMsg, RewardUserRequest,
    VestingAccountResponse, VestingData, VestingSchedule,
};

use cosmwasm_std::testing::{MockApi, MockQuerier, MockStorage};
use cosmwasm_std::{coin, MessageInfo};
use cosmwasm_std::{
    from_json,
    testing::{mock_dependencies, mock_env, mock_info},
    Attribute, BankMsg, Coin, Env, OwnedDeps, Response, StdError, SubMsg,
    Timestamp, Uint128, Uint64,
};

#[test]
fn proper_initialization() -> TestResult {
    let mut deps = mock_dependencies();

    let msg = InstantiateMsg {
        admin: "admin-sender".to_string(),
        managers: vec!["admin-sender".to_string()],
    };

    let info = mock_info("addr0000", &[coin(1000, "nibi")]);

    let _res = instantiate(deps.as_mut(), mock_env(), info, msg)?;
    Ok(())
}

#[test]
fn invalid_coin_sent_instantiation() -> TestResult {
    let mut deps = mock_dependencies();

    let msg = InstantiateMsg {
        admin: "admin-sender".to_string(),
        managers: vec!["admin-sender".to_string()],
    };

    // No coins sent
    let res = instantiate(
        deps.as_mut(),
        mock_env(),
        mock_info("addr0000", &[]),
        msg.clone(),
    );
    match res {
        Err(err) => {
            assert_eq!(
                err,
                StdError::GenericErr {
                    msg: "must deposit exactly one type of token".to_string(),
                }
            )
        }
        Ok(_) => panic!("Expected error but got success: {res:?}"),
    }

    // 2 coins sent
    let res = instantiate(
        deps.as_mut(),
        mock_env(),
        mock_info("addr0000", &[coin(1000, "nibi"), coin(1000, "usd")]),
        msg.clone(),
    );
    match res {
        Err(err) => {
            assert_eq!(
                err,
                StdError::GenericErr {
                    msg: "must deposit exactly one type of token".to_string(),
                }
            )
        }
        Ok(_) => panic!("Expected error but got success: {res:?}"),
    }

    // 0 amount coins sent
    let res = instantiate(
        deps.as_mut(),
        mock_env(),
        mock_info("addr0000", &[coin(0, "nibi")]),
        msg,
    );
    match res {
        Err(err) => {
            assert_eq!(
                err,
                StdError::GenericErr {
                    msg: "must deposit some token".to_string(),
                }
            )
        }
        Ok(_) => panic!("Expected error but got success: {res:?}"),
    }

    Ok(())
}

#[test]
fn invalid_manangers_initialization() -> TestResult {
    let mut deps = mock_dependencies();

    let msg = InstantiateMsg {
        admin: "admin-sender".to_string(),
        managers: vec![],
    };

    let info = mock_info("addr0000", &[coin(1000, "nibi")]);

    let res = instantiate(deps.as_mut(), mock_env(), info.clone(), msg.clone());
    match res {
        Err(err) => {
            assert_eq!(
                err,
                StdError::GenericErr {
                    msg: "managers cannot be empty".to_string(),
                }
            )
        }
        Ok(_) => panic!("Expected error but got success: {res:?}"),
    }

    let msg = InstantiateMsg {
        admin: "admin-sender".to_string(),
        managers: vec!["".to_string()],
    };
    let res = instantiate(deps.as_mut(), mock_env(), info.clone(), msg.clone());
    match res {
        Err(err) => {
            assert_eq!(
                err,
                StdError::GenericErr {
                    msg: "Invalid input: human address too short for this mock implementation (must be >= 3).".to_string(),
                }
            )
        }
        Ok(_) => panic!("Expected error but got success: {res:?}"),
    }

    let msg = InstantiateMsg {
        admin: "admin-sender".to_string(),
        managers: vec!["admin-sender".to_string(), "".to_string()],
    };
    let res = instantiate(deps.as_mut(), mock_env(), info.clone(), msg.clone());
    match res {
        Err(err) => {
            assert_eq!(
                err,
                StdError::GenericErr {
                    msg: "Invalid input: human address too short for this mock implementation (must be >= 3).".to_string(),
                }
            )
        }
        Ok(_) => panic!("Expected error but got success: {res:?}"),
    }

    let msg = InstantiateMsg {
        admin: "".to_string(),
        managers: vec!["admin-sender".to_string()],
    };
    let res = instantiate(deps.as_mut(), mock_env(), info.clone(), msg.clone());
    match res {
        Err(err) => {
            assert_eq!(
                err,
                StdError::GenericErr {
                    msg: "Invalid input: human address too short for this mock implementation (must be >= 3).".to_string(),
                }
            )
        }
        Ok(_) => panic!("Expected error but got success: {res:?}"),
    }

    Ok(())
}

#[test]
fn invalid_managers() -> TestResult {
    let mut deps = mock_dependencies();

    let msg = InstantiateMsg {
        admin: "admin-sender".to_string(),
        managers: vec!["admin-manager".to_string()],
    };

    // No coins sent
    let res = instantiate(
        deps.as_mut(),
        mock_env(),
        mock_info("addr0000", &[]),
        msg.clone(),
    );
    match res {
        Err(err) => {
            assert_eq!(
                err,
                StdError::GenericErr {
                    msg: "must deposit exactly one type of token".to_string(),
                }
            )
        }
        Ok(_) => panic!("Expected error but got success: {res:?}"),
    }

    // 2 coins sent
    let res = instantiate(
        deps.as_mut(),
        mock_env(),
        mock_info("addr0000", &[coin(1000, "nibi"), coin(1000, "usd")]),
        msg,
    );
    match res {
        Err(err) => {
            assert_eq!(
                err,
                StdError::GenericErr {
                    msg: "must deposit exactly one type of token".to_string(),
                }
            )
        }
        Ok(_) => panic!("Expected error but got success: {res:?}"),
    }

    Ok(())
}

#[test]
fn register_cliff_vesting_account_with_native_token() -> TestResult {
    let mut deps = mock_dependencies();
    let _res = instantiate(
        deps.as_mut(),
        mock_env(),
        mock_info("addr0000", &[coin(2000, "uusd")]),
        InstantiateMsg {
            admin: "addr0000".to_string(),
            managers: vec!["admin-sender".to_string()],
        },
    )?;

    let mut env = mock_env();
    env.block.time = Timestamp::from_seconds(100);

    let create_msg = |start_time: u64,
                      end_time: u64,
                      vesting_amount: u128,
                      cliff_amount: Option<u128>,
                      cliff_time: u64|
     -> ExecuteMsg {
        ExecuteMsg::RewardUsers {
            master_address: None,
            rewards: vec![RewardUserRequest {
                user_address: "addr0001".to_string(),
                vesting_amount: Uint128::new(vesting_amount),
                cliff_amount: cliff_amount.map(Uint128::new),
            }],
            vesting_schedule: VestingSchedule::LinearVestingWithCliff {
                start_time: Uint64::new(start_time),
                end_time: Uint64::new(end_time),
                vesting_amount: Uint128::zero(),
                cliff_amount: Uint128::zero(),
                cliff_time: Uint64::new(cliff_time),
            },
        }
    };

    // unauthorized sender
    let msg = create_msg(100, 110, 0, Some(1000), 105);
    require_error(
        &mut deps,
        &env,
        mock_info("addr0042", &[]),
        msg,
        StdError::generic_err("Unauthorized").into(),
    );

    // zero amount vesting token
    let msg = create_msg(100, 110, 0, Some(1000), 105);
    require_error(
        &mut deps,
        &env,
        mock_info("addr0000", &[]),
        msg,
        ContractError::Vesting(VestingError::ZeroVestingAmount),
    );

    // zero amount cliff token
    let msg = create_msg(100, 110, 1000, Some(0), 105);
    require_error(
        &mut deps,
        &env,
        mock_info("addr0000", &[]),
        msg,
        ContractError::Vesting(VestingError::Cliff(CliffError::ZeroAmount)),
    );

    // none amount cliff token
    let msg = create_msg(100, 110, 1000, None, 105);
    require_error(
        &mut deps,
        &env,
        mock_info("addr0000", &[]),
        msg,
        ContractError::Vesting(VestingError::Cliff(CliffError::ZeroAmount)),
    );

    // cliff time less than block time
    let msg = create_msg(100, 110, 1000, Some(1000), 99);
    require_error(
        &mut deps,
        &env,
        mock_info("addr0000", &[Coin::new(1000u128, "uusd")]),
        msg,
        ContractError::Vesting(VestingError::Cliff(CliffError::InvalidTime {
            cliff_time: 99,
            block_time: 100,
        })),
    );

    // end time less than start time
    let msg = create_msg(110, 100, 1000, Some(1000), 105);
    require_error(
        &mut deps,
        &env,
        mock_info("addr0000", &[Coin::new(1000u128, "uusd")]),
        msg,
        ContractError::Vesting(VestingError::InvalidTimeRange {
            start_time: 110,
            end_time: 100,
        }),
    );

    // cliff amount greater than vesting amount
    let (vesting_amount, cliff_amount, cliff_time) = (1000, 1001, 105);
    let msg =
        create_msg(100, 110, vesting_amount, Some(cliff_amount), cliff_time);
    require_error(
        &mut deps,
        &env,
        mock_info("addr0000", &[Coin::new(1000u128, "uusd")]),
        msg,
        ContractError::Vesting(
            CliffError::ExcessiveAmount {
                cliff_amount,
                vesting_amount,
            }
            .into(),
        ),
    );

    // deposit amount higher than unallocated
    let (vesting_amount, cliff_amount, cliff_time) = (10000, 250, 105);
    let msg =
        create_msg(100, 110, vesting_amount, Some(cliff_amount), cliff_time);
    require_error(
        &mut deps,
        &env,
        mock_info("addr0000", &[Coin::new(999u128, "uusd")]),
        msg,
        StdError::generic_err("Insufficient funds for all rewards").into(),
    );

    // valid amount
    let (vesting_amount, cliff_amount, cliff_time) = (1000, 250, 105);
    let msg =
        create_msg(100, 110, vesting_amount, Some(cliff_amount), cliff_time);

    let res =
        execute(deps.as_mut(), env.clone(), mock_info("addr0000", &[]), msg)?;

    assert_eq!(
        res.attributes,
        vec![
            Attribute {
                key: "action".to_string(),
                value: "register_vesting_account".to_string()
            },
            Attribute {
                key: "master_address".to_string(),
                value: "".to_string(),
            },
            Attribute {
                key: "address".to_string(),
                value: "addr0001".to_string()
            },
            Attribute {
                key: "vesting_amount".to_string(),
                value: "1000".to_string()
            },
            Attribute {
                key: "method".to_string(),
                value: "reward_users".to_string()
            }
        ]
    );

    // valid amount - one failed because duplicate
    let vesting_amount = 500u128;
    let cliff_amount = 250u128;
    let cliff_time = 105u64;

    let msg = ExecuteMsg::RewardUsers {
        master_address: None,
        rewards: vec![
            RewardUserRequest {
                user_address: "addr0002".to_string(),
                vesting_amount: Uint128::new(vesting_amount),
                cliff_amount: Some(Uint128::new(cliff_amount)),
            },
            RewardUserRequest {
                user_address: "addr0002".to_string(),
                vesting_amount: Uint128::new(vesting_amount),
                cliff_amount: Some(Uint128::new(cliff_amount)),
            },
        ],
        vesting_schedule: VestingSchedule::LinearVestingWithCliff {
            start_time: Uint64::new(100),
            end_time: Uint64::new(110),
            vesting_amount: Uint128::zero(),
            cliff_amount: Uint128::zero(),
            cliff_time: Uint64::new(cliff_time),
        },
    };

    let res =
        execute(deps.as_mut(), env.clone(), mock_info("addr0000", &[]), msg)?;

    assert_eq!(
        res.attributes,
        vec![
            Attribute {
                key: "action".to_string(),
                value: "register_vesting_account".to_string()
            },
            Attribute {
                key: "master_address".to_string(),
                value: "".to_string(),
            },
            Attribute {
                key: "address".to_string(),
                value: "addr0002".to_string()
            },
            Attribute {
                key: "vesting_amount".to_string(),
                value: "500".to_string()
            },
            Attribute {
                key: "method".to_string(),
                value: "reward_users".to_string()
            }
        ]
    );

    Ok(())
}

#[test]
fn test_withdraw() -> TestResult {
    let mut deps = mock_dependencies();
    let _res = instantiate(
        deps.as_mut(),
        mock_env(),
        mock_info("addr0000", &[coin(2000, "uusd")]),
        InstantiateMsg {
            admin: "addr0000".to_string(),
            managers: vec!["admin-sender".to_string()],
        },
    )?;

    let mut env = mock_env();
    env.block.time = Timestamp::from_seconds(100);

    let create_msg = |start_time: u64,
                      end_time: u64,
                      vesting_amount: u128,
                      cliff_amount: Option<u128>,
                      cliff_time: u64|
     -> ExecuteMsg {
        ExecuteMsg::RewardUsers {
            master_address: None,
            rewards: vec![RewardUserRequest {
                user_address: "addr0001".to_string(),
                vesting_amount: Uint128::new(vesting_amount),
                cliff_amount: cliff_amount.map(Uint128::new),
            }],
            vesting_schedule: VestingSchedule::LinearVestingWithCliff {
                start_time: Uint64::new(start_time),
                end_time: Uint64::new(end_time),
                vesting_amount: Uint128::zero(),
                cliff_amount: Uint128::zero(),
                cliff_time: Uint64::new(cliff_time),
            },
        }
    };

    // valid amount
    let (vesting_amount, cliff_amount, cliff_time) = (1000, 250, 105);
    let msg =
        create_msg(100, 110, vesting_amount, Some(cliff_amount), cliff_time);

    let _res =
        execute(deps.as_mut(), env.clone(), mock_info("addr0000", &[]), msg)?;

    // try to withdraw

    // unauthorized sender
    let msg = ExecuteMsg::Withdraw {
        recipient: "addr0000".to_string(),
        amount: Uint128::new(1000),
    };
    require_error(
        &mut deps,
        &env,
        mock_info("addr0042", &[]),
        msg,
        StdError::generic_err("Unauthorized").into(),
    );

    // withdraw more than unallocated
    let msg = ExecuteMsg::Withdraw {
        recipient: "addr0000".to_string(),
        amount: Uint128::new(1001),
    };
    let res =
        execute(deps.as_mut(), env.clone(), mock_info("addr0000", &[]), msg)?;

    assert_eq!(
        res.attributes,
        vec![
            Attribute {
                key: "action".to_string(),
                value: "withdraw".to_string()
            },
            Attribute {
                key: "recipient".to_string(),
                value: "addr0000".to_string()
            },
            Attribute {
                key: "amount".to_string(),
                value: "1000".to_string()
            },
            Attribute {
                key: "unallocated_amount".to_string(),
                value: "0".to_string()
            },
        ]
    );

    // withdraw but there's no more unallocated
    let msg = ExecuteMsg::Withdraw {
        recipient: "addr0000".to_string(),
        amount: Uint128::new(1),
    };
    require_error(
        &mut deps,
        &env,
        mock_info("addr0000", &[]),
        msg,
        StdError::generic_err("Nothing to withdraw").into(),
    );

    Ok(())
}

fn require_error(
    deps: &mut OwnedDeps<MockStorage, MockApi, MockQuerier>,
    env: &Env,
    info: MessageInfo,
    msg: ExecuteMsg,
    expected_error: ContractError,
) {
    let res = execute(deps.as_mut(), env.clone(), info, msg);
    match res {
        Err(err) => {
            assert_eq!(err, expected_error)
        }
        Ok(_) => panic!("Expected error but got success: {res:?}"),
    }
}

#[test]
fn register_vesting_account_with_native_token() -> TestResult {
    let mut deps = mock_dependencies();
    let _res = instantiate(
        deps.as_mut(),
        mock_env(),
        mock_info("addr0000", &[coin(1000, "uusd")]),
        InstantiateMsg {
            admin: "addr0000".to_string(),
            managers: vec!["admin-sender".to_string()],
        },
    )?;

    let mut env = mock_env();
    env.block.time = Timestamp::from_seconds(100);

    // zero amount vesting token
    let msg = ExecuteMsg::RewardUsers {
        master_address: None,
        rewards: vec![RewardUserRequest {
            user_address: "addr0001".to_string(),
            vesting_amount: Uint128::zero(),
            cliff_amount: None,
        }],
        vesting_schedule: VestingSchedule::LinearVesting {
            start_time: Uint64::new(100),
            end_time: Uint64::new(110),
            vesting_amount: Uint128::zero(),
        },
    };

    require_error(
        &mut deps,
        &env,
        mock_info("addr0000", &[Coin::new(0u128, "uusd")]),
        msg,
        ContractError::Vesting(VestingError::ZeroVestingAmount),
    );

    // too much vesting amount
    let msg = ExecuteMsg::RewardUsers {
        master_address: None,
        rewards: vec![RewardUserRequest {
            user_address: "addr0001".to_string(),
            vesting_amount: Uint128::new(1000001u128),
            cliff_amount: None,
        }],
        vesting_schedule: VestingSchedule::LinearVesting {
            start_time: Uint64::new(100),
            end_time: Uint64::new(110),
            vesting_amount: Uint128::new(1000000u128),
        },
    };
    let info = mock_info("addr0000", &[]);
    let res = execute(deps.as_mut(), env.clone(), info, msg.clone());
    match res {
        Err(ContractError::Std(StdError::GenericErr { msg, .. })) => {
            assert_eq!(msg, "Insufficient funds for all rewards")
        }
        _ => panic!("should not enter. got result: {res:?}"),
    }

    // too much vesting amount in 2 rewards
    let msg = ExecuteMsg::RewardUsers {
        master_address: None,
        rewards: vec![
            RewardUserRequest {
                user_address: "addr0001".to_string(),
                vesting_amount: Uint128::new(1000u128),
                cliff_amount: None,
            },
            RewardUserRequest {
                user_address: "addr0001".to_string(),
                vesting_amount: Uint128::new(1u128),
                cliff_amount: None,
            },
        ],
        vesting_schedule: VestingSchedule::LinearVesting {
            start_time: Uint64::new(100),
            end_time: Uint64::new(110),
            vesting_amount: Uint128::new(1000000u128),
        },
    };
    let info = mock_info("addr0000", &[]);
    let res = execute(deps.as_mut(), env.clone(), info, msg.clone());
    match res {
        Err(ContractError::Std(StdError::GenericErr { msg, .. })) => {
            assert_eq!(msg, "Insufficient funds for all rewards")
        }
        _ => panic!("should not enter. got result: {res:?}"),
    }

    // valid amount
    let msg = ExecuteMsg::RewardUsers {
        master_address: None,
        rewards: vec![RewardUserRequest {
            user_address: "addr0001".to_string(),
            vesting_amount: Uint128::new(100u128),
            cliff_amount: None,
        }],
        vesting_schedule: VestingSchedule::LinearVesting {
            start_time: Uint64::new(100),
            end_time: Uint64::new(110),
            vesting_amount: Uint128::new(100u128),
        },
    };
    let info = mock_info("addr0000", &[Coin::new(1000u128, "uusd")]);
    let res: Response = execute(deps.as_mut(), env.clone(), info, msg)?;
    assert_eq!(
        res.attributes,
        vec![
            Attribute {
                key: "action".to_string(),
                value: "register_vesting_account".to_string()
            },
            Attribute {
                key: "master_address".to_string(),
                value: "".to_string(),
            },
            Attribute {
                key: "address".to_string(),
                value: "addr0001".to_string()
            },
            Attribute {
                key: "vesting_amount".to_string(),
                value: "100".to_string()
            },
            Attribute {
                key: "method".to_string(),
                value: "reward_users".to_string()
            }
        ]
    );

    // query vesting account
    assert_eq!(
        from_json::<VestingAccountResponse>(&query(
            deps.as_ref(),
            env,
            QueryMsg::VestingAccount {
                address: "addr0001".to_string(),
            },
        )?)?,
        VestingAccountResponse {
            address: "addr0001".to_string(),
            vesting: VestingData {
                vesting_account: crate::state::VestingAccount {
                    master_address: None,
                    address: "addr0001".to_string(),
                    vesting_amount: Uint128::new(100u128),
                    vesting_schedule: VestingSchedule::LinearVesting {
                        start_time: Uint64::new(100),
                        end_time: Uint64::new(110),
                        vesting_amount: Uint128::new(100u128),
                    },
                    claimed_amount: Uint128::zero(),
                },
                vested_amount: Uint128::zero(),
                claimable_amount: Uint128::zero(),
            },
        }
    );
    Ok(())
}

#[test]
fn claim_native() -> TestResult {
    let mut deps = mock_dependencies();
    let _res = instantiate(
        deps.as_mut(),
        mock_env(),
        mock_info("addr0000", &[coin(1000000u128, "uusd")]),
        InstantiateMsg {
            admin: "addr0000".to_string(),
            managers: vec!["admin-sender".to_string()],
        },
    )?;

    // init env to time 100
    let mut env = mock_env();
    env.block.time = Timestamp::from_seconds(100);

    // valid amount
    let msg = ExecuteMsg::RewardUsers {
        master_address: None,
        rewards: vec![RewardUserRequest {
            user_address: "addr0001".to_string(),
            vesting_amount: Uint128::new(1000000u128),
            cliff_amount: None,
        }],
        vesting_schedule: VestingSchedule::LinearVesting {
            start_time: Uint64::new(100),
            end_time: Uint64::new(110),
            vesting_amount: Uint128::new(1000000u128),
        },
    };

    let info = mock_info("addr0000", &[Coin::new(1000000u128, "uusd")]);
    let _ = execute(deps.as_mut(), env.clone(), info, msg)?;

    // make time to half claimable
    env.block.time = Timestamp::from_seconds(105);

    // valid claim
    let info = mock_info("addr0001", &[]);
    let msg = ExecuteMsg::Claim { recipient: None };

    let res = execute(deps.as_mut(), env.clone(), info.clone(), msg.clone())?;
    assert_eq!(
        res.messages,
        vec![SubMsg::new(BankMsg::Send {
            to_address: "addr0001".to_string(),
            amount: vec![Coin {
                denom: "uusd".to_string(),
                amount: Uint128::new(500000u128),
            }],
        }),]
    );
    assert_eq!(
        res.attributes,
        vec![
            Attribute::new("action", "claim"),
            Attribute::new("address", "addr0001"),
            Attribute::new("vesting_amount", "1000000"),
            Attribute::new("vested_amount", "500000"),
            Attribute::new("claim_amount", "500000"),
        ],
    );

    // query vesting account
    assert_eq!(
        from_json::<VestingAccountResponse>(&query(
            deps.as_ref(),
            env.clone(),
            QueryMsg::VestingAccount {
                address: "addr0001".to_string(),
            },
        )?)?,
        VestingAccountResponse {
            address: "addr0001".to_string(),
            vesting: VestingData {
                vesting_account: crate::state::VestingAccount {
                    master_address: None,
                    address: "addr0001".to_string(),
                    vesting_amount: Uint128::new(1000000),
                    vesting_schedule: VestingSchedule::LinearVesting {
                        start_time: Uint64::new(100),
                        end_time: Uint64::new(110),
                        vesting_amount: Uint128::new(1000000u128),
                    },
                    claimed_amount: Uint128::new(500000),
                },
                vested_amount: Uint128::new(500000),
                claimable_amount: Uint128::zero(),
            },
        }
    );

    // make time to half claimable
    env.block.time = Timestamp::from_seconds(110);

    let res = execute(deps.as_mut(), env.clone(), info, msg)?;
    assert_eq!(
        res.messages,
        vec![SubMsg::new(BankMsg::Send {
            to_address: "addr0001".to_string(),
            amount: vec![Coin {
                denom: "uusd".to_string(),
                amount: Uint128::new(500000u128),
            }],
        }),]
    );
    assert_eq!(
        res.attributes,
        vec![
            Attribute::new("action", "claim"),
            Attribute::new("address", "addr0001"),
            Attribute::new("vesting_amount", "1000000"),
            Attribute::new("vested_amount", "1000000"),
            Attribute::new("claim_amount", "500000"),
        ],
    );

    // query vesting account
    let res = &query(
        deps.as_ref(),
        env,
        QueryMsg::VestingAccount {
            address: "addr0001".to_string(),
        },
    );
    //expect res to be an errro
    match res {
        Err(StdError::NotFound { .. }) => {}
        _ => panic!("should not enter. got result: {res:?}"),
    }

    Ok(())
}
