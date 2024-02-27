use std::collections::HashSet;

use cosmwasm_schema::cw_serde;

use crate::msg::VestingSchedule;
use cosmwasm_std::Uint128;
use cw_storage_plus::{Item, Map};

pub const VESTING_ACCOUNTS: Map<&str, VestingAccount> =
    Map::new("vesting_accounts");
pub const UNALLOCATED_AMOUNT: Item<Uint128> = Item::new("unallocated_amount");
pub const DENOM: Item<String> = Item::new("denom");
pub const WHITELIST: Item<Whitelist> = Item::new("whitelist");

#[cw_serde]
pub struct Whitelist {
    pub members: HashSet<String>,
    pub admin: String,
}

impl Whitelist {
    pub fn is_admin(&self, addr: impl AsRef<str>) -> bool {
        let addr = addr.as_ref();
        self.admin == addr
    }

    pub fn is_member(&self, addr: impl AsRef<str>) -> bool {
        let addr = addr.as_ref();
        self.members.contains(addr)
    }
}

#[cw_serde]
pub struct VestingAccount {
    pub address: String,
    pub vesting_amount: Uint128,
    pub vesting_schedule: VestingSchedule,
    pub claimed_amount: Uint128,
}
