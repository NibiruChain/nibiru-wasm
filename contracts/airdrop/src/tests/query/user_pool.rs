use crate::contract::{instantiate, query_user_reward, reward_users};
use crate::msg::{InstantiateMsg, RewardUserRequest};
use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
use cosmwasm_std::{coins, from_json, Addr, StdError, Uint128};
use std::vec;

#[test]
fn test_query_user_pool() {
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
        vec![RewardUserRequest {
            user_address: Addr::unchecked("user1"),
            amount: Uint128::new(999),
        }],
    )
    .unwrap();

    let res =
        query_user_reward(deps.as_ref(), env.clone(), Addr::unchecked("user1"))
            .unwrap();
    let user_pool: Uint128 = from_json(res).unwrap();
    assert_eq!(user_pool, Uint128::new(999));
}

#[test]
fn test_query_user_pool_empty() {
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

    let res =
        query_user_reward(deps.as_ref(), env.clone(), Addr::unchecked("user1"));
    assert_eq!(res, Err(StdError::generic_err("User reward does not exist")));
}
