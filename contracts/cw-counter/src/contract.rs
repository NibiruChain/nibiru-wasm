use cosmwasm_std::{
    entry_point, Binary, DepsMut, Empty, Env, MessageInfo, Response, StdResult,
};

/// Instantiate the counter and add owner
#[entry_point]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: Empty,
) -> StdResult<Response> {
    Ok(Response::new())
}

/// greeting health
/// see where is the counter at
// #[entry_point]
// pub fn query(_deps: DepsMut, _env: Env, _msg: Empty) -> StdResult<Binary> {}
