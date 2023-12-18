use std::str::FromStr;

use cosmwasm_std::{
    attr, entry_point, Binary, CosmosMsg, Deps, DepsMut, Env, MessageInfo,
    Response,
};
use cw2::set_contract_version;
use nibiru_std::{
    math::SdkDec,
    proto::{nibiru, NibiruStargateMsg},
};

use crate::{
    error::ContractError,
    msgs::{
        operator_perms, ExecuteMsg, HasPermsResponse, InitMsg, PermsResponse,
        QueryMsg,
    },
    state::{instantiate_perms, Permissions, OPERATORS},
};

pub const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
pub const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InitMsg,
) -> Result<Response, ContractError> {
    set_contract_version(
        deps.storage,
        format!("crates.io:{CONTRACT_NAME}"),
        CONTRACT_VERSION,
    )?;
    instantiate_perms(Some(&msg.owner), deps.storage, deps.api)?;
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    let deps_for_check = &deps;
    let check = CanExecute::new(deps_for_check.as_ref(), info.sender.as_ref())?;
    let mut perms = check.perms.clone();

    let contract_addr = env.contract.address.to_string();
    match msg {
        ExecuteMsg::ShiftSwapInvariant {
            pair,
            new_swap_invariant,
        } => {
            check.check_perms_operator()?;
            let cosmos_msg: CosmosMsg = nibiru::perp::MsgShiftSwapInvariant {
                sender: contract_addr,
                pair,
                new_swap_invariant: new_swap_invariant.to_string(),
            }
            .into_stargate_msg();
            let res = Response::new()
                .add_message(cosmos_msg)
                .add_attributes(vec![attr("action", "shift_swap_invariant")]);
            Ok(res)
        }

        ExecuteMsg::ShiftPegMultiplier { pair, new_peg_mult } => {
            check.check_perms_operator()?;
            let cosmos_msg: CosmosMsg = nibiru::perp::MsgShiftPegMultiplier {
                sender: contract_addr,
                pair,
                new_peg_mult: SdkDec::from_str(&new_peg_mult)?.pb_repr(),
            }
            .into_stargate_msg();
            let res = Response::new()
                .add_message(cosmos_msg)
                .add_attributes(vec![attr("action", "shift_peg_multiplier")]);
            Ok(res)
        }

        ExecuteMsg::EditOpers(action) => {
            check.check_perms_operator()?;
            let api = deps.api;
            match action {
                operator_perms::Action::AddOper { address } => {
                    let addr = api.addr_validate(address.as_str())?;
                    perms.operators.insert(addr.into_string());
                    OPERATORS.save(deps.storage, &perms.operators)?;

                    let res = Response::new().add_attributes(vec![
                        attr("action", "add_operator"),
                        attr("address", address),
                    ]);
                    Ok(res)
                }

                operator_perms::Action::RemoveOper { address } => {
                    perms.operators.remove(address.as_str());
                    OPERATORS.save(deps.storage, &perms.operators)?;

                    let res = Response::new().add_attributes(vec![
                        attr("action", "remove_operator"),
                        attr("address", address),
                    ]);
                    Ok(res)
                }
            }
        }

        ExecuteMsg::UpdateOwnership(action) => {
            Ok(execute_update_ownership(deps, env, info, action)?)
        }
    }
}

fn execute_update_ownership(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    action: cw_ownable::Action,
) -> Result<Response, cw_ownable::OwnershipError> {
    let ownership =
        cw_ownable::update_ownership(deps, &env.block, &info.sender, action)?;
    Ok(Response::new().add_attributes(ownership.into_attributes()))
}

struct CanExecute {
    is_owner: bool,
    is_operator: bool,
    sender: String,
    perms: Permissions,
}

impl CanExecute {
    pub fn new(deps: Deps, sender: &str) -> Result<Self, ContractError> {
        let perms = Permissions::load(deps.storage)?;
        Ok(CanExecute {
            is_owner: perms.is_owner(sender),
            is_operator: perms.is_operator(sender),
            sender: sender.into(),
            perms,
        })
    }

