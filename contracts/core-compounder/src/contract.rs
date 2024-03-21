use broker_bank::contract::{
    assert_not_halted, edit_opers, execute_update_ownership, query_perms_status,
    toggle_halt, withdraw, withdraw_all,
};
use broker_bank::oper_perms::Permissions;
use broker_bank::state::{IS_HALTED, OPERATORS, TO_ADDRS};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_json_binary, Binary, Coin, CosmosMsg, Deps, DepsMut, Env, MessageInfo,
    Response, StakingMsg, StdResult, Uint128,
};

use crate::msg::{ExecuteMsg, StakeMsg, UnstakeMsg};
use broker_bank::error::ContractError;
use broker_bank::msgs::{
    InstantiateMsg as BrokerBankInstantiateMsg, PermsStatus, QueryMsg,
};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: BrokerBankInstantiateMsg,
) -> StdResult<Response> {
    // Managers validation
    cw_ownable::initialize_owner(deps.storage, deps.api, Some(&msg.owner))?;
    TO_ADDRS.save(deps.storage, &msg.to_addrs)?;
    OPERATORS.save(deps.storage, &msg.opers)?;
    IS_HALTED.save(deps.storage, &false)?;

    Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    let contract_addr = env.contract.address.to_string();
    match msg {
        ExecuteMsg::Withdraw { to, denoms } => {
            withdraw(deps, env, info, to, denoms, contract_addr)
        }
        ExecuteMsg::Stake { stake_msgs, amount } => {
            stake(deps, env, info, stake_msgs, amount)
        }
        ExecuteMsg::Unstake { unstake_msgs } => {
            unstake(deps, env, info, unstake_msgs)
        }
        ExecuteMsg::ToggleHalt {} => toggle_halt(deps, env, info),
        ExecuteMsg::UpdateOwnership(action) => {
            execute_update_ownership(deps, env, info, action)
        }
        ExecuteMsg::EditOpers(action) => edit_opers(deps, env, info, action),
        ExecuteMsg::WithdrawAll { to } => {
            withdraw_all(deps, env, info, to, contract_addr)
        }
    }
}

pub fn unstake(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    unstake_msgs: Vec<UnstakeMsg>,
) -> Result<Response, ContractError> {
    cw_ownable::assert_owner(deps.storage, &info.sender)?;

    let mut messages: Vec<CosmosMsg> = vec![];
    for msg in unstake_msgs.iter() {
        if msg.amount.is_zero() {
            continue;
        }
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
    Permissions::assert_operator(deps.storage, info.sender.to_string())?;
    let is_halted = IS_HALTED.load(deps.storage)?;
    assert_not_halted(is_halted)?;

    // sum total amount of shares in the stake msgs
    let total_shares: Uint128 = stake_msgs.iter().map(|m| m.share).sum();
    if total_shares.is_zero() {
        return Err(ContractError::Std(cosmwasm_std::StdError::generic_err(
            "total shares cannot be zero",
        )));
    }

    let mut messages: Vec<CosmosMsg> = vec![];
    let mut attrs: Vec<cosmwasm_std::Attribute> = vec![];
    for stake_msg in stake_msgs.iter() {
        let amount_to_delegate = amount * stake_msg.share / total_shares;
        if amount_to_delegate.is_zero() {
            continue;
        }

        messages.push(build_stake_message(
            amount_to_delegate,
            stake_msg.validator.to_string(),
        )?);
        attrs.push(cosmwasm_std::Attribute {
            key: "stake".to_string(),
            value: format!("{}:{}", stake_msg.validator, amount_to_delegate),
        });
    }

    Ok(Response::new()
        .add_messages(messages)
        .add_attribute("action", "stake")
        .add_attribute("amount", amount)
        .add_attributes(attrs))
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
        QueryMsg::IsHalted {} => {
            let is_halted = IS_HALTED.load(deps.storage)?;
            Ok(to_json_binary(&is_halted)?)
        }
    }
}
