use cosmwasm_std::{
    entry_point, from_binary, to_binary, AllBalanceResponse, BankMsg, BankQuery,
    Binary, CosmosMsg, Deps, DepsMut, Env, MessageInfo,
    QueryRequest, Response, StdError, StdResult,
};

use cw2::set_contract_version;

use nibiru_bindings::query::QueryPerpMsg;
use nibiru_bindings::{querier::NibiruQuerier, route::NibiruRoute};

use crate::{
    msg::{
        nibiru_msg_to_cw_response, ExecuteMsg, InitMsg, NibiruExecuteMsg,
        QueryMsg, SudoersQueryResponse,
    },
    state::{Sudoers, SUDOERS},
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
        Some(msg_admin) => msg_admin,
        None => info.sender.to_string(),
    };
    let sudoers = Sudoers {
        members: vec![admin.clone()].into_iter().collect(),
        admin,
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

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env_ctx: Env,
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

        // TODO test
        ExecuteMsg::MultiLiquidate { pair, liquidations } => {
            nibiru_msg_to_cw_response(NibiruExecuteMsg::multi_liquidate(
                pair,
                liquidations,
            ))
        }

        // TODO test
        ExecuteMsg::DonateToInsuranceFund { donation } => {
            nibiru_msg_to_cw_response(
                NibiruExecuteMsg::donate_to_insurance_fund(donation),
            )
        }

        // TODO test
        ExecuteMsg::Claim {
            funds,
            claim_all,
            to,
        } => {
            // let querier = NibiruQuerier::new(&deps.querier);
            let contract_address = env_ctx.contract.address;
            let event_key = "execute_claim";
            if let Some(claim_all_value) = claim_all {
                if !claim_all_value {
                    return Err(StdError::generic_err(
                        "setting 'claim_all' to false causes an error: "
                            .to_string()
                            + "try removing claim_all as an argument entirely.",
                    ));
                }
                // Query all contract balances
                let query_request = QueryRequest::Bank(BankQuery::AllBalances {
                    address: contract_address.to_string(),
                });
                let query_result = deps.querier.query(&query_request).unwrap();
                let balances: AllBalanceResponse =
                    from_binary(&query_result).unwrap();

                // Send all funds to the specified recipient.
                let transfer_msg = BankMsg::Send {
                    to_address: to.clone(),
                    amount: balances.amount,
                };
                let cw_msg: CosmosMsg<NibiruExecuteMsg> = NibiruExecuteMsg {
                    route: NibiruRoute::NoOp,
                    msg: ExecuteMsg::NoOp {},
                }
                .into();
                Ok(Response::new()
                    .add_message(transfer_msg)
                    .add_message(cw_msg)
                    .add_attribute(
                        event_key,
                        format!("successfully claimed to {}", to),
                    ))
            } else if let Some(funds_value) = funds {
                // Send all funds to the specified recipient.
                let transfer_msg = BankMsg::Send {
                    to_address: to.clone(),
                    amount: vec![funds_value],
                };
                let cw_msg: CosmosMsg<NibiruExecuteMsg> = NibiruExecuteMsg {
                    route: NibiruRoute::NoOp,
                    msg: ExecuteMsg::NoOp {},
                }
                .into();
                return Ok(Response::new()
                    .add_message(transfer_msg)
                    .add_message(cw_msg)
                    .add_attribute(
                        event_key,
                        format!("successfully claimed to {}", to),
                    ));
            } else if funds.is_none() && claim_all.is_none() {
                return Err(StdError::generic_err(
                    "either the 'funds' or 'claim_all' arguments must be specified"));
            } else {
                return Err(StdError::generic_err(
                    "invalid 'funds' or 'claim_all' arguments passed",
                ));
            }
        }

        // TODO test
        ExecuteMsg::NoOp {} => {
            nibiru_msg_to_cw_response(NibiruExecuteMsg::no_op())
        }
    }
}

#[cfg(test)]
pub mod tests {
    use std::str::FromStr;

    use cosmwasm_std::{
        coin, coins,
        testing::{self, mock_env},
        Coin, CosmosMsg, Decimal, Uint128,
    };
    use nibiru_bindings::route::NibiruRoute;

    use super::*;

    #[test]
    fn msg_init() {
        let mut deps = testing::mock_dependencies();
        let admin = "admin";
        let msg = InitMsg {
            admin: Some(admin.to_string()),
        };
        let sender = "sender";
        let info: MessageInfo = testing::mock_info(sender, &coins(2, "token"));

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
        let info: MessageInfo = testing::mock_info(sender, &coins(2, "token"));

        let result =
            instantiate(deps.as_mut(), testing::mock_env(), info, msg).unwrap();
        assert_eq!(result.messages.len(), 0);

        let sudoers = SUDOERS.load(&deps.storage).unwrap();
        assert_eq!(sudoers.admin, sender)
    }

