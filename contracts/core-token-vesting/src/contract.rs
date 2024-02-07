#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    from_json, to_json_binary, Attribute, BankMsg, Binary, Coin, CosmosMsg,
    Deps, DepsMut, Env, MessageInfo, Order, Response, StdError, StdResult,
    Storage, Timestamp, Uint128, WasmMsg,
};

use serde_json::to_string;

use cw20::{Cw20ExecuteMsg, Cw20ReceiveMsg, Denom};
use cw_storage_plus::Bound;

use crate::errors::ContractError;
use crate::msg::{
    Cw20HookMsg, ExecuteMsg, InstantiateMsg, QueryMsg, RewardUserRequest,
    RewardUserResponse, VestingAccountResponse, VestingData, VestingSchedule,
};
use crate::state::{
    denom_to_key, Campaign, VestingAccount, CAMPAIGN, USER_REWARDS,
    VESTING_ACCOUNTS,
};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response> {
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
        ExecuteMsg::Receive(msg) => {
            receive_cw20(deps, env, info, msg).map_err(ContractError::from)
        }
        ExecuteMsg::RegisterVestingAccount {
            master_address,
            address,
            vesting_schedule,
        } => {
            // deposit validation
            if info.funds.len() != 1 {
                return Err(StdError::generic_err(
                    "must deposit only one type of token",
                )
                .into());
            }

            let deposit_coin = info.funds[0].clone();
            register_vesting_account(
                deps.storage,
                env.block.time,
                master_address,
                address,
                Denom::Native(deposit_coin.denom),
                deposit_coin.amount,
                vesting_schedule,
            )
        }
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
        ExecuteMsg::CreateCampaign {
            vesting_schedule,
            campaign_id,
            campaign_name,
            campaign_description,
            managers,
        } => create_campaign(
            deps,
            env,
            info,
            vesting_schedule,
            campaign_id,
            campaign_name,
            campaign_description,
            managers,
        ),
        ExecuteMsg::RewardUsers {
            campaign_id,
            requests,
        } => reward_users(deps, env, info, campaign_id, requests),
        ExecuteMsg::ClaimCampaign { campaign_id } => {
            claim_campaign(deps, env, info, campaign_id)
        }
        ExecuteMsg::DeactivateCampaign { campaign_id } => {
            deactivate_campaign(deps, env, info, campaign_id)
        }
        ExecuteMsg::Withdraw {
            amount,
            campaign_id,
        } => withdraw(deps, env, info, amount, campaign_id.as_str()),
    }
}

/// Deactivate a campaign and withdraw all unallocated funds
/// This will also withdraw all unallocated funds from the contract
/// and send them to the campaign owner.
fn deactivate_campaign(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    campaign_id: String,
) -> Result<Response, ContractError> {
    let mut campaign = CAMPAIGN
        .load(deps.storage, campaign_id.clone())
        .map_err(|_| StdError::generic_err("Failed to load campaign data"))?;

    if campaign.owner != info.sender
        && !campaign.managers.contains(&info.sender.to_string())
    {
        return Err(StdError::generic_err("Unauthorized. Only the campaign owner or managers can deactivate the campaign").into());
    }

    if !campaign.is_active {
        return Ok(Response::new()
            .add_attribute("method", "deactivate")
            .add_attribute("message", "Campaign is already deactivated"));
    }

    campaign.is_active = false;
    CAMPAIGN.save(deps.storage, campaign_id.clone(), &campaign)?;

    let denom = to_string(&campaign.denom).unwrap();
    let own_balance: Uint128 = deps
        .querier
        .query_balance(&env.contract.address, denom)
        .map_err(|_| StdError::generic_err("Failed to query contract balance"))?
        .amount;

    return withdraw(deps, env, info, own_balance, campaign_id.as_str());
}

fn claim_campaign(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _campaign_id: String,
) -> Result<Response, ContractError> {
    todo!()
}

fn reward_users(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    campaign_id: String,
    requests: Vec<RewardUserRequest>,
) -> Result<Response, ContractError> {
    let mut res = vec![];

    let mut campaign = CAMPAIGN
        .load(deps.storage, campaign_id.clone())
        .map_err(|_| StdError::generic_err("Failed to load campaign data"))?;

    if campaign.owner != info.sender
        && !campaign.managers.contains(&info.sender.into())
    {
        return Err(StdError::generic_err("Unauthorized").into());
    }

    if !campaign.is_active {
        return Err(StdError::generic_err("Campaign is not active").into());
    }

    let total_requested: Uint128 = requests.iter().map(|req| req.amount).sum();
    if total_requested > campaign.unallocated_amount {
        return Err(
            StdError::generic_err("Insufficient funds for all rewards").into()
        );
    }
    for req in requests {
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
        res.push(RewardUserResponse {
            user_address: req.user_address.clone(),
            success: true,
            error_msg: "".to_string(),
        });
    }

    campaign.unallocated_amount = campaign.unallocated_amount - total_requested;
    CAMPAIGN.save(deps.storage, campaign_id.clone(), &campaign)?;

    Ok(Response::new()
        .add_attribute("method", "reward_users")
        .set_data(to_json_binary(&res).unwrap()))
}

