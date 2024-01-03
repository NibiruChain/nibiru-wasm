use cosmwasm_std::{Uint128, Addr};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct InstantiateMsg {
    pub campaign_id: String,
    pub campaign_name: String,
    pub campaign_description: String,
    pub managers: Vec<Addr>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct RewardUserRequest {
    pub user_address: Addr,
    pub amount: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct RewardUserResponse {
    pub user_address: Addr,
    pub success: bool,
    pub error_msg: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    RewardUsers {
        requests: Vec<RewardUserRequest>
    },
    Claim {},
    Withdraw {
        amount: Uint128,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    Campaign { },
    GetUserReward { user_address: Addr },
}

