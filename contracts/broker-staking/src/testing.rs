use std::collections::BTreeSet;

use crate::contract::{execute, query};
use crate::msg::{ExecuteMsg, StakeMsg, UnstakeMsg};
use cosmwasm_std::{self as cw_std};
use cw_std::{
    coin, from_json, testing, BankMsg, Coin, CosmosMsg, Response, StakingMsg,
    Uint128,
};
use nibiru_std::errors::TestResult;
use serde::Serialize;

use broker_bank::{
    msgs::{PermsStatus, QueryMsg},
    oper_perms::{self, Permissions},
    state::{IS_HALTED, OPERATORS},
    tutil::{
        self, mock_info_for_sender, setup_contract, setup_contract_defaults,
        TEST_OWNER,
    },
};

struct TestCaseExec<'a> {
    to_addrs: Vec<String>,
    opers: Vec<String>,
    exec_msg: ExecuteMsg,
    sender: &'a str,
    err: Option<&'a str>,
    contract_funds_start: Option<Vec<Coin>>,
    resp_msgs: Vec<CosmosMsg>,
}

/// Test that all owner-gated execute calls fail when the tx sender is not
/// the smart contract owner.
#[test]
pub fn test_assert_owner() -> TestResult {
    let not_owner = "not-owner";
    let want_err: Option<&str> = Some("not the contract's current owner");

    let to_addrs: [String; 2] = ["mm_kucoin", "mm_bybit"].map(|s| s.to_string());
    let opers: [String; 1] = ["valid_oper"].map(|s| s.to_string());

    let test_cases: Vec<TestCaseExec> = vec![
        TestCaseExec {
            to_addrs: to_addrs.to_vec(),
            opers: opers.to_vec(),
            sender: not_owner,
            exec_msg: ExecuteMsg::EditOpers(oper_perms::Action::AddOper {
                address: String::from("new_oper"),
            }),
            err: want_err,
            contract_funds_start: None,
            resp_msgs: vec![],
        },
        TestCaseExec {
            to_addrs: to_addrs.to_vec(),
            opers: opers.to_vec(),
            sender: not_owner,
            exec_msg: ExecuteMsg::UpdateOwnership(
                nibiru_ownable::Action::TransferOwnership {
                    new_owner: String::from("new_owner"),
                    expiry: None,
                },
            ),
            err: want_err,
            contract_funds_start: None,
            resp_msgs: vec![],
        },
        TestCaseExec {
            to_addrs: to_addrs.to_vec(),
            opers: opers.to_vec(),
            sender: not_owner,
            exec_msg: ExecuteMsg::Withdraw {
                to: Some(String::from("mm_bybit")),
                denoms: vec![].into_iter().collect(),
            },
            err: want_err,
            contract_funds_start: None,
            resp_msgs: vec![],
        },
        TestCaseExec {
            to_addrs: to_addrs.to_vec(),
            opers: opers.to_vec(),
            sender: not_owner,
            exec_msg: ExecuteMsg::ToggleHalt {},
            err: want_err,
            contract_funds_start: None,
            resp_msgs: vec![],
        },
        TestCaseExec {
            to_addrs: to_addrs.to_vec(),
            opers: opers.to_vec(),
            sender: not_owner,
            exec_msg: ExecuteMsg::WithdrawAll {
                to: Some(String::from("mm_bybit")),
            },
            err: want_err,
            contract_funds_start: None,
            resp_msgs: vec![],
        },
    ];

    for tc in &test_cases {
        let to_addrs = &tc.to_addrs;
        let opers = &tc.opers;
        // instantiate smart contract from the owner
        let (mut deps, env, _info) =
            setup_contract(to_addrs.clone(), opers.clone())?;

        // send the exec msg and it should fail.
        let info = mock_info_for_sender(tc.sender);
        let res = execute(deps.as_mut(), env, info, tc.exec_msg.clone());
        assert!(res.is_err());
        let err = res.expect_err("err should be defined");
        let is_contained = err
            .to_string()
            .contains(tc.err.expect("errors should occur in this test"));
        assert!(is_contained, "got error {}", err);
    }
    Ok(())
}

