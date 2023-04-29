use cosmwasm_std::{
    entry_point, to_binary, Binary, CosmosMsg, Deps, DepsMut, Env,
    MessageInfo, Response, StdResult,
};

use cw2::set_contract_version;

use nibiru_bindings::query::QueryPerpMsg;
use nibiru_bindings::querier::NibiruQuerier;

use crate::{
    msg::{
        msg_add_margin, msg_close_position, msg_donate_to_insurance_fund,
        msg_multi_liquidate, msg_open_position, msg_remove_margin, ExecuteMsg,
        InstantiateMsg, NibiruExecuteMsgWrapper,
    },
};

const CONTRACT_NAME: &str = "cw-nibiru-bindings-perp";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[entry_point]
pub fn instantiate(
    deps: DepsMut<QueryPerpMsg>,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    Ok(Response::new()
        .add_attribute("action", "instantiate")
        .add_attribute("owner", info.sender))
}

#[entry_point]
pub fn query(
    deps: Deps<QueryPerpMsg>,
    _env: Env,
    msg: QueryPerpMsg,
) -> StdResult<Binary> {
    let querier = NibiruQuerier::new(&deps.querier);
    match msg {
        QueryPerpMsg::AllMarkets {} => to_binary(&querier.all_markets().unwrap()),
        QueryPerpMsg::BasePrice {
            pair,
            is_long,
            base_amount,
        } => to_binary(&querier.base_price(pair, is_long, base_amount).unwrap()),
        QueryPerpMsg::Position { trader, pair } => {
            to_binary(&querier.position(trader, pair).unwrap())
        }
        QueryPerpMsg::Positions { trader } => {
            to_binary(&querier.positions(trader).unwrap())
        }
        QueryPerpMsg::Metrics { pair } => to_binary(&querier.metrics(pair).unwrap()),
        QueryPerpMsg::ModuleAccounts {} => to_binary(&querier.module_accounts()?),
        QueryPerpMsg::ModuleParams {} => to_binary(&querier.module_params()?),
        QueryPerpMsg::PremiumFraction { pair } => {
            to_binary(&querier.premium_fraction(pair)?)
        }
        QueryPerpMsg::Reserves { pair } => to_binary(&querier.reserves(pair)?),
    }
}

fn nibiru_msg_to_cw_response(
    cw_msg: CosmosMsg<NibiruExecuteMsgWrapper>,
) -> StdResult<Response<NibiruExecuteMsgWrapper>> {
    Ok(Response::new().add_message(cw_msg))
}

#[entry_point]
pub fn execute(
    _deps: DepsMut<QueryPerpMsg>,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response<NibiruExecuteMsgWrapper>> {
    match msg {
        ExecuteMsg::OpenPosition {
            pair,
            is_long,
            quote_amount,
            leverage,
            base_amount_limit,
        } => nibiru_msg_to_cw_response(msg_open_position(
            pair,
            is_long,
            quote_amount,
            leverage,
            base_amount_limit,
        )),
        ExecuteMsg::ClosePosition { sender, pair } => {
            nibiru_msg_to_cw_response(msg_close_position(sender, pair))
        }
        ExecuteMsg::AddMargin {
            sender,
            pair,
            margin,
        } => nibiru_msg_to_cw_response(msg_add_margin(sender, pair, margin)),
        ExecuteMsg::RemoveMargin {
            sender,
            pair,
            margin,
        } => nibiru_msg_to_cw_response(msg_remove_margin(sender, pair, margin)),
        ExecuteMsg::MultiLiquidate { pair, liquidations } => {
            nibiru_msg_to_cw_response(msg_multi_liquidate(pair, liquidations))
        }
        ExecuteMsg::DonateToInsuranceFund { sender, donation } => {
            nibiru_msg_to_cw_response(msg_donate_to_insurance_fund(
                sender, donation,
            ))
        }
    }
}
