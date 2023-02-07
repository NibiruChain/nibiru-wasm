use crate::error::ContractError;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint64, Uint128, Decimal};
use crate::msgs::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::binding_msgs::{NibiruMsg};

/// Handling contract instantiation
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    Ok(Response::new())
}

/// Handling contract execution
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response<NibiruMsg>, ContractError> {
    match msg {
        ExecuteMsg::OpenPosition { 
            pair, 
            side, 
            quote_asset_amount, 
            leverage, 
            base_asset_amount_limit } => {
            return execute_open_position(pair, side, quote_asset_amount, leverage, base_asset_amount_limit);
        }

        ExecuteMsg::ClosePosition { pair } => {
            return execute_close_position(pair);
        }
    }
}

pub fn execute_open_position(pair: String, side: Uint64, quote_asset_amount: Uint128, leverage: Decimal, base_asset_amount_limit: Uint128) -> Result<Response<NibiruMsg>, ContractError> {
    if pair.eq("") {
        return Err(ContractError::InvalidPair(pair));
    }

    if quote_asset_amount.is_zero() {
        return Err(ContractError::InvalidQuoteAssetAmount(quote_asset_amount.to_string()));
    }

    if base_asset_amount_limit.is_zero() {
        return Err(ContractError::InvalidBaseAssetAmountLimit(base_asset_amount_limit.to_string()));
    }

    let open_position_msg = NibiruMsg::OpenPosition { pair, side, quote_asset_amount, leverage, base_asset_amount_limit };

    let res = Response::new()
        .add_attribute("method", "open_position")
        .add_message(open_position_msg);

    Ok(res)
}


pub fn execute_close_position(pair: String) -> Result<Response<NibiruMsg>, ContractError> {
    if pair.eq("") {
        return Err(ContractError::InvalidPair(pair));
    }

    let close_position_msg = NibiruMsg::ClosePosition { pair };

    let res = Response::new()
        .add_attribute("method", "close_position")
        .add_message(close_position_msg);

    Ok(res)
}


/// Handling contract query
pub fn query(_deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
    }
}
