//! testing.rs: Test helpers for the contract

use cosmwasm_std::{
    testing::{
        mock_dependencies, mock_env, mock_info, MockApi, MockQuerier,
        MockStorage,
    },
    Env, MessageInfo, OwnedDeps,
};

use crate::{contract::instantiate, msgs::InitMsg};

pub type TestResult = anyhow::Result<()>;

pub const TEST_OWNER: &str = "owner";

pub fn setup_contract(// accepted_denoms: Vec<String>,
) -> anyhow::Result<(
    OwnedDeps<MockStorage, MockApi, MockQuerier>,
    Env,
    MessageInfo,
)> {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info(TEST_OWNER, &[]);

    let msg = InitMsg {
        owner: info.sender.to_string(),
    };
    let res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg)?;
    assert_eq!(0, res.messages.len());
    Ok((deps, env, info))
}

pub fn mock_info_for_sender(sender: &str) -> MessageInfo {
    mock_info(sender, &[])
}
