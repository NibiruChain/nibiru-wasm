use crate::contract::{claim, instantiate, reward_users};
use crate::msg::{InstantiateMsg, RewardUserRequest};
use crate::state::{USER_REWARDS};
use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
use cosmwasm_std::{coins, Addr, BankMsg, CosmosMsg, StdError, SubMsg, Uint128};
use std::vec;

#[test]
fn test_claim() {
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
        },
    )
    .unwrap();

    reward_users(
        deps.as_mut(),
        env.clone(),
        mock_info("owner", &[]),
        vec![
            RewardUserRequest {
                user_address: Addr::unchecked("user1"),
                amount: Uint128::new(750),
            },
            RewardUserRequest {
                user_address: Addr::unchecked("user2"),
                amount: Uint128::new(250),
            },
        ],
    )
    .unwrap();

    // try to claim from user1
    let resp =
        claim(deps.as_mut(), env.clone(), mock_info("user1", &[])).unwrap();

    assert_eq!(
        resp.messages,
        vec![SubMsg::new(CosmosMsg::Bank(BankMsg::Send {
            to_address: "user1".to_string(),
            amount: coins(750, ""),
        }))]
    );
    assert_eq!(
        USER_REWARDS.has(deps.as_ref().storage, Addr::unchecked("user1")),
        false
    );

    // try to claim from user2
    let resp =
        claim(deps.as_mut(), env.clone(), mock_info("user2", &[])).unwrap();

    assert_eq!(
        resp.messages,
        vec![SubMsg::new(CosmosMsg::Bank(BankMsg::Send {
            to_address: "user2".to_string(),
            amount: coins(250, ""),
        }))]
    );
    assert_eq!(
        USER_REWARDS.has(deps.as_ref().storage, Addr::unchecked("user2")),
        false
    );

    // try to claim from user3 who doesn't exist
    let resp = claim(deps.as_mut(), env.clone(), mock_info("user3", &[]));

    assert_eq!(resp, Err(StdError::generic_err("User pool does not exist")));
}
