// A simple contract that maintains a whitelist of addresses.
// Takes inspiration from cw-plus/contracts/cw1-whitelist
//
// This example demonstrates a simple CosmWasm smart contract that manages a
// whitelist of addresses. The contract initializes with an admin address and
// allows the admin to add or remove addresses from the whitelist. Users can
// query whether an address is whitelisted or not.
//
// - InitMsg: Initializes the contract with the admin address.
// - ExecuteMsg: Enum representing the actions that can
//
// ### Contained Functionality
// 1. Initialize the contract with an admin address.
// 2. Allow the admin to add or remove addresses from the whitelist.
// 3. Allow anyone to query if an address is on the whitelist.

use std::collections::HashSet;

use cosmwasm_std::{
    attr, entry_point, Binary, Deps, DepsMut, Env, MessageInfo, Response,
    StdResult,
};

use crate::{
    msgs::{AdminsResponse, ExecuteMsg, InitMsg, IsMemberResponse, QueryMsg},
    // msg,
    state::{Whitelist, WHITELIST},
};

struct CanExecute {
    can: bool,
    whitelist: Whitelist,
}

fn can_execute(deps: Deps, sender: &str) -> StdResult<CanExecute> {
    let whitelist = WHITELIST.load(deps.storage).unwrap();
    Ok(CanExecute {
        can: whitelist.is_admin(sender),
        whitelist,
    })
}

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InitMsg,
) -> StdResult<Response> {
    let whitelist = Whitelist {
        members: HashSet::new(),
        admins: HashSet::from_iter(vec![msg.admin]),
    };
    WHITELIST.save(deps.storage, &whitelist)?;
    Ok(Response::default())
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response> {
    let deps_for_check = &deps;
    let admin_check: CanExecute =
        can_execute(deps_for_check.as_ref(), info.sender.as_ref())?;
    let ok = admin_check.can;
    let mut whitelist = admin_check.whitelist;

    if !ok {
        return Err(cosmwasm_std::StdError::generic_err("unauthorized"));
    }

    match msg {
        ExecuteMsg::Add { address } => {
            let api = deps.api;
            let addr = api.addr_validate(address.as_str()).unwrap();
            whitelist.admins.insert(addr.into_string());
            WHITELIST.save(deps.storage, &whitelist)?;
            Ok(Response::new().add_attributes(vec![
                attr("action", "add"),
                attr("address", address),
            ]))
        }
        ExecuteMsg::Remove { address } => {
            whitelist.admins.remove(address.as_str());
            WHITELIST.save(deps.storage, &whitelist)?;
            Ok(Response::new().add_attributes(vec![
                attr("action", "remove"),
                attr("address", address),
            ]))
        }
    }
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::IsMember { address } => {
            let whitelist = WHITELIST.load(deps.storage)?;
            let is_member: bool = whitelist.is_member(address);
            let res = IsMemberResponse { is_member };
            cosmwasm_std::to_binary(&res)
        }
        QueryMsg::Admins {} => {
            let whitelist = WHITELIST.load(deps.storage)?;
            let admins: Vec<String> = whitelist.admins.into_iter().collect();
            let res = AdminsResponse { admins };
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

        let execute_msg = ExecuteMsg::Add {
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
    fn test_execute_add() {
        // let mut deps = testing::mock_dependencies();
        // let admin = Addr::unchecked("admin");

        // let init_msg = InitMsg {
        //     admin: admin.as_str().to_string(),
        // };
        // let init_info = testing::mock_info("addr0000", &coins(2, "token"));
        // init(deps.as_mut(), testing::mock_env(), init_info, init_msg).unwrap();

        // let execute_msg = ExecuteMsg::Add {
        //     address: "addr0001".to_string(),
        // };
        // let execute_info = testing::mock_info(admin.as_str(), &[]);
        // let result = execute(deps.as_mut(), testing::mock_env(), execute_info, execute_msg).unwrap();
        // assert_eq!(result.messages.len(), 0);
        // assert_eq!(result.attributes.len(), 2);

        // let query_msg = QueryMsg::IsMember {
        //     address: "addr0001".to_string(),
        // };
        // let binary = query(deps.as_ref(), testing::mock_env(), query_msg).unwrap();
        // let response: IsMemberResponse = cosmwasm_std::from_binary(&binary).unwrap();
        // assert_eq!(response.is_member, true);
    }

    #[test]
    fn test_execute_remove() {
        let _deps = testing::mock_dependencies();
        // TODO test using cw-storage-plus
        // let admin = Addr::unchecked("admin");

        // let init_msg = InitMsg {
        //     admin: admin.as_str().to_string(),
        // };
        // let init_info = testing::mock_info("addr0000", &coins(2, "token"));
        // init(deps.as_mut(), testing::mock_env(), init_info, init_msg).unwrap();

        // let execute_msg = msg::ExecuteMsg::Add {
        //     address: "addr0001".to_string(),
        // };
        // let execute_info = testing::mock_info(admin.as_str(), &[]);
        // execute(deps.as_mut(), testing::mock_env(), execute_info.clone(), execute_msg).unwrap();

        // let execute_msg = ExecuteMsg::Remove {
        //     address: "addr0001".to_string(),
        // };
        // let result = execute(deps.as_mut(), testing::mock_env(), execute_info, execute_msg).unwrap();
        // assert_eq!(result.messages.len(), 0);
        // assert_eq!(result.attributes.len(), 2);

        // let query_msg = QueryMsg::IsMember {
        //     address: "addr0001".to_string(),
        // };
        // let binary = query(deps.as_ref(), testing::mock_env(), query_msg).unwrap();
        // let response: IsMemberResponse = cosmwasm_std::from_binary(&binary).unwrap();
        // assert_eq!(response.is_member, false);
    }
}