#[test]
fn exec_withdraw() -> TestResult {
    let to_addrs: [String; 2] = ["mm_kucoin", "mm_bybit"].map(|s| s.to_string());
    let opers: [String; 1] = ["valid_oper"].map(|s| s.to_string());
    let test_cases: Vec<TestCaseExec> = vec![
        // WithdrawAll
        TestCaseExec {
            to_addrs: to_addrs.to_vec(),
            opers: opers.to_vec(),
            sender: TEST_OWNER,
            exec_msg: ExecuteMsg::WithdrawAll {
                to: Some(String::from("mm_bybit")),
            },
            err: None,
            contract_funds_start: None,
            resp_msgs: vec![BankMsg::Send {
                to_address: String::from("mm_bybit"),
                amount: vec![],
            }
            .into()],
        },
        // WithdrawAll / Nonzero amount
        TestCaseExec {
            to_addrs: to_addrs.to_vec(),
            opers: opers.to_vec(),
            sender: TEST_OWNER,
            exec_msg: ExecuteMsg::WithdrawAll {
                to: Some(String::from("to_addr")),
            },
            err: None,
            contract_funds_start: Some(vec![Coin {
                denom: "unibi".into(),
                amount: Uint128::from(420u128),
            }]),
            resp_msgs: vec![BankMsg::Send {
                to_address: String::from("to_addr"),
                amount: vec![Coin {
                    denom: "unibi".into(),
                    amount: Uint128::from(420u128),
                }],
            }
            .into()],
        },
        // Withdraw / Nonzero amount
        TestCaseExec {
            to_addrs: to_addrs.to_vec(),
            opers: opers.to_vec(),
            sender: TEST_OWNER,
            exec_msg: ExecuteMsg::Withdraw {
                to: Some(String::from("to_addr")),
                denoms: ["uusd"].iter().map(|str| String::from(*str)).collect(),
            },
            err: None,
            contract_funds_start: Some(vec![
                Coin {
                    denom: "unibi".into(),
                    amount: Uint128::from(420u128),
                },
                Coin {
                    denom: "uusd".into(),
                    amount: Uint128::from(69u128),
                },
            ]),
            resp_msgs: vec![BankMsg::Send {
                to_address: String::from("to_addr"),
                amount: vec![Coin {
                    denom: "uusd".into(),
                    amount: Uint128::from(69u128),
                }],
            }
            .into()],
        },
    ];
    for tc in &test_cases {
        let to_addrs = &tc.to_addrs;
        let opers = &tc.opers;
        // instantiate smart contract from the owner
        let (mut deps, env, _info) =
            setup_contract(to_addrs.clone(), opers.clone())?;

        if let Some(funds_start) = &tc.contract_funds_start {
            // Set up a mock querier with contract balance
            let contract_addr = env.contract.address.to_string();
            let balances: &[(&str, &[Coin])] =
                &[(contract_addr.as_str(), funds_start.as_slice())];
            let querier = testing::MockQuerier::new(balances);
            deps.querier = querier;
        }

        // send the exec msg
        let info = mock_info_for_sender(tc.sender);
        let res = execute(deps.as_mut(), env, info, tc.exec_msg.clone());
        if let Some(want_err) = tc.err {
            let got_err = res.expect_err("errors should occur in this test");
            let is_contained = got_err.to_string().contains(want_err);
            assert!(is_contained, "got error {}", got_err);
            return Ok(());
        }
        assert!(res.is_ok());

        let resp = res?;
        let got_resp_msgs: Vec<CosmosMsgExt> = resp
            .messages
            .iter()
            .map(|sub_msg| CosmosMsgExt(&sub_msg.msg))
            .collect();
        let want_resp_msgs: Vec<CosmosMsgExt> =
            tc.resp_msgs.iter().map(CosmosMsgExt).collect();
        assert_eq!(want_resp_msgs, got_resp_msgs);
    }
    Ok(())
}

#[derive(Debug, Serialize)]
struct CosmosMsgExt<'a>(&'a CosmosMsg);

