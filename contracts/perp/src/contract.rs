use crate::error::ContractError;
use cosmwasm_std::{entry_point, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint64, Uint128, Decimal, to_binary};
use crate::msgs::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::bindings::{NibiruMsg, NibiruQuery, PositionResponse, PositionsResponse};
use crate::querier::{NibiruQuerier};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
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


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps<NibiruQuery>, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetPosition { 
            trader_address, 
            pair 
        } => to_binary(&get_position(deps, trader_address, pair)),
        QueryMsg::GetPositions { 
            trader_address, 
        } => to_binary(&get_positions(deps, trader_address)),
    }
}

fn get_position(deps: Deps<NibiruQuery>, trader_addr: String, pair: String) -> PositionResponse {
    let querier = NibiruQuerier::new(&deps.querier);
    querier.position(trader_addr, pair).unwrap()
}

fn get_positions(deps: Deps<NibiruQuery>, trader_addr: String) -> PositionsResponse {
    let querier = NibiruQuerier::new(&deps.querier);
    querier.positions(trader_addr).unwrap()
}