use broker_bank::contract::query_bank_balances;
use cosmwasm_std::{
    entry_point, BankMsg, Binary, CosmosMsg, Deps, DepsMut, Env, MessageInfo,
    Response, StdError, StdResult,
};

use cw2::set_contract_version;

use nibiru_std::proto::nibiru::perp::Direction;
use nibiru_std::proto::{nibiru, NibiruStargateMsg};

use crate::{
    msg::{ExecuteMsg, InitMsg, QueryMsg},
    state::{Sudoers, SUDOERS},
};

const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
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

// TODO: impl query entry point
#[entry_point]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    todo!();
    // let querier = NibiruQuerier::new(&deps.querier);
    // match msg {
    //     QueryMsg::AllMarkets {} => {
    //         to_json_binary(&querier.all_markets().unwrap())
    //     }
    //     QueryMsg::BasePrice {
    //         pair,
    //         is_long,
    //         base_amount,
    //     } => to_json_binary(
    //         &querier.base_price(pair, is_long, base_amount).unwrap(),
    //     ),
    //     QueryMsg::Position { trader, pair } => {
    //         to_json_binary(&querier.position(trader, pair).unwrap())
    //     }
    //     QueryMsg::Positions { trader } => {
    //         to_json_binary(&querier.positions(trader).unwrap())
    //     }
    //     QueryMsg::Metrics { pair } => {
    //         to_json_binary(&querier.metrics(pair).unwrap())
    //     }
    //     QueryMsg::ModuleAccounts {} => {
    //         to_json_binary(&querier.module_accounts()?)
    //     }
    //     QueryMsg::ModuleParams {} => to_json_binary(&querier.module_params()?),
    //     QueryMsg::PremiumFraction { pair } => {
    //         to_json_binary(&querier.premium_fraction(pair)?)
    //     }
    //     QueryMsg::Reserves { pair } => to_json_binary(&querier.reserves(pair)?),
    //     QueryMsg::OraclePrices { pairs } => {
    //         to_json_binary(&querier.oracle_prices(pairs)?)
    //     }

    //     // TODO test
    //     QueryMsg::Sudoers {} => {
    //         let sudoers = SUDOERS.load(deps.storage)?;
    //         let res = SudoersQueryResponse { sudoers };
    //         cosmwasm_std::to_json_binary(&res)
    //     }
    // }
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env_ctx: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response> {
    let deps_for_check = &deps;
    let can_execute: CanExecute =
        check_can_execute(deps_for_check.as_ref(), info.sender.as_ref())?;
    let contract_addr = env_ctx.contract.address.to_string();
    match msg {
        ExecuteMsg::MarketOrder {
            pair,
            is_long,
            quote_amount,
            leverage,
            base_amount_limit,
        } => {
            can_execute.check_member()?;
            let side: Direction = if is_long {
                Direction::Long
            } else {
                Direction::Short
            };

            // TODO: feat(nibiru-std): Fn to convert cosmwasm_std::Decimal to
            // protobuf strings for sdk.Dec [with tests] | https://github.com/NibiruChain/cw-nibiru/issues/99
            let cosmos_msg: CosmosMsg = nibiru::perp::MsgMarketOrder {
                sender: contract_addr,
                pair,
                side: side.into(),
                // TODO: sdk.Dec https://github.com/NibiruChain/cw-nibiru/issues/99
                quote_asset_amount: quote_amount.to_string(),
                // TODO: sdk.Dec https://github.com/NibiruChain/cw-nibiru/issues/99
                leverage: leverage.to_string(),
                // TODO: sdk.Dec https://github.com/NibiruChain/cw-nibiru/issues/99
                base_asset_amount_limit: base_amount_limit.to_string(),
            }
            .into_stargate_msg();
            Ok(Response::new().add_message(cosmos_msg))
        }

        ExecuteMsg::ClosePosition { pair } => {
            can_execute.check_member()?;
            let cosmos_msg: CosmosMsg = nibiru::perp::MsgClosePosition {
                sender: contract_addr,
                pair,
            }
            .into_stargate_msg();
            Ok(Response::new().add_message(cosmos_msg))
        }

        ExecuteMsg::AddMargin { pair, margin } => {
            can_execute.check_member()?;
            let cosmos_msg: CosmosMsg = nibiru::perp::MsgAddMargin {
                sender: contract_addr,
                pair,
                margin: Some(margin.into()),
            }
            .into_stargate_msg();
            Ok(Response::new().add_message(cosmos_msg))
        }

        ExecuteMsg::RemoveMargin { pair, margin } => {
            can_execute.check_member()?;
            let cosmos_msg: CosmosMsg = nibiru::perp::MsgRemoveMargin {
                sender: contract_addr,
                pair,
                margin: Some(margin.into()),
            }
            .into_stargate_msg();
            Ok(Response::new().add_message(cosmos_msg))
        }

        // TODO:test ExecuteMsg::MultiLiquidate
        ExecuteMsg::MultiLiquidate { liquidations } => {
            can_execute.check_member()?;
            let cosmos_msg: CosmosMsg = nibiru::perp::MsgMultiLiquidate {
                sender: contract_addr,
                liquidations: liquidations
                    .iter()
                    .map(|la| nibiru::perp::msg_multi_liquidate::Liquidation {
                        pair: la.pair.clone(),
                        trader: la.trader.clone(),
                    })
                    .collect(),
            }
            .into_stargate_msg();
            Ok(Response::new().add_message(cosmos_msg))
        }

        // TODO:test ExecuteMsg::DonateToInsuranceFund
        ExecuteMsg::DonateToInsuranceFund { donation } => {
            can_execute.check_member()?;
            let cosmos_msg: CosmosMsg = nibiru::perp::MsgDonateToEcosystemFund {
                sender: contract_addr,
                donation: Some(donation.into()),
            }
            .into_stargate_msg();
            Ok(Response::new().add_message(cosmos_msg))
        }

        ExecuteMsg::Claim {
            funds,
            claim_all,
            to,
        } => {
            can_execute.check_admin()?;
            let event_key = "execute_claim";
            if let Some(claim_all_value) = claim_all {
                if !claim_all_value {
                    return Err(StdError::generic_err(
                        "setting 'claim_all' to false causes an error: "
                            .to_string()
                            + "try removing claim_all as an argument entirely.",
                    ));
                }
                let contract_address = env_ctx.contract.address;
                let balances: Vec<cosmwasm_std::Coin> = query_bank_balances(
                    contract_address.to_string(),
                    deps.as_ref(),
                )?;

                // Send all funds to the specified recipient.
                let transfer_msg = BankMsg::Send {
                    to_address: to.clone(),
                    amount: balances,
                };
                Ok(Response::new().add_message(transfer_msg).add_attribute(
                    event_key,
                    format!("successfully claimed to {}", to),
                ))
            } else if let Some(funds_value) = funds {
                // Send all funds to the specified recipient.
                let transfer_msg = BankMsg::Send {
                    to_address: to.clone(),
                    amount: vec![funds_value],
                };
                return Ok(Response::new()
                    .add_message(transfer_msg)
                    .add_attribute(
                        event_key,
                        format!("successfully claimed to {}", to),
                    ));
            } else {
                return Err(StdError::generic_err(
                    "either the 'funds' or 'claim_all' arguments must be specified"));
            }
        } // TODO test: add member
          // TODO test: remove member
          // TODO test: change admin
    }
}

