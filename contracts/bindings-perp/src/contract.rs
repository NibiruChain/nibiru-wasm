use cosmwasm_std::{
    entry_point, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response,
    StdResult,
};

use cw2::set_contract_version;

use nibiru_bindings::querier::NibiruQuerier;
use nibiru_bindings::query::QueryPerpMsg;

use crate::msg::{
    nibiru_msg_to_cw_response, ExecuteMsg, InstantiateMsg, NibiruExecuteMsg,
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
        QueryPerpMsg::AllMarkets {} => {
            to_binary(&querier.all_markets().unwrap())
        }
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
        QueryPerpMsg::Metrics { pair } => {
            to_binary(&querier.metrics(pair).unwrap())
        }
        QueryPerpMsg::ModuleAccounts {} => {
            to_binary(&querier.module_accounts()?)
        }
        QueryPerpMsg::ModuleParams {} => to_binary(&querier.module_params()?),
        QueryPerpMsg::PremiumFraction { pair } => {
            to_binary(&querier.premium_fraction(pair)?)
        }
        QueryPerpMsg::Reserves { pair } => to_binary(&querier.reserves(pair)?),
    }
}

#[entry_point]
pub fn execute(
    _deps: DepsMut<QueryPerpMsg>,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response<NibiruExecuteMsg>> {
    match msg {
        ExecuteMsg::OpenPosition {
            sender,
            pair,
            is_long,
            quote_amount,
            leverage,
            base_amount_limit,
        } => nibiru_msg_to_cw_response(NibiruExecuteMsg::open_position(
            sender,
            pair,
            is_long,
            quote_amount,
            leverage,
            base_amount_limit,
        )),

        ExecuteMsg::ClosePosition { sender, pair } => nibiru_msg_to_cw_response(
            NibiruExecuteMsg::close_position(sender, pair),
        ),

        ExecuteMsg::AddMargin {
            sender,
            pair,
            margin,
        } => nibiru_msg_to_cw_response(NibiruExecuteMsg::add_margin(
            sender, pair, margin,
        )),

        ExecuteMsg::RemoveMargin {
            sender,
            pair,
            margin,
        } => nibiru_msg_to_cw_response(NibiruExecuteMsg::remove_margin(
            sender, pair, margin,
        )),

        ExecuteMsg::MultiLiquidate { pair, liquidations } => {
            nibiru_msg_to_cw_response(NibiruExecuteMsg::multi_liquidate(
                pair,
                liquidations,
            ))
        }

        ExecuteMsg::DonateToInsuranceFund { sender, donation } => {
            nibiru_msg_to_cw_response(
                NibiruExecuteMsg::donate_to_insurance_fund(sender, donation),
            )
        }

        ExecuteMsg::PegShift { pair, peg_mult } => nibiru_msg_to_cw_response(
            NibiruExecuteMsg::peg_shift(pair, peg_mult),
        ),

        ExecuteMsg::DepthShift { pair, depth_mult } => {
            nibiru_msg_to_cw_response(NibiruExecuteMsg::depth_shift(
                pair, depth_mult,
            ))
        }
    }
}

#[cfg(test)]
pub mod integration_tests {
    use crate::msg::InstantiateMsg;
    use cosmwasm_std::{coins, Decimal, Response};
    use cosmwasm_vm::testing::{
        instantiate, mock_env, mock_info, mock_instance,
    };
    use std::str::FromStr;

    // TODO test that the file exists
    static WASM: &[u8] = include_bytes!("../../../artifacts/bindings_perp.wasm");

    #[test]
    fn msg_init() {
        let mut deps = mock_instance(WASM, &[]);
        let sender = String::from("sender");
        let info = mock_info(&sender, &coins(1000, "unibi"));
        let inst_msg = InstantiateMsg {};
        let result: Response =
            instantiate(&mut deps, mock_env(), info, inst_msg).unwrap();
        assert_eq!(result.messages.len(), 0);
    }

    #[test]
    fn negative_decimal_not_possible() {
        let neg = Decimal::from_str("-420");
        assert!(neg.is_err())
    }

    // Example integration test for a custom query
    // TODO This requires writing a test querier that registers the custom enum
    //
    // const DESERIALIZATION_LIMIT: usize = 20_000;
    //
    // #[test]
    // fn query_reserves() {
    //     let mut deps = mock_instance(WASM, &[]);
    //     let sender = String::from("sender");
    //     let info = mock_info(&sender, &coins(1000, "unibi"));
    //     let inst_msg = InstantiateMsg {};
    //     let result: Response =
    //         instantiate(&mut deps, mock_env(), info, inst_msg).unwrap();
    //     assert_eq!(result.messages.len(), 0);

    //     let pair = String::from("ueth:unusd");
    //     let query_msg = NibiruQuery::Reserves { pair };
    //     let raw_resp = query(&mut deps, mock_env(), query_msg);
    //     assert!(raw_resp.is_err(), "err: {}", raw_resp.unwrap_err());
    //     let resp: ReservesResponse =
    //         from_slice(&raw_resp.unwrap(), DESERIALIZATION_LIMIT).unwrap();
    //     assert_eq!(resp.pair, pair)
    // }
}