impl<'a> PartialEq for CosmosMsgExt<'a> {
    fn eq(&self, other: &Self) -> bool {
        let err_msg = "cosmos msg should be jsonable";
        let self_str = serde_json::to_string_pretty(self).expect(err_msg);
        let other_str = serde_json::to_string_pretty(other).expect(err_msg);
        self_str.eq(&other_str)
    }
}

impl<'a> std::fmt::Display for CosmosMsgExt<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(&self.0).unwrap())
    }
}

#[test]
pub fn exec_toggle_halt() -> TestResult {
    let (mut deps, env, _info) = setup_contract_defaults()?;

    let query_msg = QueryMsg::Perms {};
    let resp: PermsStatus =
        from_json(query(deps.as_ref(), env.clone(), query_msg.clone())?)?;

    let want_is_halted = false;
    assert_eq!(resp.is_halted, want_is_halted);
    assert_eq!(
        resp.perms,
        Permissions {
            owner: Some(String::from(TEST_OWNER)),
            operators: ["oper0", "oper1"]
                .into_iter()
                .map(String::from)
                .collect(),
        }
    );

    // ToggleHalt : error case
    let exec_msg = ExecuteMsg::ToggleHalt {};
    let sender = "not_owner";
    let info = mock_info_for_sender(sender);
    let exec_resp = execute(deps.as_mut(), env.clone(), info, exec_msg.clone());
    assert!(exec_resp.is_err(), "got {exec_resp:?}");
    let resp: PermsStatus =
        from_json(query(deps.as_ref(), env.clone(), query_msg.clone())?)?;
    assert_eq!(resp.is_halted, want_is_halted);

    // ToggleHalt : success case
    let sender = TEST_OWNER;
    let mut want_is_halted = true;
    let info = mock_info_for_sender(sender);
    let _exec_resp =
        execute(deps.as_mut(), env.clone(), info.clone(), exec_msg.clone())?;
    let resp: PermsStatus =
        from_json(query(deps.as_ref(), env.clone(), query_msg.clone())?)?;
    assert_eq!(resp.is_halted, want_is_halted);

    want_is_halted = false;
    let _exec_resp =
        execute(deps.as_mut(), env.clone(), info, exec_msg.clone())?;
    let resp: PermsStatus =
        from_json(query(deps.as_ref(), env.clone(), query_msg.clone())?)?;
    assert_eq!(resp.is_halted, want_is_halted);

    Ok(())
}

// TODO: test ExecuteMsg::EditOpers
// TODO: ownership query
// pub fn get_ownership(storage: &dyn Storage) -> StdResult<Ownership<Addr>>

#[test]
fn exec_edit_opers_add() -> TestResult {
    let (mut deps, _env, _info) = tutil::setup_contract_defaults()?;
    let new_member = "new_member";
    let perms = Permissions::load(&deps.storage)?;
    let not_has: bool = !perms.is_owner(new_member);
    assert!(not_has);

    // Add an operator to the permission set
    let execute_msg = ExecuteMsg::EditOpers(oper_perms::Action::AddOper {
        address: new_member.to_string(),
    });
    let sender = tutil::TEST_OWNER;
    let execute_info = testing::mock_info(sender, &[]);

    let check_resp = |resp: Response| {
        assert_eq!(resp.messages.len(), 0, "resp.messages: {:?}", resp.messages);
        assert_eq!(
            resp.attributes.len(),
            2,
            "resp.attributes: {:#?}",
            resp.attributes
        );
    };

    let result = execute(
        deps.as_mut(),
        testing::mock_env(),
        execute_info,
        execute_msg,
    )?;
    check_resp(result);

    // Check correctness of the result
    let perms = Permissions::load(&deps.storage)?;
    let has: bool = perms.has(new_member);
    assert!(has);

    let query_req = QueryMsg::Perms {};
    let binary = query(deps.as_ref(), testing::mock_env(), query_req)?;
    let response: PermsStatus = cosmwasm_std::from_json(binary)?;
    assert!(response.perms.has(sender));
    Ok(())
}

