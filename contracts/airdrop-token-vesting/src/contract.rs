#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_json_binary, Attribute, BankMsg, Binary, Coin, CosmosMsg, Deps, DepsMut,
    Env, MessageInfo, Order, Response, StdError, StdResult, Storage, Timestamp,
    Uint128,
};
use std::cmp::min;

use serde_json::to_string;

use cw20::Denom;
use cw_storage_plus::Bound;

use crate::errors::ContractError;
use crate::msg::{
    ExecuteMsg, InstantiateMsg, QueryMsg, RewardUserRequest, RewardUserResponse,
    VestingAccountResponse, VestingData, VestingSchedule,
};
use crate::state::{
    denom_to_key, VestingAccount, Whitelist, DENOM, UNALLOCATED_AMOUNT,
    VESTING_ACCOUNTS, WHITELIST,
};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    // Funds validation
    if info.funds.len() != 1 {
        return Err(StdError::generic_err(
            "must deposit exactly one type of token",
        )
        .into());
    }
    if info.funds[0].amount.is_zero() {
        return Err(StdError::generic_err("must deposit some token").into());
    }
    // Managers validation
    if msg.managers.is_empty() {
        return Err(StdError::generic_err("managers cannot be empty").into());
    }

    deps.api.addr_validate(&msg.admin)?;
    for manager in msg.managers.iter() {
        let _ = deps.api.addr_validate(manager)?;
    }

    let unallocated_amount = info.funds[0].amount;
    let denom = info.funds[0].denom.clone();

    UNALLOCATED_AMOUNT.save(deps.storage, &unallocated_amount)?;
    DENOM.save(deps.storage, &denom)?;
    WHITELIST.save(
        deps.storage,
        &Whitelist {
            members: msg.managers.clone().into_iter().collect(),
            admin: msg.admin.clone(),
        },
    )?;

    Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::RewardUsers {
            rewards,
            master_address,
            vesting_schedule,
        } => reward_users(
            deps,
            env,
            info,
            rewards,
            master_address,
            vesting_schedule,
        ),
        ExecuteMsg::DeregisterVestingAccount {
            address,
            denom,
            vested_token_recipient,
            left_vesting_token_recipient,
        } => deregister_vesting_account(
            deps,
            env,
            info,
            address,
            denom,
            vested_token_recipient,
            left_vesting_token_recipient,
        ),
        ExecuteMsg::Claim { denoms, recipient } => {
            claim(deps, env, info, denoms, recipient)
        }
        ExecuteMsg::Withdraw { amount, recipient } => {
            withdraw(deps, env, info, amount, recipient)
        }
    }
}

/// Allow the contract owner to withdraw the funds of the campaign
///
/// Ensures the requested amount is less than or equal to the unallocated amount
pub fn withdraw(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    amount: Uint128,
    recipient: String,
) -> Result<Response, ContractError> {
    let whitelist = WHITELIST.load(deps.storage)?;
    let mut unallocated_amount = UNALLOCATED_AMOUNT.load(deps.storage)?;
    let denom = DENOM.load(deps.storage)?;

    if !whitelist.is_admin(info.sender.clone()) {
        return Err(StdError::generic_err("Unauthorized").into());
    }

    let amount_max = min(amount, unallocated_amount);
    if amount_max.is_zero() {
        return Err(StdError::generic_err("Nothing to withdraw").into());
    }

    unallocated_amount = unallocated_amount - amount_max;
    UNALLOCATED_AMOUNT.save(deps.storage, &unallocated_amount)?;

    // validate recipient address
    deps.api.addr_validate(&recipient.clone())?;

    Ok(Response::new()
        .add_messages(vec![build_send_msg(
            cw20::Denom::Native(denom),
            amount_max,
            recipient.clone(),
        )?])
        .add_attribute("action", "withdraw")
        .add_attribute("recipient", &recipient)
        .add_attribute("amount", &amount_max.to_string())
        .add_attribute("unallocated_amount", &unallocated_amount.to_string()))
}