struct CanExecute {
    is_admin: bool,
    is_member: bool,
    sender: String,
}

impl CanExecute {
    pub fn check_admin(&self) -> Result<(), cosmwasm_std::StdError> {
        match self.is_admin {
            true => Ok(()),
            false => Err(cosmwasm_std::StdError::generic_err(format!(
                "unauthorized : sender {} is not an admin",
                self.sender,
            ))),
        }
    }

    pub fn check_member(&self) -> Result<(), cosmwasm_std::StdError> {
        match self.is_member {
            true => Ok(()),
            false => Err(cosmwasm_std::StdError::generic_err(format!(
                "unauthorized : sender {} is not a sudoers member",
                self.sender,
            ))),
        }
    }
}

fn check_can_execute(deps: Deps, sender: &str) -> StdResult<CanExecute> {
    let sudoers = SUDOERS.load(deps.storage).unwrap();
    Ok(CanExecute {
        is_admin: sudoers.is_admin(sender),
        is_member: sudoers.is_member(sender),
        sender: sender.into(),
    })
}

#[cfg(test)]
pub mod tests {
    use std::str::FromStr;

    use crate::state;
    use cosmwasm_std::{
        coin, coins,
        testing::{self, mock_env, MockApi, MockQuerier},
        Coin, CosmosMsg, Decimal, MemoryStorage, OwnedDeps, SubMsg, Uint128,
    };