fn create_campaign(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _vesting_schedule: VestingSchedule,
    campaign_id: String,
    campaign_name: String,
    campaign_description: String,
    managers: Vec<String>,
) -> Result<Response, ContractError> {
    if CAMPAIGN
        .may_load(deps.storage, campaign_id.clone())?
        .is_some()
    {
        return Err(StdError::generic_err("Campaign already exists").into());
    }

    if info.funds.len() != 1 {
        return Err(StdError::generic_err("one denom sent required").into());
    }

    let coin = info.funds.get(0).unwrap();

    let campaign = Campaign {
        campaign_name: campaign_name,
        campaign_id: campaign_id.clone(),
        campaign_description: campaign_description,
        owner: info.sender.into_string(),
        managers: managers,
        unallocated_amount: coin.amount,
        denom: Denom::Native(coin.denom.clone()),
        is_active: true,
    };
    CAMPAIGN.save(deps.storage, campaign_id.clone(), &campaign)?;

    Ok(Response::new()
        .add_attribute("method", "create_campaign")
        .add_attribute("campaign_id", campaign_id)
        .add_attribute("campaign_name", &campaign.campaign_name)
        .add_attribute("initial_unallocated_amount", &coin.amount.to_string()))
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

    // validate vesting schedule
    vesting_schedule.validate(block_time, deposit_amount)?;
    let vesting_account = VestingAccount {
        master_address: master_address.clone(),
        address: address.to_string(),
        vesting_denom: deposit_denom.clone(),
        vesting_amount: deposit_amount,
        vesting_schedule,
        claimed_amount: Uint128::zero(),
    };

    VESTING_ACCOUNTS.save(
        storage,
        (address.as_str(), &denom_key),
        &vesting_account,
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
    print!(
        "claimed_amount: {}",
        vested_amount.checked_sub(claimed_amount)?
    );

    let claimable_amount = vested_amount.checked_sub(claimed_amount)?;
    if !claimable_amount.is_zero() {
        let recipient =
            vested_token_recipient.unwrap_or_else(|| address.to_string());
        let msg_send: CosmosMsg = build_send_msg(
            account.vesting_denom.clone(),
            claimable_amount,
            recipient,
        )?;
        messages.push(msg_send);
    }

    // transfer left vesting amount to owner or
    // the given `left_vesting_token_recipient` address
    let left_vesting_amount =
        account.vesting_amount.checked_sub(vested_amount)?;
    if !left_vesting_amount.is_zero() {
        let recipient =
            left_vesting_token_recipient.unwrap_or_else(|| sender.to_string());
        let msg_send: CosmosMsg = build_send_msg(
            account.vesting_denom.clone(),
            left_vesting_amount,
            recipient,
        )?;
        messages.push(msg_send);
    }

    Ok(Response::new().add_messages(messages).add_attributes(vec![
        ("action", "deregister_vesting_account"),
        ("address", address.as_str()),
        ("vesting_denom", &to_string(&account.vesting_denom).unwrap()),
        ("vesting_amount", &account.vesting_amount.to_string()),
        ("vested_amount", &vested_amount.to_string()),
        ("left_vesting_amount", &left_vesting_amount.to_string()),
    ]))
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
    Ok(match denom {
        Denom::Native(denom) => BankMsg::Send {
            to_address: to,
            amount: vec![Coin { denom, amount }],
        }
        .into(),
        Denom::Cw20(contract_addr) => WasmMsg::Execute {
            contract_addr: contract_addr.to_string(),
            msg: to_json_binary(&Cw20ExecuteMsg::Transfer {
                recipient: to,
                amount,
            })?,
            funds: vec![],
        }
        .into(),
    })
}

pub fn receive_cw20(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    cw20_msg: Cw20ReceiveMsg,
) -> Result<Response, ContractError> {
    let amount = cw20_msg.amount;
    let _sender = cw20_msg.sender;
    let contract = info.sender;

    match from_json(&cw20_msg.msg) {
        Ok(Cw20HookMsg::RegisterVestingAccount {
            master_address,
            address,
            vesting_schedule,
        }) => register_vesting_account(
            deps.storage,
            env.block.time,
            master_address,
            address,
            Denom::Cw20(contract),
            amount,
            vesting_schedule,
        ),
        Err(_) => Err(StdError::generic_err("invalid cw20 hook message").into()),
    }
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

/// Allow the contract owner to withdraw the funds of the campaign
///
/// Ensures the requested amount is available in the contract balance. Transfers
/// tokens to the contract owner's account.
pub fn withdraw(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    amount: Uint128,
    campaign_id: &str,
) -> Result<Response, ContractError> {
    let campaign = CAMPAIGN.load(deps.storage, campaign_id.to_string())?;

    if info.sender != campaign.owner {
        return Err(
            StdError::generic_err("Only contract owner can withdraw").into()
        );
    }

    // Update campaign unallocated amount
    if amount > campaign.unallocated_amount {
        let update_result = CAMPAIGN.update(
            deps.storage,
            campaign_id.to_string(),
            |campaign| -> StdResult<Campaign> {
                if let Some(mut campaign) = campaign {
                    campaign.unallocated_amount = Uint128::zero();
                    Ok(campaign)
                } else {
                    Err(StdError::generic_err("Campaign not found"))
                }
            },
        );

        if let Err(e) = update_result {
            return Err(e.into());
        }
    } else {
        let update_result = CAMPAIGN.update(
            deps.storage,
            campaign_id.to_string(),
            |campaign| -> StdResult<Campaign> {
                if let Some(mut campaign) = campaign {
                    campaign.unallocated_amount -= amount;
                    Ok(campaign)
                } else {
                    Err(StdError::generic_err("Campaign not found"))
                }
            },
        );

        if let Err(e) = update_result {
            return Err(e.into());
        }
    }

    Ok(Response::new()
        .add_messages(vec![build_send_msg(
            campaign.denom,
            amount,
            info.sender.to_string(),
        )?])
        .add_attribute("withdraw", &amount.to_string())
        .add_attribute(
            "campaign_unallocated_amount",
            &campaign.unallocated_amount.to_string(),
        ))
}