fn reward_users(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    rewards: Vec<RewardUserRequest>,
    master_address: Option<String>,
    mut vesting_schedule: VestingSchedule,
) -> Result<Response, ContractError> {
    let mut res = vec![];

    let whitelist = WHITELIST.load(deps.storage)?;

    if !(whitelist.is_member(info.sender.clone())
        || whitelist.is_admin(info.sender.clone()))
    {
        return Err(StdError::generic_err("Unauthorized").into());
    }

    let unallocated_amount = UNALLOCATED_AMOUNT.load(deps.storage)?;
    let denom = DENOM.load(deps.storage)?;

    let total_requested: Uint128 =
        rewards.iter().map(|req| req.vesting_amount).sum();
    if total_requested > unallocated_amount {
        return Err(
            StdError::generic_err("Insufficient funds for all rewards").into()
        );
    }
    vesting_schedule.validate_time(env.block.time)?;

    let mut attrs: Vec<Attribute> = vec![];
    for req in rewards {
        // validate amounts and cliff details if there's one
        req.validate(vesting_schedule.clone())?;

        // update the vesting schedule to match with the request
        match &mut vesting_schedule {
            VestingSchedule::LinearVesting { vesting_amount, .. } => {
                *vesting_amount = req.vesting_amount;
            }
            VestingSchedule::LinearVestingWithCliff {
                vesting_amount,
                cliff_amount,
                ..
            } => {
                *vesting_amount = req.vesting_amount;
                *cliff_amount = req.cliff_amount.unwrap();
            }
        }

        let result = register_vesting_account(
            deps.storage,
            env.block.time,
            master_address.clone(),
            req.user_address.clone(),
            Denom::Native(denom.clone()),
            req.vesting_amount,
            vesting_schedule.clone(),
        );

        if let Ok(response) = result {
            attrs.extend(response.attributes);
            res.push(RewardUserResponse {
                user_address: req.user_address.clone(),
                success: true,
                error_msg: "".to_string(),
            });
        } else {
            let error = result.err().unwrap();
            res.push(RewardUserResponse {
                user_address: req.user_address.clone(),
                success: false,
                error_msg: "Failed to register vesting account: ".to_string()
                    + &error.to_string(),
            });
        }
    }

    UNALLOCATED_AMOUNT
        .save(deps.storage, &(unallocated_amount - total_requested))?;

    Ok(Response::new()
        .add_attributes(attrs)
        .add_attribute("method", "reward_users")
        .set_data(to_json_binary(&res).unwrap()))
}

fn register_vesting_account(
    storage: &mut dyn Storage,
    block_time: Timestamp,
    master_address: Option<String>,
    address: String,
    deposit_denom: Denom,
    deposit_amount: Uint128,
    vesting_schedule: VestingSchedule,
) -> Result<Response, ContractError> {
    let denom_key = denom_to_key(deposit_denom.clone());

    // vesting_account existence check
    if VESTING_ACCOUNTS.has(storage, (address.as_str(), &denom_key)) {
        return Err(StdError::generic_err("already exists").into());
    }
    vesting_schedule.validate(block_time)?;

    VESTING_ACCOUNTS.save(
        storage,
        (address.as_str(), &denom_key),
        &VestingAccount {
            master_address: master_address.clone(),
            address: address.to_string(),
            vesting_denom: deposit_denom.clone(),
            vesting_amount: deposit_amount,
            vesting_schedule,
            claimed_amount: Uint128::zero(),
        },
    )?;

    Ok(Response::new().add_attributes(vec![
        ("action", "register_vesting_account"),
        (
            "master_address",
            master_address.unwrap_or_default().as_str(),
        ),
        ("address", address.as_str()),
        ("vesting_denom", &to_string(&deposit_denom).unwrap()),
        ("vesting_amount", &deposit_amount.to_string()),
    ]))
}

