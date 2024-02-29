use crate::contract::{execute, instantiate, query};
use crate::errors::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, StakeMsg, UnstakeMsg};
use crate::state::Whitelist;

use cosmwasm_std::{coin, to_json_binary, CosmosMsg, StakingMsg};
use cosmwasm_std::{
    from_json,
    testing::{mock_dependencies, mock_env, mock_info},
    StdError, Uint128,
};
pub type TestResult = Result<(), anyhow::Error>;

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
fn test_execute_permission() -> TestResult {
    let mut deps = mock_dependencies();
    let _res = instantiate(
        deps.as_mut(),
        mock_env(),
        mock_info("addr0000", &[]),
        InstantiateMsg {
            admin: "admin".to_string(),
            managers: vec!["manager1".to_string(), "manager2".to_string()],
        },
    )?;

    // assert that compounder is off
    let res = query(deps.as_ref(), mock_env(), QueryMsg::AutocompounderMode {})?;
    assert_eq!(res, to_json_binary(&false)?);

    // assert we can't stake
    match execute(
        deps.as_mut(),
        mock_env(),
        mock_info("manager1", &[]),
        ExecuteMsg::Stake {
            amount: Uint128::new(100),
            stake_msgs: vec![],
        },
    ) {
        Err(ContractError::Unauthorized {}) => {}
        _ => panic!("Unexpected error"),
    }

    // set compounder on
    let _res = execute(
        deps.as_mut(),
        mock_env(),
        mock_info("admin", &[]),
        ExecuteMsg::SetAutocompounderMode {
            autocompounder_mode: true,
        },
    )?;

    // assert that compounder is on
    let res = query(deps.as_ref(), mock_env(), QueryMsg::AutocompounderMode {})?;
    assert_eq!(res, to_json_binary(&true)?);

    // assert we can stake through valid manager
    let _res = execute(
        deps.as_mut(),
        mock_env(),
        mock_info("manager1", &[]),
        ExecuteMsg::Stake {
            amount: Uint128::new(100),
            stake_msgs: vec![StakeMsg {
                share: Uint128::new(100),
                validator: "validator".to_string(),
            }],
        },
    )?;

    // assert we can't do that through invalid manager
    match execute(
        deps.as_mut(),
        mock_env(),
        mock_info("manager3", &[]),
        ExecuteMsg::Stake {
            amount: Uint128::new(100),
            stake_msgs: vec![StakeMsg {
                share: Uint128::new(100),
                validator: "validator".to_string(),
            }],
        },
    ) {
        Err(ContractError::Unauthorized {}) => {}
        _ => panic!("Unexpected error"),
    }

    // assert manager can't update managers
    match execute(
        deps.as_mut(),
        mock_env(),
        mock_info("manager1", &[]),
        ExecuteMsg::UpdateManagers {
            managers: vec![
                "manager1".to_string(),
                "manager2".to_string(),
                "manager3".to_string(),
            ],
        },
    ) {
        Err(ContractError::Unauthorized {}) => {}
        _ => panic!("Unexpected error"),
    }

    // assert non manager can't update managers
    match execute(
        deps.as_mut(),
        mock_env(),
        mock_info("addr0000", &[]),
        ExecuteMsg::UpdateManagers {
            managers: vec![
                "manager1".to_string(),
                "manager2".to_string(),
                "manager3".to_string(),
            ],
        },
    ) {
        Err(ContractError::Unauthorized {}) => {}
        _ => panic!("Unexpected error"),
    }

    // assert admin can update managers
    let _res = execute(
        deps.as_mut(),
        mock_env(),
        mock_info("admin", &[]),
        ExecuteMsg::UpdateManagers {
            managers: vec![
                "manager1".to_string(),
                "manager2".to_string(),
                "manager42".to_string(),
            ],
        },
    )?;

    // check managers was updated correctly
    let res = query(deps.as_ref(), mock_env(), QueryMsg::AdminAndManagers {})?;
    let whitelist: Whitelist = from_json(res)?;
    assert!(whitelist.is_manager("manager42".to_string()));

    // assert manager can't set compounder mode
    match execute(
        deps.as_mut(),
        mock_env(),
        mock_info("manager1", &[]),
        ExecuteMsg::SetAutocompounderMode {
            autocompounder_mode: false,
        },
    ) {
        Err(ContractError::Unauthorized {}) => {}
        _ => panic!("Unexpected error"),
    }

    // assert non manager can't set compounder mode
    match execute(
        deps.as_mut(),
        mock_env(),
        mock_info("addr0000", &[]),
        ExecuteMsg::SetAutocompounderMode {
            autocompounder_mode: false,
        },
    ) {
        Err(ContractError::Unauthorized {}) => {}
        _ => panic!("Unexpected error"),
    }

    // assert manager can't unstake
    match execute(
        deps.as_mut(),
        mock_env(),
        mock_info("manager1", &[]),
        ExecuteMsg::Unstake {
            unstake_msgs: vec![],
        },
    ) {
        Err(ContractError::Unauthorized {}) => {}
        _ => panic!("Unexpected error"),
    }

    // assert non manager can't unstake
    match execute(
        deps.as_mut(),
        mock_env(),
        mock_info("addr0000", &[]),
        ExecuteMsg::Unstake {
            unstake_msgs: vec![],
        },
    ) {
        Err(ContractError::Unauthorized {}) => {}
        _ => panic!("Unexpected error"),
    }

    // assert managers can't withdraw
    match execute(
        deps.as_mut(),
        mock_env(),
        mock_info("manager1", &[]),
        ExecuteMsg::Withdraw {
            amount: Uint128::new(100),
            recipient: "addr0000".to_string(),
        },
    ) {
        Err(ContractError::Unauthorized {}) => {}
        _ => panic!("Unexpected error"),
    }

    // assert non managers can't withdraw
    match execute(
        deps.as_mut(),
        mock_env(),
        mock_info("addr0000", &[]),
        ExecuteMsg::Withdraw {
            amount: Uint128::new(100),
            recipient: "addr0000".to_string(),
        },
    ) {
        Err(ContractError::Unauthorized {}) => {}
        _ => panic!("Unexpected error"),
    }

    Ok(())
}

