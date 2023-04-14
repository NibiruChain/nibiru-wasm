use cosmwasm_std::{entry_point, to_binary, Binary, CustomMsg, Deps, DepsMut, Env, StdResult};

use crate::query::{NibiruQuery};

/// These need not be the same. QueryMsg specifies a contract and module-specific
/// type for a query message, whereas NibiruQuery is an enum type for any of the
/// binding queries supported in NibiruChain/x/wasm/binding
///
/// In our case, there's only one module right now, so these are equivalent.
type QueryMsg = NibiruQuery;

impl CustomMsg for QueryMsg {}

#[entry_point]
pub fn inst(deps: DepsMut<NibiruQuery>, _env: Env, msg: QueryMsg) {
    // TODO
}

#[entry_point]
pub fn query(_deps: Deps<NibiruQuery>, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::AllMarkets {} => to_binary(&NibiruQuery::AllMarkets {}),
        QueryMsg::BasePrice {
            pair,
            is_long,
            base_amount,
        } => to_binary(&NibiruQuery::BasePrice {
            pair,
            is_long,
            base_amount,
        }),
        QueryMsg::Position { trader, pair } => {
            let cw_req = NibiruQuery::Position { trader, pair };
            return to_binary(&cw_req);
        }
        QueryMsg::Positions { trader } => {
            let cw_req = NibiruQuery::Positions { trader };
            return to_binary(&cw_req);
        }
        QueryMsg::Metrics { pair } => {
            let cw_req = NibiruQuery::Metrics { pair };
            return to_binary(&cw_req);
        }
        QueryMsg::ModuleAccounts {} => {
            let cw_req = NibiruQuery::ModuleAccounts {};
            return to_binary(&cw_req);
        }
        QueryMsg::ModuleParams {} => {
            let cw_req = NibiruQuery::ModuleParams {};
            return to_binary(&cw_req);
        }
        QueryMsg::PremiumFraction { pair } => {
            let cw_req = NibiruQuery::PremiumFraction { pair };
            return to_binary(&cw_req);
        }
        QueryMsg::Reserves { pair } => {
            let cw_req = NibiruQuery::Reserves { pair };
            return to_binary(&cw_req);
        }
    }
}

#[entry_point]
pub fn execute(deps: DepsMut<NibiruQuery>, _env: Env, msg: QueryMsg) {
    // TODO
}
