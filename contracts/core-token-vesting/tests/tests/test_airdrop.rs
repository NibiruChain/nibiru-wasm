use anyhow::anyhow;
use cosmwasm_std::{
    coin,
    testing::{mock_env, mock_info, MockApi, MockQuerier, MockStorage},
    Addr, Empty, Env, OwnedDeps, StdError, Uint128, Uint64,
};
use cw20::Denom;
use token_vesting::{
    contract::execute,
    errors::ContractError,
    msg::{ExecuteMsg, RewardUserRequest, VestingSchedule},
    state::{CAMPAIGN, USER_REWARDS},
};

use super::{helpers::TestResult, test_manager::setup_with_block_time};

#[test]
fn execute_create_campaign_valid() -> TestResult {
    let (mut deps, env) = setup_with_block_time(0)?;

    // Create a campaign with valid parameters
    let create_campaign_msg = ExecuteMsg::CreateCampaign {
        vesting_schedule: VestingSchedule::LinearVesting {
            start_time: Uint64::new(100),
            end_time: Uint64::new(200),
            vesting_amount: Uint128::new(5000),
        },
        campaign_id: "campaign1".to_string(),
        campaign_name: "Test Campaign".to_string(),
        campaign_description: "A test campaign".to_string(),
        managers: vec!["manager1".to_string(), "manager2".to_string()],
    };
    let res = execute(
        deps.as_mut(),
        env,
        mock_info("creator", &[coin(5000, "token")]),
        create_campaign_msg,
    )?;

    // Assertions to verify the campaign is created correctly
    assert!(
        res.attributes
            .iter()
            .any(|attr| attr.key == "method" && attr.value == "create_campaign"),
        "Expected 'create_campaign' method in response attributes"
    );
    assert!(
        CAMPAIGN.has(&deps.storage, "campaign1".to_string()),
        "Campaign should be saved in state"
    );

    Ok(())
}

#[test]
fn execute_create_campaign_duplicate_id() -> TestResult {
    let (mut deps, _env) = setup_with_block_time(0)?;

    // Create a campaign with a unique ID
    let campaign_id = "unique_campaign_id";
    let create_campaign_msg = ExecuteMsg::CreateCampaign {
        campaign_id: campaign_id.to_string(),
        campaign_name: "Test Campaign".to_string(),
        campaign_description: "This is a test campaign".to_string(),
        managers: vec![],
        vesting_schedule: VestingSchedule::LinearVesting {
            start_time: Uint64::new(100),
            end_time: Uint64::new(200),
            vesting_amount: Uint128::new(1000),
        },
    };

    execute(
        deps.as_mut(),
        mock_env(),
        mock_info("creator", &[coin(5000, "token")]),
        create_campaign_msg.clone(),
    )?;

    // Attempt to create another campaign with the same ID
    let res = execute(
        deps.as_mut(),
        mock_env(),
        mock_info("creator", &[coin(5000, "token")]),
        create_campaign_msg,
    );

    match res {
        Err(ContractError::Std(StdError::GenericErr { msg, .. }))
            if msg.contains("Campaign already exists") =>
        {
            Ok(())
        }
        _ => Err(anyhow!(
            "Expected 'Campaign already exists' error, found {:?}",
            res
        )),
    }
}

#[test]
fn execute_create_campaign_invalid_coin_count() -> TestResult {
    let (mut deps, env) = setup_with_block_time(0)?;

    // Create a campaign with invalid coin count
    let create_campaign_msg = ExecuteMsg::CreateCampaign {
        vesting_schedule: VestingSchedule::LinearVesting {
            start_time: Uint64::new(100),
            end_time: Uint64::new(200),
            vesting_amount: Uint128::new(5000),
        },
        campaign_id: "campaign1".to_string(),
        campaign_name: "Test Campaign".to_string(),
        campaign_description: "A test campaign".to_string(),
        managers: vec!["manager1".to_string(), "manager2".to_string()],
    };
    let res = execute(
        deps.as_mut(),
        env,
        mock_info("creator", &[]),
        create_campaign_msg,
    );

    match res {
        Err(ContractError::Std(StdError::GenericErr { msg, .. }))
            if msg.contains("one denom sent required") =>
        {
            Ok(())
        }
        _ => Err(anyhow!(
            "Expected 'one denom sent required' error, found {:?}",
            res
        )),
    }
}

