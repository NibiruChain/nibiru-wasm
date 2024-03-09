#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_json_binary, Attribute, BankMsg, Binary, Coin, CosmosMsg, Deps, DepsMut,
    Env, MessageInfo, Response, StdError, StdResult, Storage, Uint128,
};
use std::cmp::min;

use serde_json::to_string;

use crate::errors::ContractError;
use crate::msg::{
    from_vesting_to_query_output, DeregisterUserResponse, ExecuteMsg,
    InstantiateMsg, QueryMsg, RewardUserRequest, RewardUserResponse,
    VestingAccountResponse, VestingData, VestingSchedule,
};
use crate::state::{
    VestingAccount, Whitelist, DENOM, UNALLOCATED_AMOUNT, VESTING_ACCOUNTS,
    WHITELIST,
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
        ));
    }
    if info.funds[0].amount.is_zero() {
        return Err(StdError::generic_err("must deposit some token"));
    }
    // Managers validation
    if msg.managers.is_empty() {
        return Err(StdError::generic_err("managers cannot be empty"));
    }

    deps.api.addr_validate(&msg.admin)?;
    for manager in msg.managers.iter() {
        deps.api.addr_validate(manager)?;
    }

    let unallocated_amount = info.funds[0].amount;
    let denom = &info.funds[0].denom;

    UNALLOCATED_AMOUNT.save(deps.storage, &unallocated_amount)?;
    DENOM.save(deps.storage, denom)?;
    WHITELIST.save(
        deps.storage,
        &Whitelist {
            members: msg.managers.into_iter().collect(),
            admin: msg.admin,
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
            vesting_schedule,
        } => reward_users(deps, env, info, rewards, vesting_schedule),
        ExecuteMsg::DeregisterVestingAccounts { addresses } => {
            deregister_vesting_accounts(deps, env, info, addresses)
        }
        ExecuteMsg::Claim {
            denoms: _denoms,
            recipient,
        } => claim(deps, env, info, recipient),
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

    if !whitelist.is_admin(info.sender) {
        return Err(StdError::generic_err("Unauthorized").into());
    }

    let amount_max = min(amount, unallocated_amount);
    if amount_max.is_zero() {
        return Err(StdError::generic_err("Nothing to withdraw").into());
    }

    unallocated_amount -= amount_max;
    UNALLOCATED_AMOUNT.save(deps.storage, &unallocated_amount)?;

    // validate recipient address
    deps.api.addr_validate(&recipient)?;

    Ok(Response::new()
        .add_messages(vec![build_send_msg(&denom, amount_max, &recipient)])
        .add_attribute("action", "withdraw")
        .add_attribute("recipient", &recipient)
        .add_attribute("amount", amount_max.to_string())
        .add_attribute("unallocated_amount", unallocated_amount.to_string()))
}