#[test]
fn exec_edit_opers_remove() -> TestResult {
    let to_addrs = vec![];
    let opers = vec![];
    let (mut deps, _env, _info) = tutil::setup_contract(to_addrs, opers)?;
    // Set up initial perms
    let opers_start: Vec<String> = ["vitalik", "musk", "satoshi"]
        .iter()
        .map(|&s| s.to_string())
        .collect();
    let mut perms = Permissions::load(&deps.storage)?;
    assert_eq!(perms.operators.len(), 0); // admin remains
    for member in opers_start.iter() {
        perms.operators.insert(member.clone());
    }
    let res = OPERATORS.save(deps.as_mut().storage, &perms.operators);
    assert!(res.is_ok());

    // Remove a member from the whitelist
    let execute_msg = ExecuteMsg::EditOpers(oper_perms::Action::RemoveOper {
        address: "satoshi".to_string(),
    });
    let sender = tutil::TEST_OWNER;
    let execute_info = testing::mock_info(sender, &[]);
    let check_resp = |resp: Response| {
        assert_eq!(resp.messages.len(), 0, "resp.messages: {:?}", resp.messages);
        assert_eq!(
            resp.attributes.len(),
            2,
            "resp.attributes: {:#?}",
            resp.attributes
        );
    };
    let result = execute(
        deps.as_mut(),
        testing::mock_env(),
        execute_info,
        execute_msg,
    )?;
    check_resp(result);

    // Check correctness of the result
    let query_req = QueryMsg::Perms {};
    let binary = query(deps.as_ref(), testing::mock_env(), query_req)?;
    let response: PermsStatus = cosmwasm_std::from_json(binary)?;
    let expected_opers: BTreeSet<String> =
        ["vitalik", "musk"].iter().map(|&s| s.to_string()).collect();
    assert_eq!(
        response.perms.operators, expected_opers,
        "got: {:#?}, wanted: {:#?}",
        response.perms.operators, expected_opers
    );
    Ok(())
}

#[test]
fn exec_stake_halted() -> TestResult {
    let to_addrs: [String; 2] = ["mm_kucoin", "mm_bybit"].map(|s| s.to_string());
    let opers: [String; 1] = ["valid_oper"].map(|s| s.to_string());
    let (mut deps, env, _info) =
        setup_contract(to_addrs.to_vec(), opers.to_vec())?;

    // Set is_halted to false
    IS_HALTED.save(deps.as_mut().storage, &false)?;

    // Success case: valid operator sends coins to an allowed address
    let valid_exec_msg = ExecuteMsg::Stake {
        stake_msgs: vec![StakeMsg {
            share: Uint128::new(100),
            validator: String::from("mm_bybit"),
        }],
        amount: Uint128::new(50),
    };
    let sender = "valid_oper";
    let info = mock_info_for_sender(sender);
    let res = execute(
        deps.as_mut(),
        env.clone(),
        info.clone(),
        valid_exec_msg.clone(),
    )?;

    for message in res.messages {
        match message.msg {
            CosmosMsg::Staking(StakingMsg::Delegate { validator, amount }) => {
                assert_eq!(validator, "mm_bybit");
                assert_eq!(amount, coin(50, "unibi"));
            }
            _ => panic!("unexpected message: {:?}", message),
        }
    }

    // Error case: Halted operations
    IS_HALTED.save(deps.as_mut().storage, &true)?;
    let sender = "valid_oper";
    let info = mock_info_for_sender(sender);
    let res = execute(deps.as_mut(), env.clone(), info, valid_exec_msg.clone());
    assert!(res.is_err());

    Ok(())
}