    use super::*;

    pub type TestResult = anyhow::Result<()>;

    pub const SENDER_NOT_ADMIN: &str = "not-admin";
    pub const SENDER_ADMIN: &str = "admin";

    #[test]
    fn msg_init() -> TestResult {
        let mut deps = testing::mock_dependencies();
        let admin = SENDER_ADMIN;
        let msg = InitMsg {
            admin: Some(admin.to_string()),
        };
        let sender = "sender";
        let info: MessageInfo = testing::mock_info(sender, &coins(2, "token"));

        let result = instantiate(deps.as_mut(), testing::mock_env(), info, msg)?;
        assert_eq!(result.messages.len(), 0);

        let sudoers = SUDOERS.load(&deps.storage)?;
        assert_eq!(sudoers.admin, admin);
        Ok(())
    }

    #[test]
    fn msg_init_admin_as_sender() -> TestResult {
        let mut deps = testing::mock_dependencies();
        let msg = InitMsg { admin: None };
        let sender = "sender";
        let info: MessageInfo = testing::mock_info(sender, &coins(2, "token"));

        let result = instantiate(deps.as_mut(), testing::mock_env(), info, msg)?;
        assert_eq!(result.messages.len(), 0);

        let sudoers = SUDOERS.load(&deps.storage)?;
        assert_eq!(sudoers.admin, sender);
        Ok(())
    }

    fn do_init(
        admin: &str,
        sender: &str,
        mut deps: OwnedDeps<MemoryStorage, MockApi, MockQuerier>,
    ) -> (
        state::Sudoers,
        OwnedDeps<MemoryStorage, MockApi, MockQuerier>,
        cosmwasm_std::MessageInfo,
    ) {
        let msg_init = InitMsg {
            admin: Some(admin.to_string()),
        };

        // let mut deps = testing::mock_dependencies();
        let info: MessageInfo = testing::mock_info(sender, &coins(2, "token"));

        let result = instantiate(
            deps.as_mut(),
            testing::mock_env(),
            info.clone(),
            msg_init,
        )
        .expect("failed instantiate in the 'do_init' fn");
        assert_eq!(result.messages.len(), 0);
        let sudoers = SUDOERS
            .load(&deps.storage)
            .expect("state should be loadable after InitMsg");
        assert_eq!(sudoers.admin, admin);
        (sudoers, deps, info)
    }

    #[test]
    fn exec_stargate() -> TestResult {
        let deps = testing::mock_dependencies();

        // Instantiate contract
        let admin = SENDER_ADMIN;
        let sender = admin;
        let (_sudoers, mut deps, info) = do_init(admin, sender, deps);

        let pair = "ETH:USD".to_string();
        let dummy_u128 = Uint128::new(420u128);
        let dummy_coin = coin(dummy_u128.clone().u128(), "token");
        let exec_msgs: Vec<ExecuteMsg> = vec![
            ExecuteMsg::MarketOrder {
                pair: pair.clone(),
                is_long: true,
                quote_amount: dummy_u128,
                leverage: Decimal::from_str("5")?,
                base_amount_limit: Uint128::zero(),
            },
            ExecuteMsg::ClosePosition { pair: pair.clone() },
            ExecuteMsg::AddMargin {
                pair: pair.clone(),
                margin: dummy_coin.clone(),
            },
            ExecuteMsg::RemoveMargin {
                pair,
                margin: dummy_coin,
            },
        ];
        for exec_msg in &exec_msgs {
            exec_stargate_test_happy(exec_msg, deps.as_mut(), info.clone())?;
            exec_stargate_test_without_permission(exec_msg, deps.as_mut())?;
        }
        Ok(())
    }

    /// Verifies that an execute message will fail when sent by a non-admin
    pub fn exec_stargate_test_without_permission(
        exec_msg: &ExecuteMsg,
        deps: DepsMut,
    ) -> TestResult {
        let sender: &str = "not-admin";
        let info: MessageInfo = testing::mock_info(sender, &coins(2, "token"));
        let resp = execute(deps, mock_env(), info, exec_msg.clone());
        assert!(resp.is_err(), "resp.err: {:?}", resp.err());
        Ok(())
    }

