use cosmwasm_std::{entry_point, DepsMut, Env, MessageInfo, Response};

use cw2::set_contract_version;

use crate::{
    error::ContractError,
    events::{
        denom_set_json, event_add_denom, event_change_denom, event_remove_denom,
    },
    msgs::{ExecuteMsg, InstantiateMsg, MigrateMsg},
    state::ACCEPTED_DENOMS,
};

#[cfg_attr(not(feature = "library"), cosmwasm_std::entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::ChangeDenom { from, to } => {
            cw_ownable::assert_owner(deps.storage, &info.sender)?;

            // "from" should be within the list of accepted denoms
            let mut denom_set = ACCEPTED_DENOMS.load(deps.storage)?;
            if !denom_set.contains(&from) {
                return Err(ContractError::RemoveNonexistentDenom {
                    denom: from.clone(),
                    denom_set,
                });
            }

            // Remove `from` and add `to` to the set
            denom_set.remove(&from);
            denom_set.insert(to.clone());
            ACCEPTED_DENOMS.save(deps.storage, &denom_set)?;

            let event = event_change_denom(
                from.as_str(),
                to.as_str(),
                denom_set_json(denom_set)?.as_str(),
            );

            Ok(Response::default().add_event(event))
        }
        ExecuteMsg::AddDenom { denom } => {
            cw_ownable::assert_owner(deps.storage, &info.sender)?;

            let mut denom_set = ACCEPTED_DENOMS.load(deps.storage)?;
            if denom_set.contains(&denom) {
                return Err(ContractError::AddExistentDenom {
                    denom,
                    denom_set: denom_set.clone(), // Cloning the set for error details
                });
            }
            denom_set.insert(denom.clone());
            ACCEPTED_DENOMS.save(deps.storage, &denom_set)?;

            let event =
                event_add_denom(&denom, denom_set_json(denom_set)?.as_str());
            Ok(Response::default().add_event(event))
        }

        ExecuteMsg::RemoveDenom { denom } => {
            cw_ownable::assert_owner(deps.storage, &info.sender)?;
            let mut denom_set = ACCEPTED_DENOMS.load(deps.storage)?;
            let was_present = denom_set.remove(&denom);
            if !was_present {
                return Err(ContractError::RemoveNonexistentDenom {
                    denom,
                    denom_set,
                });
            }
            ACCEPTED_DENOMS.save(deps.storage, &denom_set)?;

            let event = event_remove_denom(
                denom.as_str(),
                denom_set_json(denom_set)?.as_str(),
            );
            Ok(Response::default().add_event(event))
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

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(
    deps: DepsMut,
    _env: Env,
    _msg: MigrateMsg,
) -> Result<Response, ContractError> {
    // migrations::v2_0_0::migrate(deps, env, msg)

    // TODO: Handle state migration here.

    set_contract_version(
        deps.storage,
        format!("crates.io:{CONTRACT_NAME}"),
        CONTRACT_VERSION,
    )?;

    // TODO: from_version Fix this later.
    let from_version = "v0.1.0";

    Ok(Response::new()
        .add_attribute("action", "migrate")
        .add_attribute("from_version", from_version)
        .add_attribute("to_version", CONTRACT_VERSION))
}

pub const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
pub const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), cosmwasm_std::entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(
        deps.storage,
        format!("crates.io:{CONTRACT_NAME}"),
        CONTRACT_VERSION,
    )?;
    cw_ownable::initialize_owner(deps.storage, deps.api, Some(&msg.owner))?;
    ACCEPTED_DENOMS.save(deps.storage, &msg.accepted_denoms)?;
    Ok(Response::default())
}

#[cfg(test)]
pub mod tests {
    use cosmwasm_std::Response;

    use crate::{
        contract::execute,
        error::ContractError,
        msgs::{ExecuteMsg, QueryMsg},
        queries::query,
        testing::{self, TestResult, TEST_DENOM},
    };

    #[test]
    fn add_denom() -> TestResult {
        let accepted_denoms_init = vec![];
        let (mut deps, env, info) =
            testing::setup_contract(accepted_denoms_init)?;

        // Test adding a new denomination
        let denom = "testdenom".to_string();
        let msg = ExecuteMsg::AddDenom {
            denom: denom.clone(),
        };
        let res: Response =
            execute(deps.as_mut(), env.clone(), info.clone(), msg.clone())?;
        assert_eq!(0, res.messages.len());

        // Ensure the proper event is emitted
        let event = &res.events[0];
        assert_eq!(event.ty, "nusd_valuator/add_denom");
        assert_eq!(event.attributes.len(), 2);

        // Query the registered denoms
        let query_res =
            query(deps.as_ref(), env.clone(), QueryMsg::AcceptedDenoms {})?;
        let denoms: Vec<String> = serde_json::from_slice(&query_res)?;
        assert_eq!(denoms, vec![denom.clone()]);
        Ok(())
    }

