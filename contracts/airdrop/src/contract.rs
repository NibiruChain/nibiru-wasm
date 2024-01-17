use crate::{
    msg::{
        ExecuteMsg, InstantiateMsg, QueryMsg, RewardUserRequest,
        RewardUserResponse,
    },
    state::{Campaign, CAMPAIGN, USER_REWARDS},
};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_json_binary, Addr, BankMsg, Binary, Coin, CosmosMsg, Deps, DepsMut,
    Empty, Env, MessageInfo, Response, StdError, StdResult, Uint128,
};
use cw2::{get_contract_version, set_contract_version};
use semver::Version;

pub const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
pub const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, StdError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    if info.funds.len() != 1 {
        return Err(StdError::generic_err("Only one coin is allowed"));
    }

    let bond_denom = deps.querier.query_bonded_denom()?;
    let coin = info.funds.get(0).unwrap();
    if coin.denom != bond_denom {
        return Err(StdError::generic_err("Only native tokens are allowed"));
    }

    let campaign = Campaign {
        campaign_name: msg.campaign_name,
        campaign_description: msg.campaign_description,
        owner: info.sender.clone(),
        managers: msg.managers,
        unallocated_amount: coin.amount,
        is_active: true,
    };
    CAMPAIGN.save(deps.storage, &campaign)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(
    deps: DepsMut,
    _env: Env,
    _msg: Empty,
) -> Result<Response, StdError> {
    let new_version: Version = CONTRACT_VERSION
        .parse()
        .map_err(|_| StdError::generic_err("Invalid contract version format"))?;
    let current_version = get_contract_version(deps.storage)?;

    if current_version.contract != CONTRACT_NAME {
        return Err(StdError::generic_err(
            "Can only upgrade from same contract type",
        ));
    }

    if current_version.version.parse::<Version>().unwrap() >= new_version {
        return Err(StdError::generic_err(
            "Cannot upgrade from a newer contract version",
        ));
    }

    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    Ok(Response::new().add_attribute("method", "migrate"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, StdError> {
    match msg {
        ExecuteMsg::RewardUsers { requests } => {
            reward_users(deps, env, info, requests)
        }
        ExecuteMsg::Claim {} => claim(deps, env, info),
        ExecuteMsg::Withdraw { amount } => withdraw(deps, env, info, amount),
        ExecuteMsg::DeactivateCampaign {} => deactivate(deps, env, info),
    }
}

/// Reward a set of users with native tokens from the campaign pool
///
/// - Requires sender to be the contract owner or a campaign manager.
/// - Ensures there are enough unallocated funds in the campaign
/// - Saves/updates user reward pool balances.
/// - Reduces the available campaign balance
pub fn reward_users(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    requests: Vec<RewardUserRequest>,
) -> Result<Response, StdError> {
    let mut res = vec![];

    let mut campaign = CAMPAIGN
        .load(deps.storage)
        .map_err(|_| StdError::generic_err("Failed to load campaign data"))?;

    if campaign.owner != info.sender && !campaign.managers.contains(&info.sender)
    {
        return Err(StdError::generic_err("Unauthorized"));
    }

    if !campaign.is_active {
        return Err(StdError::generic_err("Campaign is not active"));
    }

    for req in requests {
        if campaign.unallocated_amount < req.amount {
            return Err(StdError::generic_err(
                "Not enough funds in the campaign",
            ));
        }

        match USER_REWARDS.may_load(deps.storage, req.user_address.clone())? {
            Some(mut user_reward) => {
                user_reward += req.amount;
                USER_REWARDS.save(
                    deps.storage,
                    req.user_address.clone(),
                    &user_reward,
                )?;
            }
            None => {
                USER_REWARDS.save(
                    deps.storage,
                    req.user_address.clone(),
                    &req.amount,
                )?;
            }
        };
        campaign.unallocated_amount -= req.amount;
        CAMPAIGN.save(deps.storage, &campaign)?;

        res.push(RewardUserResponse {
            user_address: req.user_address.clone(),
            success: true,
            error_msg: "".to_string(),
        });
    }

    Ok(Response::new()
        .add_attribute("method", "reward_users")
        .set_data(to_json_binary(&res).unwrap()))
}

/// Allow a user to claim any rewards allocated to them
///
/// Transfers the user's full reward balance to their account. Resets their
/// reward balance to 0.
pub fn claim(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
) -> Result<Response, StdError> {
    let bond_denom = deps.querier.query_bonded_denom()?;

    let campaign = CAMPAIGN.load(deps.storage)?;

    if !campaign.is_active {
        return Err(StdError::generic_err("Campaign is not active"));
    }

    match USER_REWARDS.may_load(deps.storage, info.sender.clone())? {
        Some(user_reward) => {
            USER_REWARDS.remove(deps.storage, info.sender.clone());
            USER_REWARDS.remove(deps.storage, info.sender.clone());

            Ok(Response::new()
                .add_attribute("method", "claim")
                .add_message(CosmosMsg::Bank(BankMsg::Send {
                    to_address: info.sender.to_string(),
                    amount: vec![Coin {
                        denom: bond_denom.clone(),
                        amount: user_reward,
                    }],
                })))
        }
        None => Err(StdError::generic_err("User pool does not exist")),
    }
}

pub fn deactivate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
) -> Result<Response, StdError> {
    let mut campaign = CAMPAIGN
        .load(deps.storage)
        .map_err(|_| StdError::generic_err("Failed to load campaign data"))?;

    if campaign.owner != info.sender && !campaign.managers.contains(&info.sender)
    {
        return Err(StdError::generic_err("Unauthorized"));
    }

    if !campaign.is_active {
        return Ok(Response::new()
            .add_attribute("method", "deactivate")
            .add_attribute("message", "Campaign is already deactivated"));
    }

    campaign.is_active = false;
    CAMPAIGN.save(deps.storage, &campaign)?;

    let bond_denom = deps.querier.query_bonded_denom()?;
    let own_balance: Uint128 = deps
        .querier
        .query_balance(&env.contract.address, bond_denom.clone())
        .map_err(|_| StdError::generic_err("Failed to query contract balance"))?
        .amount;

    return withdraw(deps, env, info, own_balance);
}

/// Allow the contract owner to withdraw native tokens
///
/// Ensures the requested amount is available in the contract balance. Transfers
/// tokens to the contract owner's account.
pub fn withdraw(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    amount: Uint128,
) -> Result<Response, StdError> {
    let campaign = CAMPAIGN.load(deps.storage)?;

    if info.sender != campaign.owner {
        return Err(StdError::generic_err("Only contract owner can withdraw"));
    }

    let bond_denom = deps.querier.query_bonded_denom()?;

    let own_balance: Uint128 = deps
        .querier
        .query_balance(env.contract.address, bond_denom.clone())
        .map_err(|_| StdError::generic_err("Failed to query contract balance"))?
        .amount;

    if amount > own_balance {
        return Err(StdError::generic_err("Not enough funds in the contract"));
    }

    let res = Response::new()
        .add_attribute("method", "withdraw")
        .add_message(CosmosMsg::Bank(BankMsg::Send {
            to_address: info.sender.to_string(),
            amount: vec![Coin {
                denom: bond_denom.clone(),
                amount,
            }],
        }));

    // Update campaign unallocated amount
    if amount > campaign.unallocated_amount {
        CAMPAIGN.update(
            deps.storage,
            |mut campaign| -> StdResult<Campaign> {
                campaign.unallocated_amount = Uint128::zero();
                Ok(campaign)
            },
        )?;
    } else {
        CAMPAIGN.update(
            deps.storage,
            |mut campaign| -> StdResult<Campaign> {
                campaign.unallocated_amount -= amount;
                Ok(campaign)
            },
        )?;
    }

    return Ok(res);
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Campaign {} => query_campaign(deps, env),
        QueryMsg::GetUserReward { user_address } => {
            query_user_reward(deps, env, user_address)
        }
    }
}

pub fn query_campaign(deps: Deps, _env: Env) -> StdResult<Binary> {
    match CAMPAIGN.load(deps.storage) {
        Ok(campaign) => to_json_binary(&campaign),
        Err(_) => Err(StdError::generic_err("Failed to load campaign data")),
    }
}

pub fn query_user_reward(
    deps: Deps,
    _env: Env,
    user_address: Addr,
) -> StdResult<Binary> {
    let campaign = CAMPAIGN.load(deps.storage)?;
    if !campaign.is_active {
        return Err(StdError::generic_err("Campaign is not active"));
    }

    match USER_REWARDS.load(deps.storage, user_address) {
        Ok(user_reward) => to_json_binary(&user_reward),
        Err(_) => Err(StdError::generic_err("User reward does not exist")),
    }
}
