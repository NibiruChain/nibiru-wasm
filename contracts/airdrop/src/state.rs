use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Map, Item};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Campaign {
    pub campaign_id: String,
    pub campaign_name: String,
    pub campaign_description: String,

    pub unallocated_amount: Uint128,
    pub owner: Addr,
}

pub const CAMPAIGN: Item<Campaign> = Item::new("campaign");
pub const USER_REWARDS: Map<Addr, Uint128> = Map::new("user_rewards");