#[test]
fn exec_stake() -> TestResult {
    let to_addrs: [String; 2] = ["mm_kucoin", "mm_bybit"].map(|s| s.to_string());
    let opers: [String; 1] = ["valid_oper"].map(|s| s.to_string());
    let test_cases: Vec<TestCaseExec> = vec![
        // Success
        TestCaseExec {
            to_addrs: to_addrs.to_vec(),
            opers: opers.to_vec(),
            sender: "valid_oper",
            exec_msg: ExecuteMsg::Stake {
                stake_msgs: vec![StakeMsg {
                    share: Uint128::new(100),
                    validator: String::from("mm_bybit"),
                }],
                amount: Uint128::new(50),
            },
            err: None,
            contract_funds_start: None,
            resp_msgs: vec![CosmosMsg::Staking(StakingMsg::Delegate {
                validator: String::from("mm_bybit"),
                amount: coin(50, "unibi"),
            })],
        },
        // Success : valid operation to multiple stakers
        TestCaseExec {
            to_addrs: to_addrs.to_vec(),
            opers: opers.to_vec(),
            sender: "valid_oper",
            exec_msg: ExecuteMsg::Stake {
                stake_msgs: vec![
                    StakeMsg {
                        share: Uint128::new(100),
                        validator: String::from("mm_bybit"),
                    },
                    StakeMsg {
                        share: Uint128::new(100),
                        validator: String::from("mm_kucoin"),
                    },
                ],
                amount: Uint128::new(50),
            },
            err: None,
            contract_funds_start: None,
            resp_msgs: vec![
                CosmosMsg::Staking(StakingMsg::Delegate {
                    validator: String::from("mm_bybit"),
                    amount: coin(25, "unibi"),
                }),
                CosmosMsg::Staking(StakingMsg::Delegate {
                    validator: String::from("mm_kucoin"),
                    amount: coin(25, "unibi"),
                }),
            ],
        },
        // Success : valid operation to multiple stakers - different shares
        TestCaseExec {
            to_addrs: to_addrs.to_vec(),
            opers: opers.to_vec(),
            sender: "valid_oper",
            exec_msg: ExecuteMsg::Stake {
                stake_msgs: vec![
                    StakeMsg {
                        share: Uint128::new(100),
                        validator: String::from("mm_bybit"),
                    },
                    StakeMsg {
                        share: Uint128::new(150),
                        validator: String::from("mm_kucoin"),
                    },
                ],
                amount: Uint128::new(50),
            },
            err: None,
            contract_funds_start: None,
            resp_msgs: vec![
                CosmosMsg::Staking(StakingMsg::Delegate {
                    validator: String::from("mm_bybit"),
                    amount: coin(20, "unibi"),
                }),
                CosmosMsg::Staking(StakingMsg::Delegate {
                    validator: String::from("mm_kucoin"),
                    amount: coin(30, "unibi"),
                }),
            ],
        },
        // Fail - invalid sender
        TestCaseExec {
            to_addrs: to_addrs.to_vec(),
            opers: opers.to_vec(),
            sender: "invalid_oper",
            exec_msg: ExecuteMsg::Stake {
                stake_msgs: vec![StakeMsg {
                    share: Uint128::new(100),
                    validator: String::from("mm_bybit"),
                }],
                amount: Uint128::new(50),
            },
            err: Some(
                "insufficient permissions: address is not a contract operator",
            ),
            contract_funds_start: None,
            resp_msgs: vec![CosmosMsg::Staking(StakingMsg::Delegate {
                validator: String::from("mm_bybit"),
                amount: coin(50, "unibi"),
            })],
        },
        // Fail - empty stake messages
        TestCaseExec {
            to_addrs: to_addrs.to_vec(),
            opers: opers.to_vec(),
            sender: "valid_oper",
            exec_msg: ExecuteMsg::Stake {
                stake_msgs: vec![],
                amount: Uint128::new(50),
            },
            err: Some("total shares cannot be zero"),
            contract_funds_start: None,
            resp_msgs: vec![CosmosMsg::Staking(StakingMsg::Delegate {
                validator: String::from("mm_bybit"),
                amount: coin(50, "unibi"),
            })],
        },
    ];
    for tc in &test_cases {
        let to_addrs = &tc.to_addrs;
        let opers = &tc.opers;
        // instantiate smart contract from the owner
        let (mut deps, env, _info) =
            setup_contract(to_addrs.clone(), opers.clone())?;

        if let Some(funds_start) = &tc.contract_funds_start {
            // Set up a mock querier with contract balance
            let contract_addr = env.contract.address.to_string();
            let balances: &[(&str, &[Coin])] =
                &[(contract_addr.as_str(), funds_start.as_slice())];
            let querier = testing::MockQuerier::new(balances);
            deps.querier = querier;
        }

        // send the exec msg
        let info = mock_info_for_sender(tc.sender);
        let res = execute(deps.as_mut(), env, info, tc.exec_msg.clone());

        if let Some(want_err) = tc.err {
            let got_err = res.expect_err("errors should occur in this test");
            let is_contained = got_err.to_string().contains(want_err);
            assert!(is_contained, "got error {}", got_err);
            return Ok(());
        }
        assert!(res.is_ok());

        let resp = res?;
        let got_resp_msgs: Vec<CosmosMsgExt> = resp
            .messages
            .iter()
            .map(|sub_msg| CosmosMsgExt(&sub_msg.msg))
            .collect();
        let want_resp_msgs: Vec<CosmosMsgExt> =
            tc.resp_msgs.iter().map(CosmosMsgExt).collect();
        assert_eq!(want_resp_msgs, got_resp_msgs);
    }
    Ok(())
}