#[test]
fn execute_create_campaign_2_coins() -> TestResult {
    let (mut deps, env) = setup_with_block_time(0)?;

    // Create a campaign with 2 coins
    let create_campaign_msg = ExecuteMsg::CreateCampaign {
        vesting_schedule: VestingSchedule::LinearVesting {
            start_time: Uint64::new(100),
            end_time: Uint64::new(200),
            vesting_amount: Uint128::new(5000),
        },
        campaign_id: "campaign2".to_string(),
        campaign_name: "Test Campaign".to_string(),
        campaign_description: "A test campaign".to_string(),
        managers: vec!["manager1".to_string(), "manager2".to_string()],
    };
    let res = execute(
        deps.as_mut(),
        env,
        mock_info("creator", &[coin(5000, "token"), coin(5000, "token")]),
        create_campaign_msg,
    );

    match res {
        Err(ContractError::Std(StdError::GenericErr { msg, .. }))
            if msg.contains("one denom sent required") =>
        {
            Ok(())
        }
        _ => Err(anyhow!(
            "Expected 'one denom sent required' error, found {:?}",
            res
        )),
    }
}

#[test]
fn execute_reward_users_unactive_campaign() -> TestResult {
    let (mut deps, env) = setup_with_block_time(0)?;

    // Create a campaign
    let campaign_id = "campaign1".to_string();
    execute(
        deps.as_mut(),
        env.clone(),
        mock_info("creator", &[coin(10000, "token")]),
        ExecuteMsg::CreateCampaign {
            campaign_id: campaign_id.clone(),
            campaign_name: "Campaign One".to_string(),
            campaign_description: "The first campaign".to_string(),
            managers: vec!["manager1".to_string()],
            vesting_schedule: VestingSchedule::LinearVesting {
                start_time: Uint64::new(100),
                end_time: Uint64::new(200),
                vesting_amount: Uint128::new(10000),
            },
        },
    )?;

    // Deactivate the campaign
    let msg = ExecuteMsg::DeactivateCampaign {
        campaign_id: campaign_id.clone(),
    };
    let info = mock_info("creator", &[]);
    execute(deps.as_mut(), env.clone(), info, msg)?;

    // Reward users
    let reward_users_msg = ExecuteMsg::RewardUsers {
        campaign_id: campaign_id.clone(),
        requests: vec![
            RewardUserRequest {
                user_address: "user1".to_string(),
                amount: Uint128::new(500),
            },
            RewardUserRequest {
                user_address: "user2".to_string(),
                amount: Uint128::new(1500),
            },
        ],
    };
    let res = execute(
        deps.as_mut(),
        env,
        mock_info("creator", &[]),
        reward_users_msg,
    );

    match res {
        Err(ContractError::Std(StdError::GenericErr { msg, .. }))
            if msg.contains("Campaign is not active") =>
        {
            Ok(())
        }
        _ => Err(anyhow!(
            "Expected 'Campaign is not active' error, found {:?}",
            res
        )),
    }
}

#[test]
fn execute_reward_users_unauthorized() -> TestResult {
    let (mut deps, env) = setup_with_block_time(0)?;

    // Create a campaign
    let campaign_id = "campaign1".to_string();
    execute(
        deps.as_mut(),
        env.clone(),
        mock_info("creator", &[coin(10000, "token")]),
        ExecuteMsg::CreateCampaign {
            campaign_id: campaign_id.clone(),
            campaign_name: "Campaign One".to_string(),
            campaign_description: "The first campaign".to_string(),
            managers: vec!["manager1".to_string()],
            vesting_schedule: VestingSchedule::LinearVesting {
                start_time: Uint64::new(100),
                end_time: Uint64::new(200),
                vesting_amount: Uint128::new(10000),
            },
        },
    )?;

    // Reward users
    let reward_users_msg = ExecuteMsg::RewardUsers {
        campaign_id: campaign_id.clone(),
        requests: vec![
            RewardUserRequest {
                user_address: "user1".to_string(),
                amount: Uint128::new(500),
            },
            RewardUserRequest {
                user_address: "user2".to_string(),
                amount: Uint128::new(1500),
            },
        ],
    };
    let res = execute(
        deps.as_mut(),
        env,
        mock_info("unauthorized_user", &[]),
        reward_users_msg,
    );

    match res {
        Err(ContractError::Std(StdError::GenericErr { msg, .. }))
            if msg.contains("Unauthorized") =>
        {
            Ok(())
        }
        _ => Err(anyhow!("Expected 'Unauthorized' error, found {:?}", res)),
    }
}

