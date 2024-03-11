use std::collections::BTreeSet;

use cosmwasm_std::{
    self as cw_std, attr, to_json_binary, AllBalanceResponse, BankMsg,
    BankQuery, Binary, Deps, DepsMut, Env, MessageInfo, QueryRequest, Response,
    StdResult,
};

use crate::oper_perms::Permissions;
use crate::{
    msgs::{PermsStatus, QueryMsg},
    oper_perms,
    state::{Log, IS_HALTED, LOGS, OPERATORS},
};

use cw2::set_contract_version;

use crate::{
    error::ContractError,
    events::{event_bank_send, event_toggle_halt, event_withdraw},
    msgs::{ExecuteMsg, InstantiateMsg},
    state::TO_ADDRS,
};

#[cfg_attr(not(feature = "library"), cosmwasm_std::entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    let contract_addr = env.contract.address.to_string();
    match msg {
        ExecuteMsg::BankSend { coins, to } => {
            // assert sender is operator
            Permissions::assert_operator(deps.storage, info.sender.to_string())?;
            // assert: Operator execute calls should not be halted.
            let is_halted = IS_HALTED.load(deps.storage)?;
            assert_not_halted(is_halted)?;

            // assert: Recipient addr must be in the TO_ADDRS set.
            if !TO_ADDRS.load(deps.storage)?.contains(&to) {
                return Err(ContractError::ToAddrNotAllowed {
                    to_addr: to.to_string(),
                });
            }

            // Events and tx history logging
            let coins_json = serde_json::to_string(&coins)?;
            let event = event_bank_send(&coins_json, info.sender.as_str());
            LOGS.push_front(
                deps.storage,
                &Log {
                    block_height: env.block.height,
                    sender_addr: info.sender.to_string(),
                    event: event.clone(),
                },
            )?;

            // Reply with TxMsg to send funds
            Ok(Response::new()
                .add_message(BankMsg::Send {
                    to_address: to,
                    amount: coins,
                })
                .add_event(event))
        }

        ExecuteMsg::ToggleHalt {} => {
            cw_ownable::assert_owner(deps.storage, &info.sender)?;
            let new_is_halted = !IS_HALTED.load(deps.storage)?;
            IS_HALTED.save(deps.storage, &new_is_halted)?;
            Ok(Response::new().add_event(event_toggle_halt(&new_is_halted)))
        }

        ExecuteMsg::UpdateOwnership(action) => {
            Ok(execute_update_ownership(deps, env, info, action)?)
        }

        ExecuteMsg::EditOpers(action) => {
            cw_ownable::assert_owner(deps.storage, &info.sender)?;
            let mut perms = Permissions::load(deps.storage)?;
            let api = deps.api;
            match action {
                oper_perms::Action::AddOper { address } => {
                    let addr = api.addr_validate(address.as_str())?;
                    perms.operators.insert(addr.into_string());
                    OPERATORS.save(deps.storage, &perms.operators)?;

                    let res = Response::new().add_attributes(vec![
                        attr("action", "add_operator"),
                        attr("address", address),
                    ]);
                    Ok(res)
                }

                oper_perms::Action::RemoveOper { address } => {
                    perms.operators.remove(address.as_str());
                    OPERATORS.save(deps.storage, &perms.operators)?;

                    let res = Response::new().add_attributes(vec![
                        attr("action", "remove_operator"),
                        attr("address", address),
                    ]);
                    Ok(res)
                }
            }
        }

        ExecuteMsg::WithdrawAll { to } => {
            cw_ownable::assert_owner(deps.storage, &info.sender)?;
            let to_addr: String = match to {
                Some(given_to_addr) => given_to_addr,
                None => info.sender.to_string(),
            };
            let balances = query_bank_balances(contract_addr, deps.as_ref())?;
            let tx_msg = BankMsg::Send {
                to_address: to_addr.to_string(),
                amount: balances.amount.clone(),
            };
            let event = event_withdraw(
                serde_json::to_string(&balances.amount)?.as_str(),
                &to_addr,
            );
            LOGS.push_front(
                deps.storage,
                &Log {
                    block_height: env.block.height,
                    sender_addr: info.sender.to_string(),
                    event: event.clone(),
                },
            )?;
            Ok(Response::new().add_message(tx_msg).add_event(event))
        }

        ExecuteMsg::Withdraw { to, denoms } => {
            cw_ownable::assert_owner(deps.storage, &info.sender)?;
            let to_addr: String = match to {
                Some(given_to_addr) => given_to_addr,
                None => info.sender.to_string(),
            };
            let balances: AllBalanceResponse =
                query_bank_balances(contract_addr, deps.as_ref())?;
            let balances: Vec<cw_std::Coin> = balances
                .amount
                .iter()
                .filter(|b_coin| denoms.contains(&b_coin.denom))
                .cloned()
                .collect();

            let tx_msg = BankMsg::Send {
                to_address: to_addr.to_string(),
                amount: balances.clone(),
            };
            let event = event_withdraw(
                serde_json::to_string(&balances)?.as_str(),
                &to_addr,
            );
            LOGS.push_front(
                deps.storage,
                &Log {
                    block_height: env.block.height,
                    sender_addr: info.sender.to_string(),
                    event: event.clone(),
                },
            )?;
            Ok(Response::new().add_message(tx_msg).add_event(event))
        }
    }
}

