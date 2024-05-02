use crate::msg::{HelloResp, QueryMsg};
use cosmwasm_std::{
    to_json_binary, Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response,
    StdResult,
};

pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: Empty,
) -> StdResult<Response> {
    Ok(Response::new())
}

pub fn query(_deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    use QueryMsg::*;

    match msg {
        HelloWorld {} => to_json_binary(&query::hello_world()?),
    }
}

mod query {
    use super::*;

    pub fn hello_world() -> StdResult<HelloResp> {
        let response = HelloResp {
            greeting: "Hello Nibiru Developers".to_owned(),
        };

        Ok(response)
    }
}
