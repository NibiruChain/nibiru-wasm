use cosmwasm_std::{
    entry_point, to_binary, Binary, CosmosMsg, CustomMsg, Deps, DepsMut, Env,
    MessageInfo, Response, StdResult,
};
use cw2::set_contract_version;

use crate::{
    msg::{
        msg_add_margin, msg_close_position, msg_donate_to_insurance_fund,
        msg_multi_liquidate, msg_open_position, msg_remove_margin, ExecuteMsg,
        InstantiateMsg, NibiruExecuteMsgWrapper,
    },
    querier::NibiruQuerier,
    query::NibiruQuery,
};

const CONTRACT_NAME: &str = "cw-nibiru-bindings-perp";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[entry_point]
pub fn instantiate(
    deps: DepsMut<NibiruQuery>,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    Ok(Response::new()
        .add_attribute("action", "instantiate")
        .add_attribute("owner", info.sender))
}

/// These need not be the same. QueryMsg specifies a contract and module-specific
/// type for a query message, whereas NibiruQuery is an enum type for any of the
/// binding queries supported in NibiruChain/x/wasm/binding
///
/// In our case, there's only one module right now, so NibiruQuery and QueryMsg
/// are equivalent.
type QueryMsg = NibiruQuery;

impl CustomMsg for QueryMsg {}

#[entry_point]
pub fn query(
    deps: Deps<NibiruQuery>,
    _env: Env,
    msg: QueryMsg,
) -> StdResult<Binary> {
    let querier = NibiruQuerier::new(&deps.querier);
    match msg {
        QueryMsg::AllMarkets {} => to_binary(&querier.all_markets().unwrap()),
        QueryMsg::BasePrice {
            pair,
            is_long,
            base_amount,
        } => to_binary(&querier.base_price(pair, is_long, base_amount).unwrap()),
        QueryMsg::Position { trader, pair } => {
            to_binary(&querier.position(trader, pair).unwrap())
        }
        QueryMsg::Positions { trader } => {
            to_binary(&querier.positions(trader).unwrap())
        }
        QueryMsg::Metrics { pair } => to_binary(&querier.metrics(pair).unwrap()),
        QueryMsg::ModuleAccounts {} => to_binary(&querier.module_accounts()?),
        QueryMsg::ModuleParams {} => to_binary(&querier.module_params()?),
        QueryMsg::PremiumFraction { pair } => {
            to_binary(&querier.premium_fraction(pair)?)
        }
        QueryMsg::Reserves { pair } => to_binary(&querier.reserves(pair)?),
    }
}

fn nibiru_msg_to_cw_response(
    cw_msg: CosmosMsg<NibiruExecuteMsgWrapper>,
) -> StdResult<Response<NibiruExecuteMsgWrapper>> {
    Ok(Response::new().add_message(cw_msg))
}

#[entry_point]
pub fn execute(
    _deps: DepsMut<NibiruQuery>,
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