fn execute_update_ownership(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    action: cw_ownable::Action,
) -> Result<Response, cw_ownable::OwnershipError> {
    let ownership =
        cw_ownable::update_ownership(deps, &env.block, &info.sender, action)?;
    Ok(Response::new().add_attributes(ownership.into_attributes()))
}

fn assert_not_halted(is_halted: bool) -> Result<(), ContractError> {
    match is_halted {
        true => Ok(()),
        false => Err(ContractError::OperationsHalted),
    }
}

pub const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
pub const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), cosmwasm_std::entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(
        deps.storage,
        format!("crates.io:{CONTRACT_NAME}"),
        CONTRACT_VERSION,
    )?;
    cw_ownable::initialize_owner(deps.storage, deps.api, Some(&msg.owner))?;
    TO_ADDRS.save(deps.storage, &msg.to_addrs)?;
    OPERATORS.save(deps.storage, &msg.opers)?;
    IS_HALTED.save(deps.storage, &false)?;
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), cosmwasm_std::entry_point)]
pub fn query(
    deps: Deps,
    _env: Env,
    msg: QueryMsg,
) -> Result<Binary, ContractError> {
    match msg {
        QueryMsg::Perms {} => {
            let perms_status: PermsStatus = query_perms_status(deps)?;
            Ok(to_json_binary(&perms_status)?)
        }
        QueryMsg::Ownership {} => {
            Ok(to_json_binary(&cw_ownable::get_ownership(deps.storage)?)?)
        }
    }
}

pub fn query_accepted_denoms(deps: Deps) -> StdResult<BTreeSet<String>> {
    TO_ADDRS.load(deps.storage)
}


/// Query all bank balances or return an empty response.
///
/// ```rust
/// use broker_bank::contract::query_bank_balances;
/// use cosmwasm_std::{
///     testing::{mock_dependencies, mock_env},
///     AllBalanceResponse, DepsMut, Env, StdResult};
///
/// let env: Env = mock_env();
/// let mut deps = mock_dependencies();
/// let mut deps: DepsMut = deps.as_mut();
/// let contract_addr = env.contract.address.to_string();
/// let balances: StdResult<AllBalanceResponse> =
///    query_bank_balances(contract_addr.to_string(), deps.as_ref());
/// assert!(balances.is_ok())
/// ```
pub fn query_bank_balances(
    addr: String,
    deps: Deps,
) -> StdResult<AllBalanceResponse> {
    let query_result =
        deps.querier
            .query(&QueryRequest::Bank(BankQuery::AllBalances {
                address: addr,
            }))?;
    let balances: AllBalanceResponse = match query_result {
        Some(res) => res,
        None => AllBalanceResponse::default(),
    };
    Ok(balances)
}