#[test]
fn execute_reward_users_valid() -> TestResult {
    let (mut deps, env) = setup_with_block_time(0)?;

    // Create a campaign
    let campaign_id = "campaign1".to_string();
    execute(
        deps.as_mut(),
        env.clone(),
        mock_info("creator", &[coin(10000, "token")]),
        ExecuteMsg::CreateCampaign {
            campaign_id: campaign_id.clone(),
            campaign_name: "Campaign One".to_string(),
            campaign_description: "The first campaign".to_string(),
            managers: vec!["manager1".to_string()],
            vesting_schedule: VestingSchedule::LinearVesting {
                start_time: Uint64::new(100),
                end_time: Uint64::new(200),
                vesting_amount: Uint128::new(10000),
            },
        },
    )?;

    // Reward users
    let reward_users_msg = ExecuteMsg::RewardUsers {
        campaign_id: campaign_id.clone(),
        requests: vec![
            RewardUserRequest {
                user_address: "user1".to_string(),
                amount: Uint128::new(500),
            },
            RewardUserRequest {
                user_address: "user2".to_string(),
                amount: Uint128::new(1500),
            },
        ],
    };
    execute(
        deps.as_mut(),
        env.clone(),
        mock_info("creator", &[]),
        reward_users_msg,
    )?;

    // Verify user rewards and campaign state
    let user1_rewards =
        USER_REWARDS.load(deps.as_ref().storage, "user1".to_string())?;
    assert_eq!(
        user1_rewards,
        Uint128::new(500),
        "User1 rewards do not match."
    );

    let user2_rewards =
        USER_REWARDS.load(deps.as_ref().storage, "user2".to_string())?;
    assert_eq!(
        user2_rewards,
        Uint128::new(1500),
        "User2 rewards do not match."
    );

    let updated_campaign =
        CAMPAIGN.load(deps.as_ref().storage, campaign_id.clone())?;
    assert_eq!(
        updated_campaign.unallocated_amount,
        Uint128::new(8000),
        "Campaign unallocated amount does not match expected."
    );

    // Additional reward
    let reward_users_msg = ExecuteMsg::RewardUsers {
        campaign_id: campaign_id.clone(),
        requests: vec![RewardUserRequest {
            user_address: "user1".to_string(),
            amount: Uint128::new(1000),
        }],
    };
    execute(
        deps.as_mut(),
        env,
        mock_info("creator", &[]),
        reward_users_msg,
    )?;

    // Verify user rewards and campaign state
    let user1_rewards =
        USER_REWARDS.load(deps.as_ref().storage, "user1".to_string())?;
    assert_eq!(
        user1_rewards,
        Uint128::new(1500),
        "User1 rewards do not match."
    );

    Ok(())
}

#[test]
fn execute_reward_users_insufficient_funds() -> TestResult {
    let (mut deps, _env) = setup_with_block_time(0)?;

    // Create a campaign with limited funds
    let campaign_id = "limited_fund_campaign";
    execute(
        deps.as_mut(),
        mock_env(),
        mock_info("creator", &[coin(500, "token")]),
        ExecuteMsg::CreateCampaign {
            campaign_id: campaign_id.to_string(),
            campaign_name: "Limited Fund Campaign".to_string(),
            campaign_description: "This campaign has limited funds".to_string(),
            managers: vec![],
            vesting_schedule: VestingSchedule::LinearVesting {
                start_time: Uint64::new(100),
                end_time: Uint64::new(200),
                vesting_amount: Uint128::new(500),
            },
        },
    )?;

    // Attempt to reward users more than available funds
    let res = execute(
        deps.as_mut(),
        mock_env(),
        mock_info("creator", &[]),
        ExecuteMsg::RewardUsers {
            campaign_id: campaign_id.to_string(),
            requests: vec![RewardUserRequest {
                user_address: "user1".to_string(),
                amount: Uint128::new(600), // More than available
            }],
        },
    );

    match res {
        Err(ContractError::Std(StdError::GenericErr { msg, .. }))
            if msg.contains("Insufficient funds for all rewards") =>
        {
            Ok(())
        }
        _ => Err(anyhow!(
            "Expected 'Insufficient funds for all rewards' error, found {:?}",
            res
        )),
    }
}

#[test]
fn execute_claim_no_vesting_account() -> TestResult {
    let (mut deps, env) = setup_with_block_time(0)?;

    // Attempt to claim tokens without registering a vesting account
    let claim_msg = ExecuteMsg::Claim {
        denoms: vec![Denom::Native("token".to_string())],
        recipient: Some("recipient".to_string()),
    };
    let info = mock_info("user1", &[]);
    let res = execute(deps.as_mut(), env.clone(), info, claim_msg);

    // Verify that it results in an error
    match res {
        Err(ContractError::Std(StdError::GenericErr { msg, .. })) => {
            assert!(
                msg.contains("vesting entry is not found for denom"),
                "Unexpected error message: {}",
                msg
            );
        }
        _ => return Err(anyhow!("Expected error, got {:?}", res)),
    }

    Ok(())
}

