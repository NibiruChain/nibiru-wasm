#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_json_binary, BankMsg, Binary, Coin, CosmosMsg, Deps, DepsMut, Env,
    MessageInfo, Response, StakingMsg, StdError, StdResult, Uint128,
};

use crate::errors::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, StakeMsg, UnstakeMsg};
use crate::state::{Whitelist, COMPOUNDER_ON, WHITELIST};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    // Managers validation
    if msg.managers.is_empty() {
        return Err(StdError::generic_err("managers cannot be empty").into());
    }

    deps.api.addr_validate(&msg.admin)?;
    for manager in msg.managers.iter() {
        let _ = deps.api.addr_validate(manager)?;
    }

    WHITELIST.save(
        deps.storage,
        &Whitelist {
            managers: msg.managers.into_iter().collect(),
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
        ExecuteMsg::SetAutocompounderMode {
            autocompounder_mode,
        } => set_autocompounder_mode(deps, env, info, autocompounder_mode),
        ExecuteMsg::Withdraw { amount, recipient } => {
            withdraw(deps, env, info, amount, recipient)
        }
        ExecuteMsg::Stake { stake_msgs, amount } => {
            stake(deps, env, info, stake_msgs, amount)
        }
        ExecuteMsg::Unstake { unstake_msgs } => {
            unstake(deps, env, info, unstake_msgs)
        }
        ExecuteMsg::UpdateManagers { managers } => {
            update_managers(deps, info, managers)
        }
    }
}

/// Admin functions
pub fn update_managers(
    deps: DepsMut,
    info: MessageInfo,
    managers: Vec<String>,
) -> Result<Response, ContractError> {
    let whitelist = WHITELIST.load(deps.storage)?;
    if !whitelist.is_admin(info.sender) {
        return Err(ContractError::Unauthorized {});
    }

    for manager in managers.iter() {
        let _ = deps.api.addr_validate(manager)?;
    }

    WHITELIST.save(
        deps.storage,
        &Whitelist {
            managers: managers.into_iter().collect(),
            admin: whitelist.admin,
        },
    )?;

    Ok(Response::new())
}

pub fn set_autocompounder_mode(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    autocompounder_mode: bool,
) -> Result<Response, ContractError> {
    let whitelist = WHITELIST.load(deps.storage)?;
    if !whitelist.is_admin(info.sender) {
        return Err(ContractError::Unauthorized {});
    }

    COMPOUNDER_ON.save(deps.storage, &autocompounder_mode)?;

    Ok(Response::new())
}

pub fn withdraw(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    amount: Uint128,
    recipient: String,
) -> Result<Response, ContractError> {
    let whitelist = WHITELIST.load(deps.storage)?;
    if !whitelist.is_admin(info.sender) {
        return Err(ContractError::Unauthorized {});
    }

    let messages: Vec<CosmosMsg> = vec![];
    send_if_amount_is_not_zero(
        &mut messages.clone(),
        amount,
        env.contract.address.to_string(),
        Some(recipient.clone()),
        env.contract.address.to_string(),
    )?;

    Ok(Response::new()
        .add_messages(messages)
        .add_attribute("action", "withdraw")
        .add_attribute("recipient", recipient)
        .add_attribute("amount", amount))
}

pub fn unstake(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    unstake_msgs: Vec<UnstakeMsg>,
) -> Result<Response, ContractError> {
    let whitelist = WHITELIST.load(deps.storage)?;
    if !whitelist.is_admin(info.sender) {
        return Err(ContractError::Unauthorized {});
    }

    let mut messages: Vec<CosmosMsg> = vec![];
    for msg in unstake_msgs.iter() {
        messages.push(build_unstakes_messages(
            msg.amount,
            msg.validator.to_string(),
        )?);
    }

    Ok(Response::new()
        .add_messages(messages)
        .add_attribute("action", "unstake"))
}

/// Managers functions

pub fn stake(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    stake_msgs: Vec<StakeMsg>,
    amount: Uint128,
) -> Result<Response, ContractError> {
    let sender = info.sender.clone();

    let whitelist = WHITELIST.load(deps.storage)?;
    if !whitelist.is_manager(sender.clone()) && !whitelist.is_admin(sender) {
        return Err(ContractError::Unauthorized {});
    }

    // sum total amount of shares in the stake msgs
    let total_shares: Uint128 = stake_msgs.iter().map(|m| m.share).sum();
    if total_shares.is_zero() {
        return Err(ContractError::InvalidStakeShares {});
    }

    let mut messages: Vec<CosmosMsg> = vec![];
    for stake_msg in stake_msgs.iter() {
        let _ = deps.api.addr_validate(&stake_msg.validator)?;

        let amount_to_delegate = amount * stake_msg.share / total_shares;

        messages.push(build_stake_message(
            amount_to_delegate,
            stake_msg.validator.to_string(),
        )?);
    }

    Ok(Response::new()
        .add_messages(messages)
        .add_attribute("action", "stake")
        .add_attribute("amount", amount))
}

///
/// creates a send message if the amount to send is not zero
///
/// If we provide a recipient, we use it. Otherwise, we use the default_recipient
fn send_if_amount_is_not_zero(
    messages: &mut Vec<CosmosMsg>,
    amount: Uint128,
    denom: String,
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

fn build_send_msg(
    denom: String,
    amount: Uint128,
    to: String,
) -> StdResult<CosmosMsg> {
    Ok(BankMsg::Send {
        to_address: to,
        amount: vec![Coin {
            denom: denom,
            amount,
        }],
    }
    .into())
}

fn build_stake_message(
    amount: Uint128,
    validator: String,
) -> StdResult<CosmosMsg> {
    Ok(CosmosMsg::Staking(StakingMsg::Delegate {
        validator: validator,
        amount: Coin {
            denom: "unibi".to_string(),
            amount,
        },
    }))
}

fn build_unstakes_messages(
    amount: Uint128,
    validator: String,
) -> StdResult<CosmosMsg> {
    Ok(CosmosMsg::Staking(StakingMsg::Undelegate {
        validator: validator,
        amount: Coin {
            denom: "unibi".to_string(),
            amount,
        },
    }))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::AutocompounderMode {} => {
            to_json_binary(&query_autocompounder_mode(deps, env)?)
        }
        QueryMsg::AdminAndManagers {} => {
            to_json_binary(&query_admin_and_managers(deps)?)
        }
    }
}

pub fn query_autocompounder_mode(deps: Deps, _env: Env) -> StdResult<bool> {
    COMPOUNDER_ON.load(deps.storage)
}

pub fn query_admin_and_managers(deps: Deps) -> StdResult<Whitelist> {
    WHITELIST.load(deps.storage)
}