pub fn query_perms_status(deps: Deps) -> Result<PermsStatus, ContractError> {
    let perms = oper_perms::Permissions::load(deps.storage)?;
    let perms_status = PermsStatus {
        perms,
        is_halted: IS_HALTED.load(deps.storage)?,
    };
    Ok(perms_status)
}

#[cfg(test)]
pub mod tests {
    use cosmwasm_std::{
        self as cw_std,
        testing::{mock_env, mock_info},
        DepsMut, MessageInfo,
    };
    use cw_std::{testing, Coin};
    use nibiru_std::errors::TestResult;

    use crate::{
        contract::execute,
        msgs::ExecuteMsg,
        oper_perms,
        tutil::{mock_info_for_sender, setup_contract, TEST_OWNER},
    };


    struct TestCaseExec<'a> {
        to_addrs: Vec<String>,
        opers: Vec<String>,
        exec_msg: ExecuteMsg,
        sender: &'a str,
        err: Option<&'a str>,
        contract_funds_start: Option<&'a Coin>,
    }

    /// Test that all owner-gated execute calls fail when the tx sender is not
    /// the smart contract owner.
    #[test]
    pub fn test_assert_owner() -> TestResult {
        let not_owner = "not-owner";
        let want_err: Option<&str> = Some("not the contract's current owner");

        let to_addrs: [String; 2] =
            ["mm_kucoin", "mm_bybit"].map(|s| s.to_string());
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
            },
            TestCaseExec {
                to_addrs: to_addrs.to_vec(),
                opers: opers.to_vec(),
                sender: not_owner,
                exec_msg: ExecuteMsg::UpdateOwnership(
                    cw_ownable::Action::TransferOwnership {
                        new_owner: String::from("new_owner"),
                        expiry: None,
                    },
                ),
                err: want_err,
                contract_funds_start: None,
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
            },
            TestCaseExec {
                to_addrs: to_addrs.to_vec(),
                opers: opers.to_vec(),
                sender: not_owner,
                exec_msg: ExecuteMsg::ToggleHalt {},
                err: want_err,
                contract_funds_start: None,
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

    // TODO: test update ownership
    #[test]
    fn exec_withdraw() -> TestResult {
        let to_addrs: [String; 2] =
            ["mm_kucoin", "mm_bybit"].map(|s| s.to_string());
        let opers: [String; 1] = ["valid_oper"].map(|s| s.to_string());
        let test_cases: Vec<TestCaseExec> = vec![
            TestCaseExec {
                to_addrs: to_addrs.to_vec(),
                opers: opers.to_vec(),
                sender: TEST_OWNER,
                exec_msg: ExecuteMsg::WithdrawAll {
                    to: Some(String::from("mm_bybit")),
                },
                err: None,
                contract_funds_start: None,
            },
        ];
        for tc in &test_cases {
            let to_addrs = &tc.to_addrs;
            let opers = &tc.opers;
            // instantiate smart contract from the owner
            let (mut deps, env, _info) =
                setup_contract(to_addrs.clone(), opers.clone())?;

            if let Some(funds_start) = tc.contract_funds_start {
                // Set up a mock querier with contract balance
                let contract_addr = env.contract.address.to_string();
                let balances: &[(&str, &[Coin])] =
                    &[(contract_addr.as_str(), &[funds_start.clone()])];
                let querier = testing::MockQuerier::new(balances);
                deps.querier = querier;
            }

            // send the exec msg
            let info = mock_info_for_sender(tc.sender);
            let res = execute(deps.as_mut(), env, info, tc.exec_msg.clone());
            if let Some(want_err) = tc.err {
                let got_err = res.expect_err("errors should occur in this test");
                let is_contained = got_err
                    .to_string()
                    .contains(want_err);
                assert!(is_contained, "got error {}", got_err);
                return Ok(())
            }
            assert!(res.is_ok());
        }
        Ok(())
    }
}