fn deregister_vesting_account(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    address: String,
    denom: Denom,
    vested_token_recipient: Option<String>,
    left_vesting_token_recipient: Option<String>,
) -> Result<Response, ContractError> {
    let denom_key = denom_to_key(denom.clone());
    let sender = info.sender;

    let mut messages: Vec<CosmosMsg> = vec![];

    // vesting_account existence check
    let account = VESTING_ACCOUNTS
        .may_load(deps.storage, (address.as_str(), &denom_key))?;
    if account.is_none() {
        return Err(ContractError::Std(StdError::generic_err(format!(
            "vesting entry is not found for denom {:?}",
            to_string(&denom).unwrap(),
        ))));
    }

    let account = account.unwrap();
    if account.master_address.is_none()
        || account.master_address.unwrap() != sender
    {
        return Err(StdError::generic_err("unauthorized").into());
    }

    // remove vesting account
    VESTING_ACCOUNTS.remove(deps.storage, (address.as_str(), &denom_key));

    let vested_amount = account
        .vesting_schedule
        .vested_amount(env.block.time.seconds())?;
    let claimed_amount = account.claimed_amount;

    // transfer already vested but not claimed amount to
    // a account address or the given `vested_token_recipient` address
    let claimable_amount = vested_amount.checked_sub(claimed_amount)?;
    send_if_amount_is_not_zero(
        &mut messages,
        claimable_amount,
        account.vesting_denom.clone(),
        vested_token_recipient,
        address.clone(),
    )?;

    // transfer left vesting amount to owner or
    // the given `left_vesting_token_recipient` address
    let left_vesting_amount =
        account.vesting_amount.checked_sub(vested_amount)?;
    send_if_amount_is_not_zero(
        &mut messages,
        left_vesting_amount,
        account.vesting_denom.clone(),
        left_vesting_token_recipient,
        sender.clone().to_string(),
    )?;

    Ok(Response::new().add_messages(messages).add_attributes(vec![
        ("action", "deregister_vesting_account"),
        ("address", address.as_str()),
        ("vesting_denom", &to_string(&account.vesting_denom).unwrap()),
        ("vesting_amount", &account.vesting_amount.to_string()),
        ("vested_amount", &vested_amount.to_string()),
        ("left_vesting_amount", &left_vesting_amount.to_string()),
    ]))
}

///
/// creates a send message if the amount to send is not zero
///
/// If we provide a recipient, we use it. Otherwise, we use the default_recipient
fn send_if_amount_is_not_zero(
    messages: &mut Vec<CosmosMsg>,
    amount: Uint128,
    denom: Denom,
    recipient_option: Option<String>,
    default_recipient: String,
) -> Result<(), ContractError> {
    if !amount.is_zero() {
        let recipient = recipient_option.unwrap_or_else(|| default_recipient);
        let msg_send: CosmosMsg = build_send_msg(denom, amount, recipient)?;
        messages.push(msg_send);
    }

    Ok(())
}

fn claim(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    denoms: Vec<Denom>,
    recipient: Option<String>,
) -> Result<Response, ContractError> {
    let sender = info.sender;
    let recipient = recipient.unwrap_or_else(|| sender.to_string());

    let mut messages: Vec<CosmosMsg> = vec![];
    let mut attrs: Vec<Attribute> = vec![];
    for denom in denoms.iter() {
        let denom_key = denom_to_key(denom.clone());

        // vesting_account existence check
        let account = VESTING_ACCOUNTS
            .may_load(deps.storage, (sender.as_str(), &denom_key))?;
        if account.is_none() {
            return Err(StdError::generic_err(format!(
                "vesting entry is not found for denom {}",
                to_string(&denom).unwrap(),
            ))
            .into());
        }

        let mut account = account.unwrap();
        let vested_amount = account
            .vesting_schedule
            .vested_amount(env.block.time.seconds())?;
        let claimed_amount = account.claimed_amount;

        let claimable_amount = vested_amount.checked_sub(claimed_amount)?;
        if claimable_amount.is_zero() {
            continue;
        }

        account.claimed_amount = vested_amount;
        if account.claimed_amount == account.vesting_amount {
            VESTING_ACCOUNTS.remove(deps.storage, (sender.as_str(), &denom_key));
        } else {
            VESTING_ACCOUNTS.save(
                deps.storage,
                (sender.as_str(), &denom_key),
                &account,
            )?;
        }

        let msg_send: CosmosMsg = build_send_msg(
            account.vesting_denom.clone(),
            claimable_amount,
            recipient.clone(),
        )?;

        messages.push(msg_send);
        attrs.extend(
            vec![
                ("vesting_denom", &to_string(&account.vesting_denom).unwrap()),
                ("vesting_amount", &account.vesting_amount.to_string()),
                ("vested_amount", &vested_amount.to_string()),
                ("claim_amount", &claimable_amount.to_string()),
            ]
            .into_iter()
            .map(|(key, val)| Attribute::new(key, val)),
        );
    }

    Ok(Response::new()
        .add_messages(messages)
        .add_attributes(vec![("action", "claim"), ("address", sender.as_str())])
        .add_attributes(attrs))
}

