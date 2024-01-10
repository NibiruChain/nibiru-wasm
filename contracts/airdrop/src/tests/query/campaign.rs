use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
use cosmwasm_std::{coins, from_json, Addr, Uint128};

use crate::contract::{instantiate, query_campaign};
use crate::msg::InstantiateMsg;
use crate::state::Campaign;

#[test]
fn test_query_campaign() {
    let mut deps = mock_dependencies();
    let env = mock_env();

    instantiate(
        deps.as_mut(),
        env.clone(),
        mock_info("owner", &coins(1000, "")),
        InstantiateMsg {
            campaign_id: "campaign_id".to_string(),
            campaign_name: "campaign_name".to_string(),
            campaign_description: "campaign_description".to_string(),
            managers: vec![
                Addr::unchecked("manager1"),
                Addr::unchecked("manager2"),
            ],
        },
    )
    .unwrap();

    let res = query_campaign(deps.as_ref(), env.clone()).unwrap();
    let campaign: Campaign = from_json(res).unwrap();
    assert_eq!(
        campaign,
        Campaign {
            campaign_name: "campaign_name".to_string(),
            campaign_description: "campaign_description".to_string(),
            owner: Addr::unchecked("owner"),
            managers: vec![
                Addr::unchecked("manager1"),
                Addr::unchecked("manager2")
            ],
            unallocated_amount: Uint128::new(1000),
            is_active: true,
        }
    );
}