#[test]
fn exec_unstake() -> TestResult {
    let to_addrs: [String; 2] = ["mm_kucoin", "mm_bybit"].map(|s| s.to_string());
    let opers: [String; 1] = ["valid_oper"].map(|s| s.to_string());
    let test_cases: Vec<TestCaseExec> = vec![
        // Success
        TestCaseExec {
            to_addrs: to_addrs.to_vec(),
            opers: opers.to_vec(),
            sender: "owner",
            exec_msg: ExecuteMsg::Unstake {
                unstake_msgs: vec![UnstakeMsg {
                    amount: Uint128::new(100),
                    validator: String::from("mm_bybit"),
                }],
            },
            err: None,
            contract_funds_start: None,
            resp_msgs: vec![CosmosMsg::Staking(StakingMsg::Undelegate {
                validator: String::from("mm_bybit"),
                amount: coin(100, "unibi"),
            })],
        },
        // Success - multiple
        TestCaseExec {
            to_addrs: to_addrs.to_vec(),
            opers: opers.to_vec(),
            sender: "owner",
            exec_msg: ExecuteMsg::Unstake {
                unstake_msgs: vec![
                    UnstakeMsg {
                        amount: Uint128::new(100),
                        validator: String::from("mm_bybit"),
                    },
                    UnstakeMsg {
                        amount: Uint128::new(20),
                        validator: String::from("mm_kucoin"),
                    },
                    UnstakeMsg {
                        amount: Uint128::new(0),
                        validator: String::from("mm_binance"),
                    },
                ],
            },
            err: None,
            contract_funds_start: None,
            resp_msgs: vec![
                CosmosMsg::Staking(StakingMsg::Undelegate {
                    validator: String::from("mm_bybit"),
                    amount: coin(100, "unibi"),
                }),
                CosmosMsg::Staking(StakingMsg::Undelegate {
                    validator: String::from("mm_kucoin"),
                    amount: coin(20, "unibi"),
                }),
            ],
        },
        // Fail - oper can't do that
        TestCaseExec {
            to_addrs: to_addrs.to_vec(),
            opers: opers.to_vec(),
            sender: "valid_oper",
            exec_msg: ExecuteMsg::Unstake {
                unstake_msgs: vec![UnstakeMsg {
                    amount: Uint128::new(100),
                    validator: String::from("mm_bybit"),
                }],
            },
            err: Some("Caller is not the contract's current owner"),
            contract_funds_start: None,
            resp_msgs: vec![CosmosMsg::Staking(StakingMsg::Undelegate {
                validator: String::from("mm_bybit"),
                amount: coin(100, "unibi"),
            })],
        },
        // Fail - non oper also can't
        TestCaseExec {
            to_addrs: to_addrs.to_vec(),
            opers: opers.to_vec(),
            sender: "invalid_oper",
            exec_msg: ExecuteMsg::Unstake {
                unstake_msgs: vec![UnstakeMsg {
                    amount: Uint128::new(100),
                    validator: String::from("mm_bybit"),
                }],
            },
            err: Some("Caller is not the contract's current owner"),
            contract_funds_start: None,
            resp_msgs: vec![CosmosMsg::Staking(StakingMsg::Undelegate {
                validator: String::from("mm_bybit"),
                amount: coin(100, "unibi"),
            })],
        },
    ];
    for tc in &test_cases {
        let to_addrs = &tc.to_addrs;
        let opers = &tc.opers;
        // instantiate smart contract from the owner
        let (mut deps, env, _info) =
            setup_contract(to_addrs.clone(), opers.clone())?;

        if let Some(funds_start) = &tc.contract_funds_start {
            // Set up a mock querier with contract balance
            let contract_addr = env.contract.address.to_string();
            let balances: &[(&str, &[Coin])] =
                &[(contract_addr.as_str(), funds_start.as_slice())];
            let querier = testing::MockQuerier::new(balances);
            deps.querier = querier;
        }

        // send the exec msg
        let info = mock_info_for_sender(tc.sender);
        let res = execute(deps.as_mut(), env, info, tc.exec_msg.clone());

        if let Some(want_err) = tc.err {
            let got_err = res.expect_err("errors should occur in this test");
            let is_contained = got_err.to_string().contains(want_err);
            assert!(is_contained, "got error {}", got_err);
            return Ok(());
        }
        assert!(res.is_ok(), "got {res:?}");

        let resp = res?;
        let got_resp_msgs: Vec<CosmosMsgExt> = resp
            .messages
            .iter()
            .map(|sub_msg| CosmosMsgExt(&sub_msg.msg))
            .collect();
        let want_resp_msgs: Vec<CosmosMsgExt> =
            tc.resp_msgs.iter().map(CosmosMsgExt).collect();
        assert_eq!(want_resp_msgs, got_resp_msgs);
    }
    Ok(())
}

