use cosmwasm_schema::cw_serde;
use cosmwasm_std::{
    attr, entry_point, Binary, CosmosMsg, CustomMsg, Deps, DepsMut, Env,
    MessageInfo, Response, StdResult,
};
use nibiru_macro::cw_custom;
use nibiru_std::bindings::route::NibiruRoute;

use crate::{
    msgs::{ExecuteMsg, InitMsg, IsMemberResponse, QueryMsg, WhitelistResponse},
    state::{Whitelist, WHITELIST},
};

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InitMsg,
) -> StdResult<Response> {
    let whitelist = Whitelist {
        members: vec![msg.admin.clone()].into_iter().collect(),
        admin: msg.admin,
    };
    WHITELIST.save(deps.storage, &whitelist)?;
    Ok(Response::default())
}

fn check_admin(can: CanExecute) -> Result<(), cosmwasm_std::StdError> {
    match can.is_admin {
        true => Ok(()),
        false => Err(cosmwasm_std::StdError::generic_err(format!(
            "unauthorized : sender {} is not an admin",
            can.sender,
        ))),
    }
}

fn check_member(can: CanExecute) -> Result<(), cosmwasm_std::StdError> {
    match can.is_member {
        true => Ok(()),
        false => Err(cosmwasm_std::StdError::generic_err(format!(
            "unauthorized : sender {} is not a whitelist member",
            can.sender,
        ))),
    }
}