fn reward_users(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    rewards: Vec<RewardUserRequest>,
    vesting_schedule: VestingSchedule,
) -> Result<Response, ContractError> {
    let mut res = vec![];

    let whitelist = WHITELIST.load(deps.storage)?;

    if !(whitelist.is_member(&info.sender) || whitelist.is_admin(&info.sender)) {
        return Err(StdError::generic_err("Unauthorized").into());
    }

    let unallocated_amount = UNALLOCATED_AMOUNT.load(deps.storage)?;

    let total_requested: Uint128 =
        rewards.iter().map(|req| req.vesting_amount).sum();
    if total_requested > unallocated_amount {
        return Err(StdError::generic_err(format!(
            "Insufficient funds for all rewards. Have {} but want {}",
            unallocated_amount, total_requested
        ))
        .into());
    }
    vesting_schedule.validate()?;

    let mut attrs: Vec<Attribute> = vec![];
    for req in rewards {
        // validate amounts and cliff details if there's one
        req.validate()?;

        let result = register_vesting_account(
            deps.storage,
            &req.user_address,
            req.vesting_amount,
            req.cliff_amount,
            &vesting_schedule,
        );

        match result {
            Ok(response) => {
                attrs.extend(response.attributes);
                res.push(RewardUserResponse {
                    user_address: req.user_address,
                    success: true,
                    error_msg: "".to_string(),
                });
            }
            Err(error) => {
                res.push(RewardUserResponse {
                    user_address: req.user_address,
                    success: false,
                    error_msg: format!(
                        "Failed to register vesting account: {}",
                        error
                    ),
                });
            }
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
    address: &str,
    vesting_amount: Uint128,
    cliff_amount: Uint128,
    vesting_schedule: &VestingSchedule,
) -> Result<Response, ContractError> {
    // vesting_account existence check
    if VESTING_ACCOUNTS.has(storage, address) {
        return Err(StdError::generic_err("already exists").into());
    }
    vesting_schedule.validate()?;

    VESTING_ACCOUNTS.save(
        storage,
        address,
        &VestingAccount {
            address: address.to_string(),
            vesting_amount,
            cliff_amount,
            vesting_schedule: vesting_schedule.clone(),
            claimed_amount: Uint128::zero(),
        },
    )?;

    Ok(Response::new().add_attributes(vec![
        ("action", "register_vesting_account"),
        ("address", address),
        ("vesting_amount", &vesting_amount.to_string()),
    ]))
}

fn deregister_vesting_accounts(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    addresses: Vec<String>,
) -> Result<Response, ContractError> {
    let whitelist = WHITELIST.load(deps.storage)?;
    if !(whitelist.is_member(&info.sender) || whitelist.is_admin(&info.sender)) {
        return Err(StdError::generic_err("Unauthorized").into());
    }
    
    let mut res = vec![];
    let mut attrs: Vec<Attribute> = vec![];

    for address in addresses {
        let result = deregister_vesting_account(
            deps.storage,
            env.block.time.seconds(),
            &address,
            &whitelist.admin,
        );

        match result {
            Ok(response) => {
                attrs.extend(response.attributes);
                res.push(DeregisterUserResponse {
                    user_address: address,
                    success: true,
                    error_msg: "".to_string(),
                });
            }
            Err(error) => {
                res.push(DeregisterUserResponse {
                    user_address: address,
                    success: false,
                    error_msg: format!(
                        "Failed to deregister vesting account: {}",
                        error
                    ),
                });
            }
        }
    }
    
    Ok(Response::new()
        .add_attributes(attrs)
        .add_attribute("action", "deregister_vesting_accounts")
        .set_data(to_json_binary(&res).unwrap()))
}

fn deregister_vesting_account(
    storage: &mut dyn Storage,
    timestamp: u64,
    address: &str,
    admin_address: &str,
) -> Result<Response, ContractError> {
    let mut messages: Vec<CosmosMsg> = vec![];

    // vesting_account existence check
    let account = VESTING_ACCOUNTS.may_load(storage, address)?;
    let denom = DENOM.load(storage)?;

    if account.is_none() {
        return Err(ContractError::Std(StdError::generic_err(format!(
            "vesting entry is not found for address {:?}",
            to_string(&address).unwrap(),
        ))));
    }
    let account = account.unwrap();

    // remove vesting account
    VESTING_ACCOUNTS.remove(storage, address);

    let vested_amount = account.vested_amount(timestamp)?;
    let claimed_amount = account.claimed_amount;

    // transfer already vested amount to vested_token_recipient and if
    // it is not provided, transfer it to the address that is the owner of the vesting account
    let claimable_amount = vested_amount.checked_sub(claimed_amount)?;
    send_if_amount_is_not_zero(
        &mut messages,
        claimable_amount,
        &denom,
        address,
    )?;

    // transfer left vesting amount to left_vesting_token_recipient and if
    // it is not provided, transfer it to the master_address
    let left_vesting_amount =
        account.vesting_amount.checked_sub(vested_amount)?;
    send_if_amount_is_not_zero(
        &mut messages,
        left_vesting_amount,
        &denom,
        admin_address,
    )?;

    Ok(Response::new().add_messages(messages).add_attributes(vec![
        ("action", "deregister_vesting_account"),
        ("address", address),
        ("vesting_amount", &account.vesting_amount.to_string()),
        ("vested_amount", &vested_amount.to_string()),
        ("left_vesting_amount", &left_vesting_amount.to_string()),
    ]))
}

///
/// creates a send message if the amount to send is not zero
///
fn send_if_amount_is_not_zero(
    messages: &mut Vec<CosmosMsg>,
    amount: Uint128,
    denom: &str,
    recipient: &str,
) -> Result<(), ContractError> {
    if !amount.is_zero() {
        messages.push(build_send_msg(denom, amount, recipient));
    }

    Ok(())
}

fn claim(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    recipient: Option<String>,
) -> Result<Response, ContractError> {
    let sender = info.sender;
    let recipient = recipient.unwrap_or_else(|| sender.to_string());
    let denom = DENOM.load(deps.storage)?;

    let mut attrs: Vec<Attribute> = vec![];

    // vesting_account existence check
    let account = VESTING_ACCOUNTS.may_load(deps.storage, sender.as_str())?;
    if account.is_none() {
        return Err(StdError::generic_err(format!(
            "vesting entry is not found for denom {}",
            to_string(&denom).unwrap(),
        ))
        .into());
    }

    let mut account = account.unwrap();
    let vested_amount = account.vested_amount(env.block.time.seconds())?;
    let claimed_amount = account.claimed_amount;

    let claimable_amount = vested_amount.checked_sub(claimed_amount)?;
    if claimable_amount.is_zero() {
        return Err(StdError::generic_err("nothing left to claim").into());
    }

    account.claimed_amount = vested_amount;
    if account.claimed_amount == account.vesting_amount {
        VESTING_ACCOUNTS.remove(deps.storage, sender.as_str());
    } else {
        VESTING_ACCOUNTS.save(deps.storage, sender.as_str(), &account)?;
    }

    attrs.extend(
        vec![
            ("vesting_amount", &account.vesting_amount.to_string()),
            ("vested_amount", &vested_amount.to_string()),
            ("claim_amount", &claimable_amount.to_string()),
        ]
        .into_iter()
        .map(|(key, val)| Attribute::new(key, val)),
    );

    Ok(Response::new()
        .add_messages(vec![build_send_msg(&denom, claimable_amount, &recipient)])
        .add_attributes(vec![("action", "claim"), ("address", sender.as_str())])
        .add_attributes(attrs))
}

fn build_send_msg(
    denom: &str,
    amount: Uint128,
    to: &str,
) -> CosmosMsg {
    BankMsg::Send {
        to_address: to.to_string(),
        amount: vec![Coin {
            denom: denom.to_string(),
            amount,
        }],
    }.into()
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::VestingAccount {
            address,
            start_after: _start_after,
            limit: _limit,
        } => to_json_binary(&vesting_account(deps, env, address)?),
    }
}

/// address: Bech 32 address for the owner of the vesting accounts. This will be
///   the prefix we filter by in state.
/// limit: Maximum number of vesting accounts to retrieve when reading the
///   VESTING_ACCOUNTs store.
fn vesting_account(
    deps: Deps,
    env: Env,
    address: String,
) -> StdResult<VestingAccountResponse> {
    let account = VESTING_ACCOUNTS.may_load(deps.storage, address.as_str())?;
    let whitelist = WHITELIST.load(deps.storage)?;
    let denom = DENOM.load(deps.storage)?;

    match account {
        None => Ok(VestingAccountResponse {
            address,
            vestings: vec![],
        }),
        Some(account) => {
            let vested_amount =
                account.vested_amount(env.block.time.seconds())?;

            let vesting_schedule_query = from_vesting_to_query_output(
                &account.vesting_schedule,
                account.vesting_amount,
                account.cliff_amount,
            );

            let vesting = VestingData {
                master_address: Some(whitelist.admin.clone()),
                vesting_denom: cw20::Denom::Native(denom),
                vesting_amount: account.vesting_amount,
                vesting_schedule: vesting_schedule_query,

                vested_amount,
                claimable_amount: vested_amount
                    .checked_sub(account.claimed_amount)?,
            };

            Ok(VestingAccountResponse {
                address,
                vestings: vec![vesting],
            })
        }
    }
}
