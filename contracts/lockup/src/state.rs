use cosmwasm_std::{Addr, Coin, Uint128};
use cw_storage_plus::{
    Index, IndexList, IndexedMap, Item, Map, MultiIndex, SnapshotItem, SnapshotMap,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::ops::Mul;

pub const NOT_UNLOCKING_BLOCK_IDENTIFIER: u64 = u64::MAX;
pub const LOCKS_ID: Item<u64> = Item::new("locks_id");

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct Lock {
    pub id: u64,
    pub coin: Coin,
    pub owner: Addr,
    pub duration_blocks: u64,
    pub start_block: u64,
    pub end_block: u64,
    pub funds_withdrawn: bool,
}

pub struct LockIndexes<'a> {
    pub denom_end: MultiIndex<'a, (String, u64), Lock, u64>,
    pub addr_denom_end: MultiIndex<'a, (Addr, String, u64), Lock, u64>,
}

impl<'a> IndexList<Lock> for LockIndexes<'a> {
    fn get_indexes(&'_ self) -> Box<dyn Iterator<Item = &'_ dyn Index<Lock>> + '_> {
        let v: Vec<&dyn Index<Lock>> = vec![&self.addr_denom_end, &self.denom_end];
        Box::new(v.into_iter())
    }
}

pub fn locks<'a>() -> IndexedMap<'a, u64, Lock, LockIndexes<'a>> {
    let indexes = LockIndexes {
        addr_denom_end: MultiIndex::new(
            |lock| -> (_, _, _) { (lock.owner.clone(), lock.coin.denom.clone(), lock.end_block) },
            "locks",
            "denom_end_addr",
        ),
        denom_end: MultiIndex::new(
            |lock| -> (_, _) { (lock.coin.denom.clone(), lock.end_block) },
            "locks",
            "denom_end",
        ),
    };

    return IndexedMap::new("locks", indexes);
}