#[test]
fn test_withdraw_rewards() -> TestResult {
    let to_addrs: [String; 2] = ["mm_kucoin", "mm_bybit"].map(|s| s.to_string());
    let opers: [String; 1] = ["valid_oper"].map(|s| s.to_string());
    let test_cases: Vec<TestCaseExec> = vec![
        // Success
        TestCaseExec {
            to_addrs: to_addrs.to_vec(),
            opers: opers.to_vec(),
            sender: "owner",
            exec_msg: ExecuteMsg::ClaimRewards {},
            err: None,
            contract_funds_start: None,
            resp_msgs: vec![],
        },
        // Success - oper can do that
        TestCaseExec {
            to_addrs: to_addrs.to_vec(),
            opers: opers.to_vec(),
            sender: "valid_oper",
            exec_msg: ExecuteMsg::ClaimRewards {},
            err: None,
            contract_funds_start: None,
            resp_msgs: vec![],
        },
        // Fail - non oper can't
        TestCaseExec {
            to_addrs: to_addrs.to_vec(),
            opers: opers.to_vec(),
            sender: "invalid_oper",
            exec_msg: ExecuteMsg::ClaimRewards {},
            err: Some("insufficient permissions"),
            contract_funds_start: None,
            resp_msgs: vec![],
        },
    ];
    for tc in &test_cases {
        let to_addrs = &tc.to_addrs;
        let opers = &tc.opers;
        // instantiate smart contract from the owner
        let (mut deps, env, _info) =
            setup_contract(to_addrs.clone(), opers.clone())?;

        if let Some(funds_start) = &tc.contract_funds_start {
            // Set up a mock querier with contract balance
            let contract_addr = env.contract.address.to_string();
            let balances: &[(&str, &[Coin])] =
                &[(contract_addr.as_str(), funds_start.as_slice())];
            let querier = testing::MockQuerier::new(balances);
            deps.querier = querier;
        }

        // send the exec msg
        let info = mock_info_for_sender(tc.sender);
        let res = execute(deps.as_mut(), env, info, tc.exec_msg.clone());

        if let Some(want_err) = tc.err {
            let got_err = res.expect_err("errors should occur in this test");
            let is_contained = got_err.to_string().contains(want_err);
            assert!(is_contained, "got error {}", got_err);
            return Ok(());
        }
        assert!(res.is_ok(), "got {res:?}");

        let resp = res?;
        let got_resp_msgs: Vec<CosmosMsgExt> = resp
            .messages
            .iter()
            .map(|sub_msg| CosmosMsgExt(&sub_msg.msg))
            .collect();
        let want_resp_msgs: Vec<CosmosMsgExt> =
            tc.resp_msgs.iter().map(CosmosMsgExt).collect();
        assert_eq!(want_resp_msgs, got_resp_msgs);
    }
    Ok(())
}
