use cosmwasm_std::{
    entry_point, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response,
    StdResult,
};

use cw2::set_contract_version;

use nibiru_bindings::querier::NibiruQuerier;
use nibiru_bindings::query::QueryPerpMsg;

use crate::{
    msg::{
        nibiru_msg_to_cw_response, ExecuteMsg, InitMsg, NibiruExecuteMsg,
        QueryMsg, SudoersQueryResponse,
    },
    state::{SUDOERS, Sudoers},
};

const CONTRACT_NAME: &str = "cw-nibiru-bindings-perp";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InitMsg,
) -> StdResult<Response> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    let admin: String = match msg.admin {
        Some(msg_admin) => { msg_admin }
        None => { info.sender.to_string().clone() }
    };  
    let sudoers = Sudoers {
        members: vec![admin.clone()].into_iter().collect(),
        admin: admin,
    };
    SUDOERS.save(deps.storage, &sudoers)?;
    Ok(Response::new()
        .add_attribute("action", "instantiate")
        .add_attribute("owner", info.sender))
}

// TODO test
#[entry_point]
pub fn query(
    deps: Deps<QueryPerpMsg>,
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
        // TODO test
        QueryMsg::Sudoers {} => {
            let sudoers = SUDOERS.load(deps.storage)?;
            let res = SudoersQueryResponse { sudoers };
            cosmwasm_std::to_binary(&res)
        }
    }
}

// TODO test
#[entry_point]
pub fn execute(
    _deps: DepsMut<QueryPerpMsg>,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response<NibiruExecuteMsg>> {
    match msg {
        ExecuteMsg::OpenPosition {
            pair,
            is_long,
            quote_amount,
            leverage,
            base_amount_limit,
        } => nibiru_msg_to_cw_response(NibiruExecuteMsg::open_position(
            pair,
            is_long,
            quote_amount,
            leverage,
            base_amount_limit,
        )),

        ExecuteMsg::ClosePosition { pair } => {
            nibiru_msg_to_cw_response(NibiruExecuteMsg::close_position(pair))
        }

        ExecuteMsg::AddMargin { pair, margin } => nibiru_msg_to_cw_response({
            NibiruExecuteMsg::add_margin(pair, margin)
        }),

        ExecuteMsg::RemoveMargin { pair, margin } => nibiru_msg_to_cw_response(
            NibiruExecuteMsg::remove_margin(pair, margin),
        ),

        ExecuteMsg::MultiLiquidate { pair, liquidations } => {
            nibiru_msg_to_cw_response(NibiruExecuteMsg::multi_liquidate(
                pair,
                liquidations,
            ))
        }

        ExecuteMsg::DonateToInsuranceFund { donation } => {
            nibiru_msg_to_cw_response(
                NibiruExecuteMsg::donate_to_insurance_fund(donation),
            )
        }

        ExecuteMsg::NoOp {} => {
            nibiru_msg_to_cw_response(NibiruExecuteMsg::no_op())
        }
    }
}

#[cfg(test)]
pub mod tests {
    use cosmwasm_std::{testing, coins};

    use super::*;

    #[test]
    fn msg_init() {
        let mut deps = testing::mock_dependencies();
        let admin = "admin";
        let msg = InitMsg {
            admin: Some(admin.to_string()),
        };
        let sender  = "sender";
        let info: MessageInfo =
            testing::mock_info(sender, &coins(2, "token"));

        let result =
            instantiate(deps.as_mut(), testing::mock_env(), info, msg).unwrap();
        assert_eq!(result.messages.len(), 0);

        let sudoers = SUDOERS.load(&deps.storage).unwrap();
        assert_eq!(sudoers.admin, admin)
    }

    #[test]
    fn msg_init_admin_as_sender() {
        let mut deps = testing::mock_dependencies();
        let msg = InitMsg { admin: None };
        let sender = "sender";
        let info: MessageInfo =
            testing::mock_info(sender, &coins(2, "token"));

        let result =
            instantiate(deps.as_mut(), testing::mock_env(), info, msg).unwrap();
        assert_eq!(result.messages.len(), 0);
        
        let sudoers = SUDOERS.load(&deps.storage).unwrap();
        assert_eq!(sudoers.admin, sender)
    }
}