#[test]
fn test_share_calc() -> TestResult {
    let mut deps = mock_dependencies();
    let _res = instantiate(
        deps.as_mut(),
        mock_env(),
        mock_info("addr0000", &[]),
        InstantiateMsg {
            admin: "admin".to_string(),
            managers: vec!["manager1".to_string(), "manager2".to_string()],
        },
    )?;

    // toggle it on
    let _res = execute(
        deps.as_mut(),
        mock_env(),
        mock_info("admin", &[]),
        ExecuteMsg::SetAutocompounderMode {
            autocompounder_mode: true,
        },
    )?;

    // stake 100 tokens to 2 validators
    let res = execute(
        deps.as_mut(),
        mock_env(),
        mock_info("manager1", &[]),
        ExecuteMsg::Stake {
            amount: Uint128::new(100),
            stake_msgs: vec![
                StakeMsg {
                    share: Uint128::new(100),
                    validator: "validator1".to_string(),
                },
                StakeMsg {
                    share: Uint128::new(100),
                    validator: "validator2".to_string(),
                },
            ],
        },
    )?;

    for message in res.messages {
        match message.msg {
            CosmosMsg::Staking(StakingMsg::Delegate { validator, amount }) => {
                if validator == "validator1" {
                    assert_eq!(amount, coin(50, "unibi"));
                } else if validator == "validator2" {
                    assert_eq!(amount, coin(50, "unibi"));
                } else {
                    panic!("Unexpected recipient: {validator}");
                }
            }
            _ => panic!("Unexpected message"),
        }
    }

    // stake 100 tokens to 2 validators different shares
    let res = execute(
        deps.as_mut(),
        mock_env(),
        mock_info("manager1", &[]),
        ExecuteMsg::Stake {
            amount: Uint128::new(100),
            stake_msgs: vec![
                StakeMsg {
                    share: Uint128::new(1),
                    validator: "validator1".to_string(),
                },
                StakeMsg {
                    share: Uint128::new(999),
                    validator: "validator2".to_string(),
                },
            ],
        },
    )?;

    for message in res.messages {
        match message.msg {
            CosmosMsg::Staking(StakingMsg::Delegate { validator, amount }) => {
                if validator == "validator2" {
                    assert_eq!(amount, coin(99, "unibi"));
                } else {
                    panic!("Unexpected recipient: {validator} - {amount}");
                }
            }
            _ => panic!("Unexpected message"),
        }
    }

    // stake 100 tokens to 3 validators different shares
    let res = execute(
        deps.as_mut(),
        mock_env(),
        mock_info("manager1", &[]),
        ExecuteMsg::Stake {
            amount: Uint128::new(100),
            stake_msgs: vec![
                StakeMsg {
                    share: Uint128::new(100),
                    validator: "validator1".to_string(),
                },
                StakeMsg {
                    share: Uint128::new(100),
                    validator: "validator2".to_string(),
                },
                StakeMsg {
                    share: Uint128::new(100),
                    validator: "validator3".to_string(),
                },
            ],
        },
    )?;

    for message in res.messages {
        match message.msg {
            CosmosMsg::Staking(StakingMsg::Delegate { validator, amount }) => {
                if validator == "validator1" {
                    assert_eq!(amount, coin(33, "unibi"));
                } else if validator == "validator2" {
                    assert_eq!(amount, coin(33, "unibi"));
                } else if validator == "validator3" {
                    assert_eq!(amount, coin(33, "unibi"));
                } else {
                    panic!("Unexpected recipient: {validator}");
                }
            }
            _ => panic!("Unexpected message"),
        }
    }

    Ok(())
}

