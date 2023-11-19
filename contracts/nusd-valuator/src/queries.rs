use cosmwasm_std::{
    to_json_binary, Binary, Coin, Deps, Env, StdResult, Uint128,
};
use std::collections::BTreeSet;

use crate::msgs::QueryMsg;
use crate::state::ACCEPTED_DENOMS;

#[cfg_attr(not(feature = "library"), cosmwasm_std::entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Mintable { from_coins } => {
            to_json_binary(&query_mintable(deps, from_coins)?)
        }
        QueryMsg::Redeemable {
            redeem_amount,
            to_denom,
        } => to_json_binary(&query_redeemable(
            deps,
            redeem_amount,
            to_denom.as_str(),
        )?),
        QueryMsg::AcceptedDenoms {} => {
            to_json_binary(&query_accepted_denoms(deps)?)
        }
        QueryMsg::RedeemableChoices { redeem_amount } => {
            to_json_binary(&query_redeemable_choices(deps, redeem_amount)?)
        }
        QueryMsg::Ownership {} => {
            to_json_binary(&cw_ownable::get_ownership(deps.storage)?)
        }
    }
}

pub fn query_accepted_denoms(deps: Deps) -> StdResult<BTreeSet<String>> {
    ACCEPTED_DENOMS.load(deps.storage)
}

// TODO: query_mintable
pub fn query_mintable(
    _deps: Deps,
    _from_coins: BTreeSet<String>,
) -> StdResult<Uint128> {
    todo!()
}

// TODO: query_redeemable
pub fn query_redeemable(
    _deps: Deps,
    _redeem_amount: Uint128,
    _to_denom: &str,
) -> StdResult<Uint128> {
    todo!()
}

pub fn query_redeemable_choices(
    deps: Deps,
    redeem_amount: Uint128,
) -> StdResult<Vec<Coin>> {
    let accepted_denoms = query_accepted_denoms(deps)?;
    let choices: StdResult<Vec<Coin>> = accepted_denoms
        .iter()
        .map(|denom| {
            Ok(Coin {
                denom: denom.clone(),
                amount: query_redeemable(deps, redeem_amount, denom)?,
            })
        })
        .collect();
    choices
}