    /// Verifies that an execute message will succeed and contain a
    /// `cosmwasm_std::SubMsg` for the `CosmosMsg::Stargate` to be executed.
    pub fn exec_stargate_test_happy(
        exec_msg: &ExecuteMsg,
        deps: DepsMut,
        info: MessageInfo,
    ) -> TestResult {
        let resp = execute(deps, mock_env(), info, exec_msg.clone())?;
        assert_eq!(resp.messages.len(), 1, "resp.messages: {:?}", resp.messages);
        Ok(())
    }

    struct TestCaseExecClaim<'a> {
        exec_msg: ExecuteMsg,
        sender: &'a str,
        err: Option<&'a str>,
        funds_sent: Option<&'a str>,
    }

    /// exec_claim: Test the ExecuteMsg::Claim variant.
    #[test]
    fn exec_claim() -> TestResult {
        // Prepare the test environment
        let deps = testing::mock_dependencies();
        let env = mock_env();
        let contract_address = env.contract.address.clone();
        let to_address = String::from("recipient_address");

        // Instantiate contract
        let admin = to_address.as_str();
        let sender = to_address.as_str();
        let (_sudoers, mut deps, _info) = do_init(admin, sender, deps);

        // Set up a mock querier with contract balance
        let balances: &[(&str, &[Coin])] =
            &[(contract_address.as_str(), &[Coin::new(100u128, "token")])];
        let querier = testing::MockQuerier::new(balances);
        deps.querier = querier;

        let test_cases: Vec<TestCaseExecClaim> = vec![
            // claim with no args
            TestCaseExecClaim {
                exec_msg: ExecuteMsg::Claim {
                    funds: None,
                    claim_all: None,
                    to: to_address.clone(),
                },
                sender: to_address.as_str(),
                err: Some("arguments must be specified"),
                funds_sent: None,
            },
            // claim happy / partial
            TestCaseExecClaim {
                exec_msg: ExecuteMsg::Claim {
                    funds: Some(Coin::new(50u128, "token")),
                    claim_all: None,
                    to: to_address.clone(),
                },
                sender: to_address.as_str(),
                err: None,
                funds_sent: Some("50"),
            },
            // claim happy / all
            TestCaseExecClaim {
                exec_msg: ExecuteMsg::Claim {
                    funds: None,
                    claim_all: Some(true),
                    to: to_address.clone(),
                },
                sender: to_address.as_str(),
                err: None,
                funds_sent: Some("100"),
            },
            // claim / sad / all set to false
            TestCaseExecClaim {
                exec_msg: ExecuteMsg::Claim {
                    funds: None,
                    claim_all: Some(false),
                    to: to_address.clone(),
                },
                sender: to_address.as_str(),
                err: Some("false causes an error"),
                funds_sent: None,
            },
        ];
        for tc in &test_cases {
            // Instantiate contract
            let deps = testing::mock_dependencies();
            let sender = tc.sender;
            let (_sudoers, mut deps, info) = do_init(admin, sender, deps);

            // Set up a mock querier with contract balance
            let balances: &[(&str, &[Coin])] =
                &[(contract_address.as_str(), &[Coin::new(100u128, "token")])];
            let querier = testing::MockQuerier::new(balances);
            deps.querier = querier;

            let res =
                execute(deps.as_mut(), mock_env(), info, tc.exec_msg.clone());
            if let Some(err) = tc.err {
                assert!(res.is_err());
                assert!(res.unwrap_err().to_string().contains(err));
                continue;
            }

            // Check for the correct number of SubMsgs
            assert!(res.is_ok(), "res: {:?}", res);
            let resp = res?;
            assert_eq!(resp.messages.len(), 1);

            // Check that the correct amount of funds are sent
            if let Some(want_funds_sent) = tc.funds_sent {
                let sub_msg: &SubMsg = &resp.messages[0];
                let transfer_msg: &CosmosMsg = &sub_msg.msg;
                let msg_json: String =
                    serde_json::to_string_pretty(&transfer_msg)
                        .expect("Failed to serialized JSON");
                println!("msg_json: {:?}", msg_json);

                let contract_balance: &Coin = &balances[0].1[0];
                let denom: String = contract_balance.denom.clone();
                let amount: String = want_funds_sent.to_string();
                let expected_json_elements: Vec<String> = vec![
                    format!(r#""denom": "{}""#, denom),
                    format!(r#""amount": "{}""#, amount),
                ];
                for elem in &expected_json_elements {
                    assert!(
                        msg_json.to_string().clone().contains(elem),
                        "elem: {}",
                        elem
                    );
                }
            }
        }
        Ok(())
    }
}