    /// Attempting to add a denom that already exists should error.
    #[test]
    fn add_denom_err_existent() -> TestResult {
        let accepted_denoms_init: Vec<String> =
            [TEST_DENOM].iter().map(|s| s.to_string()).collect();
        let (mut deps, env, info) =
            testing::setup_contract(accepted_denoms_init)?;
        let denom = TEST_DENOM.to_string();
        let msg = ExecuteMsg::AddDenom {
            denom: denom.clone(),
        };
        let err = execute(deps.as_mut(), env.clone(), info.clone(), msg.clone())
            .expect_err("Expected error, but execute did not fail");
        assert_eq!(
            err,
            ContractError::AddExistentDenom {
                denom: denom.clone(),
                denom_set: vec![denom.clone()].into_iter().collect()
            }
        );

        // Ensure the state remains unchanged
        let query_res =
            query(deps.as_ref(), env.clone(), QueryMsg::AcceptedDenoms {})?;
        let denoms: Vec<String> = serde_json::from_slice(&query_res)?;
        assert_eq!(denoms, vec![denom.clone()]);
        Ok(())
    }

    #[test]
    fn remove_denom() -> TestResult {
        let accepted_denoms_init = vec![TEST_DENOM]
            .into_iter()
            .map(|s| s.to_string())
            .collect();
        let (mut deps, env, info) =
            testing::setup_contract(accepted_denoms_init)?;

        // Test removing a denomination
        let denom = TEST_DENOM.to_string();
        let msg = ExecuteMsg::RemoveDenom {
            denom: denom.clone(),
        };
        let res: Response =
            execute(deps.as_mut(), env.clone(), info.clone(), msg.clone())?;
        assert_eq!(0, res.messages.len());

        // Ensure the proper event is emitted
        let event = &res.events[0];
        assert_eq!(event.ty, "nusd_valuator/remove_denom");
        assert_eq!(event.attributes.len(), 2);

        // Query the registered denoms (should be empty now)
        let want_denom_set: Vec<String> = vec![];
        let query_res =
            query(deps.as_ref(), env.clone(), QueryMsg::AcceptedDenoms {})?;
        let denoms: Vec<String> = serde_json::from_slice(&query_res)?;
        assert_eq!(denoms, want_denom_set);

        // Attempt to remove the same denom again (should fail)
        let err = execute(deps.as_mut(), env.clone(), info.clone(), msg.clone())
            .unwrap_err();
        assert_eq!(
            err,
            ContractError::RemoveNonexistentDenom {
                denom,
                denom_set: want_denom_set.clone().into_iter().collect(),
            }
        );

        // Ensure the state remains unchanged
        let query_res =
            query(deps.as_ref(), env.clone(), QueryMsg::AcceptedDenoms {})?;
        let denoms: Vec<String> = serde_json::from_slice(&query_res)?;
        assert_eq!(denoms, want_denom_set);
        Ok(())
    }

    /// test: Removin a denom that isn't in the set should error.
    #[test]
    fn remove_denom_err_nonexistent() -> TestResult {
        let accepted_denoms_init: Vec<String> = vec![TEST_DENOM]
            .into_iter()
            .map(|s| s.to_string())
            .collect();
        let (mut deps, env, info) =
            testing::setup_contract(accepted_denoms_init.clone())?;

        // Attempt to remove a denom that's not in the set (should fail)
        let want_denom_set = accepted_denoms_init.clone();
        let denom = String::from("denom_not_in_set");
        let msg = ExecuteMsg::RemoveDenom {
            denom: denom.clone(),
        };
        let err = execute(deps.as_mut(), env.clone(), info.clone(), msg.clone())
            .unwrap_err();
        assert_eq!(
            err,
            ContractError::RemoveNonexistentDenom {
                denom,
                denom_set: want_denom_set.clone().into_iter().collect(),
            }
        );

        // Ensure the state remains unchanged
        let query_res =
            query(deps.as_ref(), env.clone(), QueryMsg::AcceptedDenoms {})?;
        let denoms: Vec<String> = serde_json::from_slice(&query_res)?;
        assert_eq!(denoms, want_denom_set);
        Ok(())
    }

    // TODO: test change denom
    #[test]
    fn change_denom() -> TestResult {
        Ok(())
    }

    // TODO: test update ownership
    #[test]
    fn update_ownership() -> TestResult {
        Ok(())
    }
}