fn build_send_msg(
    denom: Denom,
    amount: Uint128,
    to: String,
) -> StdResult<CosmosMsg> {
    Ok(BankMsg::Send {
        to_address: to,
        amount: vec![Coin {
            denom: denom_to_key(denom),
            amount,
        }],
    }
    .into())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::VestingAccount {
            address,
            start_after,
            limit,
        } => to_json_binary(&vesting_account(
            deps,
            env,
            address,
            start_after,
            limit,
        )?),
    }
}

const MAX_LIMIT: u32 = 30;
const DEFAULT_LIMIT: u32 = 10;

/// address: Bech 32 address for the owner of the vesting accounts. This will be
///   the prefix we filter by in state.
/// limit: Maximum number of vesting accounts to retrieve when reading the
///   VESTING_ACCOUNTs store.
fn vesting_account(
    deps: Deps,
    env: Env,
    address: String,
    min_denom: Option<Denom>,
    limit: Option<u32>,
) -> StdResult<VestingAccountResponse> {
    let mut vestings: Vec<VestingData> = vec![];
    // Ensure the value of 'limit' does not exceed MAX_LIMIT
    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;

    for item in VESTING_ACCOUNTS
        .prefix(address.as_str())
        .range(
            deps.storage,
            min_denom
                .map(denom_to_key)
                .map(|s| s.as_bytes().to_vec())
                .map(Bound::ExclusiveRaw),
            None,
            Order::Ascending,
        )
        // limits the number of vesting accounts retrieved
        .take(limit)
    {
        let (_, account) = item?;
        let vested_amount = account
            .vesting_schedule
            .vested_amount(env.block.time.seconds())?;

        vestings.push(VestingData {
            master_address: account.master_address,
            vesting_denom: account.vesting_denom,
            vesting_amount: account.vesting_amount,
            vested_amount,
            vesting_schedule: account.vesting_schedule,
            claimable_amount: vested_amount
                .checked_sub(account.claimed_amount)?,
        })
    }

    Ok(VestingAccountResponse { address, vestings })
}

#[cfg(test)]
pub mod tests {

    use super::*;
    use anyhow::anyhow;
    use cosmwasm_std::{
        coin,
        testing::{self, MockApi, MockQuerier, MockStorage},
        Empty, OwnedDeps, Uint64,
    };

    pub type TestResult = Result<(), anyhow::Error>;

    pub fn mock_env_with_time(block_time: u64) -> Env {
        let mut env = testing::mock_env();
        env.block.time = Timestamp::from_seconds(block_time);
        env
    }

    /// Convenience function for instantiating the contract at and setting up
    /// the env to have the given block time.
    pub fn setup_with_block_time(
        block_time: u64,
    ) -> anyhow::Result<(OwnedDeps<MockStorage, MockApi, MockQuerier, Empty>, Env)>
    {
        let mut deps = testing::mock_dependencies();
        let env = mock_env_with_time(block_time);
        instantiate(
            deps.as_mut(),
            env.clone(),
            testing::mock_info("admin-sender", &[coin(5000, "token")]),
            InstantiateMsg {
                admin: "admin-sender".to_string(),
                managers: vec!["admin-sender".to_string()],
            },
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

        let register_msg = ExecuteMsg::RewardUsers {
            master_address: Some("addr0002".to_string()),
            rewards: vec![RewardUserRequest {
                user_address: "addr0001".to_string(),
                vesting_amount: Uint128::new(5000u128),
                cliff_amount: None,
            }],
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

        let register_msg = ExecuteMsg::RewardUsers {
            master_address: Some("addr0002".to_string()),
            rewards: vec![RewardUserRequest {
                user_address: "addr0001".to_string(),
                vesting_amount: Uint128::new(5000u128),
                cliff_amount: None,
            }],
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
}