    /// Errors if the sender does not have operator permissions.
    pub fn check_perms_operator(&self) -> Result<(), ContractError> {
        match self.is_operator || self.is_owner {
            true => Ok(()),
            false => Err(ContractError::NoOperatorPerms {
                sender: self.sender.to_string(),
            }),
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(
    deps: Deps,
    _env: Env,
    msg: QueryMsg,
) -> Result<Binary, ContractError> {
    match msg {
        QueryMsg::HasPerms { address } => {
            let perms = Permissions::load(deps.storage)?;
            let has_perms: bool = perms.is_operator(&address);
            let res = HasPermsResponse {
                has_perms,
                perms,
                addr: address,
            };
            Ok(cosmwasm_std::to_json_binary(&res)?)
        }
        QueryMsg::Perms {} => {
            let perms = Permissions::load(deps.storage)?;
            let res = PermsResponse { perms };
            Ok(cosmwasm_std::to_json_binary(&res)?)
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::{
        msgs::{ExecuteMsg, InitMsg},
        state::OPERATORS,
        testing::{self as t, TestResult},
    };

    use cosmwasm_std::{coins, testing};
    use std::collections::BTreeSet;

    // ---------------------------------------------------------------------------
    // Tests
    // ---------------------------------------------------------------------------

    #[test]
    fn test_instantiate() -> TestResult {
        let mut deps = testing::mock_dependencies();
        let msg = InitMsg {
            owner: "admin".to_string(),
        };
        let info: MessageInfo =
            testing::mock_info("addr0000", &coins(2, "token"));

        let result = instantiate(deps.as_mut(), testing::mock_env(), info, msg)?;

        assert_eq!(result.messages.len(), 0);
        Ok(())
    }

    #[test]
    fn test_has_admin_power() -> TestResult {
        let sender = "not-admin";
        let (deps, _env, _info) = t::setup_contract()?;
        let perms = Permissions::load(&deps.storage)?;
        let not_has: bool = !perms.is_owner(sender);
        assert!(not_has);
        let sender = t::TEST_OWNER;
        let has: bool = perms.is_owner(sender);
        assert!(has);
        Ok(())
    }

    #[test]
    fn test_exec_unauthorized() -> TestResult {
        let (mut deps, _env, _info) = t::setup_contract()?;
        let execute_msg =
            ExecuteMsg::EditOpers(operator_perms::Action::AddOper {
                address: "addr0001".to_string(),
            });
        let unauthorized_info = testing::mock_info("unauthorized", &[]);
        let result = execute(
            deps.as_mut(),
            testing::mock_env(),
            unauthorized_info,
            execute_msg,
        );
        assert!(result.is_err());
        Ok(())
    }

    #[test]
    fn test_exec_edit_opers_add() -> TestResult {
        let (mut deps, _env, _info) = t::setup_contract()?;
        let new_member = "new_member";
        let perms = Permissions::load(&deps.storage)?;
        let not_has: bool = !perms.is_owner(new_member);
        assert!(not_has);

        // Add an operator to the permission set
        let execute_msg =
            ExecuteMsg::EditOpers(operator_perms::Action::AddOper {
                address: new_member.to_string(),
            });
        let sender = t::TEST_OWNER;
        let execute_info = testing::mock_info(sender, &[]);

        let check_resp = |resp: Response| {
            assert_eq!(
                resp.messages.len(),
                0,
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
        )?;
        check_resp(result);

        // Check correctness of the result
        let perms = Permissions::load(&deps.storage)?;
        let has: bool = perms.has(new_member);
        assert!(has);

        let query_req = QueryMsg::HasPerms {
            address: new_member.to_string(),
        };
        let binary = query(deps.as_ref(), testing::mock_env(), query_req)?;
        let response: HasPermsResponse = cosmwasm_std::from_json(binary)?;
        assert!(response.has_perms);
        Ok(())
    }

    #[test]
    fn test_exec_edit_opers_remove() -> TestResult {
        let (mut deps, _env, _info) = t::setup_contract()?;
        // Set up initial perms
        let opers_start: Vec<String> = ["vitalik", "musk", "satoshi"]
            .iter()
            .map(|&s| s.to_string())
            .collect();
        let mut perms = Permissions::load(&deps.storage)?;
        assert_eq!(perms.operators.len(), 0); // admin remains
        for member in opers_start.iter() {
            perms.operators.insert(member.clone());
        }
        let res = OPERATORS.save(deps.as_mut().storage, &perms.operators);
        assert!(res.is_ok());

        // Remove a member from the whitelist
        let execute_msg =
            ExecuteMsg::EditOpers(operator_perms::Action::RemoveOper {
                address: "satoshi".to_string(),
            });
        let sender = t::TEST_OWNER;
        let execute_info = testing::mock_info(sender, &[]);
        let check_resp = |resp: Response| {
            assert_eq!(
                resp.messages.len(),
                0,
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
        )?;
        check_resp(result);

        // Check correctness of the result
        let query_req = QueryMsg::Perms {};
        let binary = query(deps.as_ref(), testing::mock_env(), query_req)?;
        let response: PermsResponse = cosmwasm_std::from_json(binary)?;
        let expected_opers: BTreeSet<String> =
            ["vitalik", "musk"].iter().map(|&s| s.to_string()).collect();
        assert_eq!(
            response.perms.operators, expected_opers,
            "got: {:#?}, wanted: {:#?}",
            response.perms.operators, expected_opers
        );
        Ok(())
    }

    /// TODO: test change owner
    #[test]
    fn test_exec_change_admin() -> TestResult {
        Ok(())
    }
}