#[test]
fn test_withdraw() -> TestResult {
    let mut deps = mock_dependencies();
    let _res = instantiate(
        deps.as_mut(),
        mock_env(),
        mock_info("addr0000", &[]),
        InstantiateMsg {
            admin: "admin".to_string(),
            managers: vec!["manager1".to_string(), "manager2".to_string()],
        },
    )?;

    // withdraw
    let _res = execute(
        deps.as_mut(),
        mock_env(),
        mock_info("admin", &[]),
        ExecuteMsg::Withdraw {
            amount: Uint128::new(100),
            recipient: "addr0000".to_string(),
        },
    )?;

    Ok(())
}

#[test]
fn test_unstake() -> TestResult {
    let mut deps = mock_dependencies();
    let _res = instantiate(
        deps.as_mut(),
        mock_env(),
        mock_info("addr0000", &[]),
        InstantiateMsg {
            admin: "admin".to_string(),
            managers: vec!["manager1".to_string(), "manager2".to_string()],
        },
    )?;

    // unstake
    let res = execute(
        deps.as_mut(),
        mock_env(),
        mock_info("admin", &[]),
        ExecuteMsg::Unstake {
            unstake_msgs: vec![
                UnstakeMsg {
                    amount: Uint128::new(100),
                    validator: "validator".to_string(),
                },
                UnstakeMsg {
                    amount: Uint128::new(100),
                    validator: "validator2".to_string(),
                },
                UnstakeMsg {
                    amount: Uint128::new(0),
                    validator: "validator3".to_string(),
                },
            ],
        },
    )?;

    for message in res.messages {
        match message.msg {
            CosmosMsg::Staking(StakingMsg::Undelegate { validator, amount }) => {
                if validator == "validator" {
                    assert_eq!(amount, coin(100, "unibi"));
                } else if validator == "validator2" {
                    assert_eq!(amount, coin(100, "unibi"));
                } else {
                    panic!("Unexpected recipient: {validator}");
                }
            }
            _ => panic!("Unexpected message"),
        }
    }

    Ok(())
}
