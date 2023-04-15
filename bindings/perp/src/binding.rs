use cosmwasm_std::{
    entry_point, to_binary, Binary, CustomMsg, Deps, DepsMut, Env, MessageInfo,
    Response, StdResult,
};
use cw2::set_contract_version;

use crate::{msg::InstantiateMsg, querier::NibiruQuerier, query::NibiruQuery};

/// These need not be the same. QueryMsg specifies a contract and module-specific
/// type for a query message, whereas NibiruQuery is an enum type for any of the
/// binding queries supported in NibiruChain/x/wasm/binding
///
/// In our case, there's only one module right now, so NibiruQuery and QueryMsg
/// are equivalent.
type QueryMsg = NibiruQuery;

impl CustomMsg for QueryMsg {}

const CONTRACT_NAME: &str = "cw-nibiru-bindings-perp";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[entry_point]
pub fn inst(
    deps: DepsMut<NibiruQuery>,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    return Ok(Response::new()
        .add_attribute("action", "instantiate")
        .add_attribute("owner", info.sender));
}

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
        QueryMsg::ModuleAccounts {} => {
            to_binary(&querier.module_accounts().unwrap())
        }
        QueryMsg::ModuleParams {} => {
            to_binary(&querier.module_params().unwrap())
        }
        QueryMsg::PremiumFraction { pair } => {
            to_binary(&querier.premium_fraction(pair).unwrap())
        }
        QueryMsg::Reserves { pair } => {
            to_binary(&querier.reserves(pair).unwrap())
        }
    }
}

#[entry_point]
pub fn execute(deps: DepsMut<NibiruQuery>, _env: Env, msg: QueryMsg) {
    // TODO
}
