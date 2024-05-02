// use crate::contract::query::get_price;
use crate::{
    contract::query::get_count,
    error::ContractError,
    msgs::{ExecuteMsg, HelloResp, InstantiateMsg, QueryMsg},
    state::{State, COUNT},
};
use cosmwasm_std::{
    to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};
// use cw_ownable::{get_ownership, update_ownership, OwnershipError};

pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    // cw_ownable::initialize_owner(
    //     deps.storage,
    //     deps.api,
    //     Some(info.sender.clone().as_str()),
    // )?;

    let state = State { count: msg.count };
    COUNT.save(deps.storage, &state)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender)
        .add_attribute("count", msg.count.to_string()))
}

pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    use QueryMsg::*;

    match msg {
        HelloWorld {} => to_json_binary(&query::hello_world()?),
        // GetPrice { pair } => to_json_binary(&get_price(deps, pair)?),
        // Ownership {} => to_json_binary(&get_ownership(deps.storage)?),
        GetCount {} => to_json_binary(&get_count(deps)?),
    }
}

mod query {
    use super::*;

    // use nibiru_std::proto::{
    //     nibiru::oracle::QueryExchangeRateRequest, NibiruStargateQuery,
    // };

    use crate::msgs::GetCountResp;

    pub fn hello_world() -> StdResult<HelloResp> {
        let response = HelloResp {
            greeting: "Hello Nibiru Developers".to_owned(),
        };

        Ok(response)
    }

    // pub fn get_price(
    //     deps: Deps,
    //     pair: String,
    // ) -> Result<GetPriceResp, ContractError> {
    //     let query_request = QueryExchangeRateRequest { pair: pair.clone() };

    //     let query = query_request.into_stargate_query()?;

    //     todo!();
    //     let response: GetPriceResp = deps.querier.query(&query)?;

    //     Ok(response)
    // }

    pub fn get_count(deps: Deps) -> StdResult<GetCountResp> {
        let state = COUNT.load(deps.storage)?;

        Ok(GetCountResp { count: state.count })
    }
}

pub fn execute(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    use ExecuteMsg::*;

    match msg {
        // UpdateOwnership(action) => {
        //     update_ownership(deps, &env.block, &info.sender, action)?;
        // }
        Increment {} => todo!(),
        Reset { count: _ } => todo!(),
    }
}