#[test]
fn execute_withdraw_valid() -> TestResult {
    let (mut deps, env) = setup_with_block_time(0)?;

    // Create a campaign first
    let create_campaign_msg = ExecuteMsg::CreateCampaign {
        vesting_schedule: VestingSchedule::LinearVesting {
            start_time: Uint64::new(env.block.time.seconds()),
            end_time: Uint64::new(env.block.time.seconds() + 100),
            vesting_amount: Uint128::new(1000),
        },
        campaign_id: "campaign1".to_string(),
        campaign_name: "Test Campaign".to_string(),
        campaign_description: "A campaign for testing".to_string(),
        managers: vec!["manager1".to_string()],
    };
    let info = mock_info("owner", &[coin(1000, "denom")]);
    execute(deps.as_mut(), env.clone(), info, create_campaign_msg)?;

    // fund the contract manually
    deps.querier.update_balance(
        Addr::unchecked(&env.contract.address),
        vec![coin(1000, "denom")],
    );

    // Attempt to withdraw unallocated funds
    let withdraw_msg = ExecuteMsg::Withdraw {
        amount: Uint128::new(500),
        campaign_id: "campaign1".to_string(),
    };
    let info = mock_info("owner", &[]);
    execute(deps.as_mut(), env.clone(), info, withdraw_msg)?;

    // Verify campaign unallocated amount is updated
    let campaign = CAMPAIGN
        .load(&deps.storage, "campaign1".to_string())
        .unwrap();
    assert_eq!(
        campaign.unallocated_amount,
        Uint128::new(500),
        "Campaign unallocated amount not updated correctly"
    );

    Ok(())
}

#[test]
fn execute_withdraw_unauthorized() -> TestResult {
    let (mut deps, env) = setup_with_block_time(100)?;

    // Create a campaign with some funds
    create_test_campaign(&mut deps, &env, "campaign1", "owner");

    // Attempt to withdraw funds from the contract by an unauthorized user
    let msg = ExecuteMsg::Withdraw {
        amount: Uint128::new(500),
        campaign_id: "campaign1".to_string(),
    };
    let info = mock_info("unauthorized_user", &[]);
    let res = execute(deps.as_mut(), env.clone(), info, msg);

    match res {
        Err(ContractError::Std(StdError::GenericErr { msg, .. }))
            if msg.contains("Only contract owner can withdraw") =>
        {
            Ok(())
        }
        _ => Err(anyhow!("Expected unauthorized withdraw attempt to fail")),
    }
}

#[test]
fn execute_deactivate_campaign_authorized() -> TestResult {
    let (mut deps, env) = setup_with_block_time(200)?;

    // Create a campaign and mark it active
    create_test_campaign(&mut deps, &env, "campaign2", "owner");

    // Deactivate the campaign by the owner
    let msg = ExecuteMsg::DeactivateCampaign {
        campaign_id: "campaign2".to_string(),
    };
    let info = mock_info("owner", &[]);
    execute(deps.as_mut(), env.clone(), info, msg)?;

    // Check if the campaign is deactivated
    let campaign =
        CAMPAIGN.load(deps.as_ref().storage, "campaign2".to_string())?;
    assert_eq!(campaign.is_active, false, "Campaign should be deactivated");

    Ok(())
}

#[test]
fn execute_deactivate_campaign_unauthorized() -> TestResult {
    let (mut deps, env) = setup_with_block_time(300)?;

    // Create a campaign and mark it active
    create_test_campaign(&mut deps, &env, "campaign3", "owner");

    // Attempt to deactivate the campaign by an unauthorized user
    let msg = ExecuteMsg::DeactivateCampaign {
        campaign_id: "campaign3".to_string(),
    };
    let info = mock_info("unauthorized_user", &[]);
    let res = execute(deps.as_mut(), env, info, msg);

    match res {
        Err(ContractError::Std(StdError::GenericErr { msg, .. }))
            if msg.contains("Unauthorized") =>
        {
            Ok(())
        }
        _ => Err(anyhow!(
            "Expected unauthorized deactivation attempt to fail"
        )),
    }
}

// Helper function to create a test campaign
fn create_test_campaign(
    deps: &mut OwnedDeps<MockStorage, MockApi, MockQuerier, Empty>,
    env: &Env,
    campaign_id: &str,
    owner: &str,
) {
    let msg = ExecuteMsg::CreateCampaign {
        vesting_schedule: VestingSchedule::LinearVesting {
            start_time: Uint64::new(env.block.time.seconds() + 100),
            end_time: Uint64::new(env.block.time.seconds() + 200),
            vesting_amount: Uint128::new(1000),
        },
        campaign_id: campaign_id.to_string(),
        campaign_name: "Test Campaign".to_string(),
        campaign_description: "A campaign for testing".to_string(),
        managers: vec![owner.to_string()],
    };
    let info = mock_info(owner, &[coin(1000, "token")]);
    let _ = execute(deps.as_mut(), env.clone(), info, msg).unwrap();
}
