use crate::contract::{instantiate, withdraw};
use crate::msg::InstantiateMsg;
use cosmwasm_std::testing::{
    mock_dependencies, mock_dependencies_with_balance, mock_env, mock_info,
};
use cosmwasm_std::{coins, Addr, BankMsg, CosmosMsg, StdError, SubMsg, Uint128};
use std::vec;

#[test]
fn test_withdraw_ok() {
    let mut deps = mock_dependencies_with_balance(&coins(1000, ""));
    let env = mock_env();

    instantiate(
        deps.as_mut(),
        env.clone(),
        mock_info("owner", &coins(1000, "")),
        InstantiateMsg {
            campaign_id: "campaign_id".to_string(),
            campaign_name: "campaign_name".to_string(),
            campaign_description: "campaign_description".to_string(),
            managers: vec![
                Addr::unchecked("manager1"),
                Addr::unchecked("manager2"),
            ],
        },
    )
    .unwrap();

    // try to withdraw
    let resp = withdraw(
        deps.as_mut(),
        env.clone(),
        mock_info("owner", &[]),
        Uint128::new(1000),
    )
    .unwrap();

    assert_eq!(
        resp.messages,
        vec![SubMsg::new(CosmosMsg::Bank(BankMsg::Send {
            to_address: "owner".to_string(),
            amount: coins(1000, ""),
        }))]
    );
}

#[test]
fn test_withdraw_too_much() {
    let mut deps = mock_dependencies_with_balance(&coins(1000, ""));
    let env = mock_env();

    instantiate(
        deps.as_mut(),
        env.clone(),
        mock_info("owner", &coins(1000, "")),
        InstantiateMsg {
            campaign_id: "campaign_id".to_string(),
            campaign_name: "campaign_name".to_string(),
            campaign_description: "campaign_description".to_string(),
            managers: vec![
                Addr::unchecked("manager1"),
                Addr::unchecked("manager2"),
            ],
        },
    )
    .unwrap();

    // try to withdraw
    let resp = withdraw(
        deps.as_mut(),
        env.clone(),
        mock_info("owner", &[]),
        Uint128::new(1001),
    );

    assert_eq!(
        resp,
        Err(StdError::generic_err("Not enough funds in the contract"))
    );
}

#[test]
fn test_withdraw_unauthorized() {
    let mut deps = mock_dependencies();
    let env = mock_env();

    instantiate(
        deps.as_mut(),
        env.clone(),
        mock_info("owner", &coins(1000, "")),
        InstantiateMsg {
            campaign_id: "campaign_id".to_string(),
            campaign_name: "campaign_name".to_string(),
            campaign_description: "campaign_description".to_string(),
            managers: vec![
                Addr::unchecked("manager1"),
                Addr::unchecked("manager2"),
            ],
        },
    )
    .unwrap();

    // try to withdraw
    let res = withdraw(
        deps.as_mut(),
        env.clone(),
        mock_info("not_owner", &[]),
        Uint128::new(1000),
    );
    assert_eq!(
        res,
        Err(StdError::generic_err("Only contract owner can withdraw"))
    );
}
