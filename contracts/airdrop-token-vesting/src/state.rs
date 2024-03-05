use std::collections::HashSet;

use cosmwasm_schema::cw_serde;

use cosmwasm_std::{StdResult, Uint128, Uint64};
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
    pub start_time: Uint64, // vesting start time in unix seconds
    pub cliff_time: Uint64, // cliff time in unix seconds
    pub end_time: Uint64,   // vesting end time in unix seconds
    pub cliff_amount: Uint128, // amount that will be unvested at cliff_time
    pub vesting_amount: Uint128, // total vesting amount
    pub claimed_amount: Uint128,
}

impl VestingAccount {
    pub fn vested_amount(&self, block_time: u64) -> StdResult<Uint128> {
        if block_time < self.cliff_time.u64() {
            return Ok(Uint128::zero());
        }

        if block_time == self.cliff_time.u64() {
            return Ok(self.cliff_amount);
        }

        if block_time >= self.end_time.u64() {
            return Ok(self.vesting_amount);
        }

        let remaining_token =
            self.vesting_amount.checked_sub(self.cliff_amount)?;
        let vested_token = remaining_token
            .checked_mul(Uint128::from(block_time - self.cliff_time.u64()))?
            .checked_div(Uint128::from(self.end_time - self.cliff_time))?;

        Ok(vested_token + self.cliff_amount)
    }
}