    #[test]
    fn execute_perp_msgs_happy() {
        let mut deps = testing::mock_dependencies();
        let admin = "admin";
        let _msg = InitMsg {
            admin: Some(admin.to_string()),
        };
        let sender = "sender";
        let info: MessageInfo = testing::mock_info(sender, &coins(2, "token"));

        let pair = "ETH:USD".to_string();
        let dummy_u128 = Uint128::new(420u128);
        let dummy_coin = coin(dummy_u128.clone().u128(), "token");
        let exec_msgs: Vec<(ExecuteMsg, NibiruRoute)> = vec![
            (
                ExecuteMsg::OpenPosition {
                    pair: pair.clone(),
                    is_long: true,
                    quote_amount: dummy_u128,
                    leverage: Decimal::from_str("5").unwrap(),
                    base_amount_limit: Uint128::zero(),
                },
                NibiruRoute::Perp,
            ),
            (
                ExecuteMsg::ClosePosition { pair: pair.clone() },
                NibiruRoute::Perp,
            ),
            (
                ExecuteMsg::AddMargin {
                    pair: pair.clone(),
                    margin: dummy_coin.clone(),
                },
                NibiruRoute::Perp,
            ),
            (
                ExecuteMsg::RemoveMargin {
                    pair: pair,
                    margin: dummy_coin,
                },
                NibiruRoute::Perp,
            ),
        ];
        for (exec_msg, route) in &exec_msgs {
            let resp = execute(
                deps.as_mut(),
                mock_env(),
                info.clone(),
                exec_msg.clone(),
            )
            .unwrap();
            assert_eq!(
                resp.messages.len(),
                1,
                "resp.messages: {:?}",
                resp.messages
            );

            // Inspect the message contained in the response to see if it has the expected route
            let msg = &resp.messages[0];
            let custom_exec_msg: &CosmosMsg<NibiruExecuteMsg> = &msg.msg;
            let msg_json = serde_json::to_string_pretty(&custom_exec_msg)
                .expect("Failed to serialized JSON");
            let route_json: String =
                serde_json::to_string_pretty(route).unwrap();
            let route_field_json = format!("\"route\": {}", route_json);
            assert!(
                msg_json.to_string().clone().contains(&route_field_json),
                "route_string {}",
                route_field_json
            );
        }
    }

    #[test]
    fn test_execute_claim() {
        // Prepare the test environment
        let mut deps = testing::mock_dependencies();
        let env = mock_env();
        let contract_address = env.contract.address.clone();
        let to_address = String::from("recipient_address");

        // Set up a mock querier with contract balance
        let balances: &[(&str, &[Coin])] =
            &[(contract_address.as_str(), &[Coin::new(100, "token")])];
        let querier = testing::MockQuerier::new(balances);
        deps.querier = querier;

        // Define the ExecuteMsg::Claim variant
        let msg = ExecuteMsg::Claim {
            funds: Some(Coin::new(50, "token")),
            claim_all: None,
            to: to_address.clone(),
        };

        // Execute the claim
        let sender = to_address.as_str(); // send to self
        let info: MessageInfo = testing::mock_info(sender, &coins(2, "token"));
        let res = execute(deps.as_mut(), env, info, msg);

        // Assert the result
        assert!(res.is_ok());
    }

    #[test]
    fn test_execute_claim_with_no_args() {
        // Prepare the test environment
        let mut deps = testing::mock_dependencies();
        let env = mock_env();
        let contract_address = env.contract.address.clone();
        let to_address = String::from("recipient_address");

        // Set up a mock querier with contract balance
        let balances: &[(&str, &[Coin])] =
            &[(contract_address.as_str(), &[Coin::new(100, "token")])];
        let querier = testing::MockQuerier::new(balances);
        deps.querier = querier;

        // Define the ExecuteMsg::Claim variant
        let msg = ExecuteMsg::Claim {
            funds: None,
            claim_all: None,
            to: to_address.clone(),
        };

        // Execute the claim
        let sender = to_address.as_str(); // send to self
        let info: MessageInfo = testing::mock_info(sender, &coins(2, "token"));
        let res = execute(deps.as_mut(), env, info, msg);

        // Assert the result
        assert!(res.is_err());
        assert!(res
            .unwrap_err()
            .to_string()
            .contains("arguments must be specified"))
    }
}