/// ExecuteResponse allows the execute entry point to return different response
/// types depending on the input. This is possible because we wrap the Response
/// type with variants of ExecuteResponse. These variants store a Response type.
///
/// In CosmWasm, there are multiple entry points for handling different message
/// types, such as instantiate, execute, query, sudo, and migrate. However,
/// each entry point returns a single type of response. You cannot have multiple
/// entry points return different returning different response types for the
/// same message type. Ref: https://book.cosmwasm.com/basics/entry-points.html
#[cw_serde]
#[cw_custom]
pub struct ContractExecMsg {
    pub route: NibiruRoute,
    pub msg: Option<ExecuteMsg>,
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response<ContractExecMsg>> {
    let deps_for_check = &deps;
    let check: CanExecute =
        can_execute(deps_for_check.as_ref(), info.sender.as_ref())?;
    let mut whitelist = check.whitelist.clone();

    match msg {
        ExecuteMsg::InsuranceFundWithdraw { amount, to } => {
            check_member(check)?;
            let cw_msg = ContractExecMsg {
                route: NibiruRoute::Perp,
                msg: Some(ExecuteMsg::InsuranceFundWithdraw { amount, to }),
            };
            let res = Response::new()
                .add_message(cw_msg)
                .add_attributes(vec![attr("action", "insurance_fund_withdraw")]);
            Ok(res)
        }

        ExecuteMsg::SetMarketEnabled { pair, enabled } => {
            check_member(check)?;
            let cw_msg = ContractExecMsg {
                route: NibiruRoute::Perp,
                msg: Some(ExecuteMsg::SetMarketEnabled { pair, enabled }),
            };
            let res = Response::new()
                .add_message(cw_msg)
                .add_attributes(vec![attr("action", "toggle_market_enabled")]);
            Ok(res)
        }

        ExecuteMsg::CreateMarket {
            pair,
            peg_mult,
            sqrt_depth,
            market_params,
        } => {
            check_member(check)?;
            let cw_msg = ContractExecMsg {
                route: NibiruRoute::Perp,
                msg: Some(ExecuteMsg::CreateMarket {
                    pair,
                    peg_mult,
                    sqrt_depth,
                    market_params,
                }),
            };
            let res = Response::new()
                .add_message(cw_msg)
                .add_attributes(vec![attr("action", "create_market")]);
            Ok(res)
        }

        ExecuteMsg::EditOracleParams {
            vote_period,
            vote_threshold,
            reward_band,
            whitelist,
            slash_fraction,
            slash_window,
            min_valid_per_window,
            twap_lookback_window,
            min_voters,
            validator_fee_ratio,
        } => {
            check_member(check)?;
            let cw_msg = ContractExecMsg {
                route: NibiruRoute::Oracle,
                msg: Some(ExecuteMsg::EditOracleParams {
                    vote_period,
                    vote_threshold,
                    reward_band,
                    whitelist,
                    slash_fraction,
                    slash_window,
                    min_valid_per_window,
                    twap_lookback_window,
                    min_voters,
                    validator_fee_ratio,
                }),
            };
            let res = Response::new()
                .add_message(cw_msg)
                .add_attributes(vec![attr("action", "edit_oracle")]);
            Ok(res)
        }

        ExecuteMsg::AddMember { address } => {
            check_admin(check)?;
            let api = deps.api;
            let addr = api.addr_validate(address.as_str()).unwrap();
            whitelist.members.insert(addr.into_string());
            WHITELIST.save(deps.storage, &whitelist)?;

            let cw_msg = ContractExecMsg {
                route: NibiruRoute::NoOp,
                msg: None,
            };
            let res = Response::new().add_message(cw_msg).add_attributes(vec![
                attr("action", "add_member"),
                attr("address", address),
            ]);
            Ok(res)
        }

        ExecuteMsg::RemoveMember { address } => {
            check_admin(check)?;
            whitelist.members.remove(address.as_str());
            WHITELIST.save(deps.storage, &whitelist)?;

            let cw_msg = ContractExecMsg {
                route: NibiruRoute::NoOp,
                msg: None,
            };
            let res = Response::new().add_message(cw_msg).add_attributes(vec![
                attr("action", "remove_member"),
                attr("address", address),
            ]);
            Ok(res)
        }

        ExecuteMsg::ChangeAdmin { address } => {
            check_admin(check)?;
            let api = deps.api;
            let new_admin = api.addr_validate(address.as_str()).unwrap();
            whitelist.admin = new_admin.clone().into_string();
            whitelist.members.insert(new_admin.to_string());
            WHITELIST.save(deps.storage, &whitelist)?;

            let cw_msg = ContractExecMsg {
                route: NibiruRoute::NoOp,
                msg: None,
            };
            let res = Response::new().add_message(cw_msg).add_attributes(vec![
                attr("action", "change_admin"),
                attr("address", address),
            ]);
            Ok(res)
        }
    }
}

struct CanExecute {
    is_admin: bool,
    is_member: bool,
    sender: String,
    whitelist: Whitelist,
}

fn can_execute(deps: Deps, sender: &str) -> StdResult<CanExecute> {
    let whitelist = WHITELIST.load(deps.storage).unwrap();
    Ok(CanExecute {
        is_admin: whitelist.is_admin(sender),
        is_member: whitelist.is_member(sender),
        sender: sender.into(),
        whitelist,
    })
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::IsMember { address } => {
            let whitelist = WHITELIST.load(deps.storage)?;
            let is_member: bool = whitelist.is_member(address);
            let res = IsMemberResponse {
                is_member,
                whitelist,
            };
            cosmwasm_std::to_binary(&res)
        }
        QueryMsg::Whitelist {} => {
            let whitelist = WHITELIST.load(deps.storage)?;
            let res = WhitelistResponse { whitelist };
            cosmwasm_std::to_binary(&res)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        msgs::{ExecuteMsg, InitMsg},
        state::WHITELIST,
    };

    use cosmwasm_std::{coins, testing, Addr};
    use std::collections::HashSet;

    // ---------------------------------------------------------------------------
    // Tests
    // ---------------------------------------------------------------------------

    #[test]
    fn test_instantiate() {
        let mut deps = testing::mock_dependencies();
        let msg = InitMsg {
            admin: "admin".to_string(),
        };
        let info: MessageInfo =
            testing::mock_info("addr0000", &coins(2, "token"));

        let result =
            instantiate(deps.as_mut(), testing::mock_env(), info, msg).unwrap();
        assert_eq!(result.messages.len(), 0);
    }

    #[test]
    fn test_has_admin_power() {
        let admin = Addr::unchecked("admin");
        let msg = &InitMsg {
            admin: admin.to_string(),
        };

        let sender = "not-admin";
        let mut deps = testing::mock_dependencies();
        let msg_info = testing::mock_info(sender, &coins(2, "token"));
        instantiate(deps.as_mut(), testing::mock_env(), msg_info, msg.clone())
            .unwrap();
        let whitelist = WHITELIST.load(&deps.storage).unwrap();
        let has: bool = whitelist.is_admin(sender);
        assert!(!has);

        let sender = "admin";
        let mut deps = testing::mock_dependencies();
        let msg_info = testing::mock_info(sender, &coins(2, "token"));
        instantiate(deps.as_mut(), testing::mock_env(), msg_info, msg.clone())
            .unwrap();
        let whitelist = WHITELIST.load(&deps.storage).unwrap();
        let has: bool = whitelist.is_admin(sender);
        assert!(has);
    }

    #[test]
    fn test_execute_unauthorized() {
        let mut deps = testing::mock_dependencies();
        let admin = Addr::unchecked("admin");

        let msg = InitMsg {
            admin: admin.as_str().to_string(),
        };
        let msg_info = testing::mock_info("addr0000", &coins(2, "token"));
        instantiate(deps.as_mut(), testing::mock_env(), msg_info, msg).unwrap();

        let execute_msg = ExecuteMsg::AddMember {
            address: "addr0001".to_string(),
        };
        let unauthorized_info = testing::mock_info("unauthorized", &[]);
        let result = execute(
            deps.as_mut(),
            testing::mock_env(),
            unauthorized_info,
            execute_msg,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_execute_add_member() {
        // Init contract
        let mut deps = testing::mock_dependencies();
        let admin = Addr::unchecked("admin");

        let init_msg = InitMsg {
            admin: admin.as_str().to_string(),
        };
        let init_info = testing::mock_info("addr0000", &coins(2, "token"));
        instantiate(deps.as_mut(), testing::mock_env(), init_info, init_msg)
            .unwrap();

        let new_member = "new_member";
        let whitelist = WHITELIST.load(&deps.storage).unwrap();
        let has: bool = whitelist.is_admin(new_member);
        assert!(!has);

        // Add a member to whitelist
        let execute_msg = ExecuteMsg::AddMember {
            address: new_member.to_string(),
        };
        let execute_info = testing::mock_info(admin.as_str(), &[]);

        let check_resp = |resp: Response<ContractExecMsg>| {
            assert_eq!(
                resp.messages.len(),
                1,
                "resp.messages: {:?}",
                resp.messages
            );
            assert_eq!(
                resp.attributes.len(),
                2,
                "resp.attributes: {:#?}",
                resp.attributes
            );
        };

        let result = execute(
            deps.as_mut(),
            testing::mock_env(),
            execute_info,
            execute_msg,
        )
        .unwrap();
        check_resp(result);

        // Check correctness of the result
        let whitelist = WHITELIST.load(&deps.storage).unwrap();
        let has: bool = whitelist.has(new_member);
        assert!(has);

        let query_req = QueryMsg::IsMember {
            address: new_member.to_string(),
        };
        let binary =
            query(deps.as_ref(), testing::mock_env(), query_req).unwrap();
        let response: IsMemberResponse =
            cosmwasm_std::from_binary(&binary).unwrap();
        assert!(response.is_member);
    }

    #[test]
    fn test_execute_remove_member() {
        // Init contract
        let _deps = testing::mock_dependencies();
        let mut deps = testing::mock_dependencies();
        let admin = Addr::unchecked("admin");

        let init_msg = InitMsg {
            admin: admin.as_str().to_string(),
        };
        let init_info = testing::mock_info("addr0000", &coins(2, "token"));
        instantiate(deps.as_mut(), testing::mock_env(), init_info, init_msg)
            .unwrap();

        // Set up initial whitelist
        let members_start: Vec<String> = ["vitalik", "musk", "satoshi"]
            .iter()
            .map(|&s| s.to_string())
            .collect();
        let mut whitelist = WHITELIST.load(&deps.storage).unwrap();
        assert_eq!(whitelist.members.len(), 1); // admin remains
        for member in members_start.iter() {
            whitelist.members.insert(member.clone());
        }
        let res = WHITELIST.save(deps.as_mut().storage, &whitelist);
        assert!(res.is_ok());

        // Remove a member from the whitelist
        let execute_msg = ExecuteMsg::RemoveMember {
            address: "satoshi".to_string(),
        };
        let execute_info = testing::mock_info(admin.as_str(), &[]);
        let check_resp = |resp: Response<ContractExecMsg>| {
            assert_eq!(
                resp.messages.len(),
                1,
                "resp.messages: {:?}",
                resp.messages
            );
            assert_eq!(
                resp.attributes.len(),
                2,
                "resp.attributes: {:#?}",
                resp.attributes
            );
        };
        let result = execute(
            deps.as_mut(),
            testing::mock_env(),
            execute_info,
            execute_msg,
        )
        .unwrap();
        check_resp(result);

        // Check correctness of the result
        let query_req = QueryMsg::Whitelist {};
        let binary =
            query(deps.as_ref(), testing::mock_env(), query_req).unwrap();
        let response: WhitelistResponse =
            cosmwasm_std::from_binary(&binary).unwrap();
        let expected_members: HashSet<String> = ["vitalik", "musk", "admin"]
            .iter()
            .map(|&s| s.to_string())
            .collect();
        assert_eq!(
            response.whitelist.members, expected_members,
            "got: {:#?}, wanted: {:#?}",
            response.whitelist.members, expected_members
        );
    }

    #[test]
    fn test_execute_change_admin() {
        // Init contract
        let mut deps = testing::mock_dependencies();
        let admin = Addr::unchecked("admin");

        let init_msg = InitMsg {
            admin: admin.as_str().to_string(),
        };
        let init_info = testing::mock_info("addr0000", &coins(2, "token"));
        instantiate(deps.as_mut(), testing::mock_env(), init_info, init_msg)
            .unwrap();

        let new_admin = "new_admin";
        let whitelist = WHITELIST.load(&deps.storage).unwrap();
        let has: bool = whitelist.is_admin(new_admin);
        assert!(!has);

        // Add a member to whitelist
        let execute_msg = ExecuteMsg::ChangeAdmin {
            address: new_admin.to_string(),
        };
        let execute_info = testing::mock_info(admin.as_str(), &[]);

        let check_resp = |resp: Response<ContractExecMsg>| {
            assert_eq!(
                resp.messages.len(),
                1,
                "resp.messages: {:?}",
                resp.messages
            );
            assert_eq!(
                resp.attributes.len(),
                2,
                "resp.attributes: {:#?}",
                resp.attributes
            );
        };

        let result = execute(
            deps.as_mut(),
            testing::mock_env(),
            execute_info,
            execute_msg,
        )
        .unwrap();
        check_resp(result);

        // Check correctness of the result
        let whitelist = WHITELIST.load(&deps.storage).unwrap();
        let has: bool = whitelist.has(new_admin);
        assert!(has);

        // The new admin should not yet be a member
        let query_req = QueryMsg::IsMember {
            address: new_admin.to_string(),
        };
        let binary =
            query(deps.as_ref(), testing::mock_env(), query_req).unwrap();
        let response: IsMemberResponse =
            cosmwasm_std::from_binary(&binary).unwrap();
        assert!(response.is_member);
    }
}
