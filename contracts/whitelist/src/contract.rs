/// A simple contract that maintains a whitelist of addresses.
/// Takes inspiration from cw-plus/contracts/cw1-whitelist
///
/// This example demonstrates a simple CosmWasm smart contract that manages a
/// whitelist of addresses. The contract initializes with an admin address and
/// allows the admin to add or remove addresses from the whitelist. Users can
/// query whether an address is whitelisted or not.
///
/// ### Entry Points
///
/// - InitMsg: Initializes the contract with the admin address.
/// - ExecuteMsg: Enum for executing msgs
///   - ExecuteMsg::AddMember adds an address to the whitelist
///   - ExecuteMsg::RemoveMember removes and address from the whitelist.
///   - ExecuteMsg::DepthShift
///   - ExecuteMsg::PegShift
///
/// ### Contained Functionality
///
/// 1. Initialize the contract with an admin address.
/// 2. Allow the admin to add or remove addresses from the whitelist.
/// 3. Allow anyone to query if an address is on the whitelist.
/// 4. Members of the whitelist set can execute permissioned calls on the Nibiru
///    x/perp module for dynamic optimizations like peg shift and depth shift.
use std::collections::HashSet;

use cosmwasm_std::{
    attr, entry_point, Binary, Deps, DepsMut, Env, MessageInfo, Response,
    StdResult,
};

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
        members: HashSet::new(),
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

#[entry_point]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response> {
    let deps_for_check = &deps;
    let check: CanExecute =
        can_execute(deps_for_check.as_ref(), info.sender.as_ref())?;
    let mut whitelist = check.whitelist.clone();

    match msg {
        ExecuteMsg::AddMember { address } => {
            check_admin(check)?;
            let api = deps.api;
            let addr = api.addr_validate(address.as_str()).unwrap();
            whitelist.members.insert(addr.into_string());
            WHITELIST.save(deps.storage, &whitelist)?;
            Ok(Response::new().add_attributes(vec![
                attr("action", "add_member"),
                attr("address", address),
            ]))
        }

        ExecuteMsg::RemoveMember { address } => {
            check_admin(check)?;
            whitelist.members.remove(address.as_str());
            WHITELIST.save(deps.storage, &whitelist)?;
            Ok(Response::new().add_attributes(vec![
                attr("action", "remove_member"),
                attr("address", address),
            ]))
        } // TODO Change admin

          // TODO PegShift

          // TODO DepthShift
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

    use cosmwasm_std::coins;
    use cosmwasm_std::{testing, Addr};

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
        let result = execute(
            deps.as_mut(),
            testing::mock_env(),
            execute_info,
            execute_msg,
        )
        .unwrap();
        assert_eq!(
            result.messages.len(),
            0,
            "result.messages: {:?}",
            result.messages
        );
        assert_eq!(
            result.attributes.len(),
            2,
            "result.attributes: {:#?}",
            result.attributes
        );

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
        let members_start: Vec<String> = vec!["vitalik", "musk", "satoshi"]
            .iter()
            .map(|&s| s.to_string())
            .collect();
        let mut whitelist = WHITELIST.load(&deps.storage).unwrap();
        assert_eq!(whitelist.members.len(), 0);
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
        let result = execute(
            deps.as_mut(),
            testing::mock_env(),
            execute_info,
            execute_msg,
        )
        .unwrap();
        assert_eq!(
            result.messages.len(),
            0,
            "result.messages: {:?}",
            result.messages
        );
        assert_eq!(
            result.attributes.len(),
            2,
            "result.attributes: {:#?}",
            result.attributes
        );

        // Check correctness of the result
        let query_req = QueryMsg::Whitelist {};
        let binary =
            query(deps.as_ref(), testing::mock_env(), query_req).unwrap();
        let response: WhitelistResponse =
            cosmwasm_std::from_binary(&binary).unwrap();
        let expected_members: HashSet<String> = vec!["vitalik", "musk"]
            .iter()
            .map(|&s| s.to_string())
            .collect();
        assert_eq!(
            response.whitelist.members, expected_members,
            "got: {:#?}, wanted: {:#?}",
            response.whitelist.members, expected_members
        );
    }
}
