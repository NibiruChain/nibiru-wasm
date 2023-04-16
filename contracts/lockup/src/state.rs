use cosmwasm_std::{Addr, Coin};
use cw_storage_plus::{Index, IndexList, IndexedMap, Item, MultiIndex};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

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
    pub denom_start: MultiIndex<'a, (String, u64), Lock, u64>,
    pub addr_denom_start: MultiIndex<'a, (Addr, String, u64), Lock, u64>,
}

impl<'a> IndexList<Lock> for LockIndexes<'a> {
    fn get_indexes(
        &'_ self,
    ) -> Box<dyn Iterator<Item = &'_ dyn Index<Lock>> + '_> {
        let v: Vec<&dyn Index<Lock>> = vec![
            &self.addr_denom_end,
            &self.denom_end,
            &self.denom_start,
            &self.addr_denom_start,
        ];
        Box::new(v.into_iter())
    }
}

pub fn locks<'a>() -> IndexedMap<'a, u64, Lock, LockIndexes<'a>> {
    let indexes = LockIndexes {
        addr_denom_end: MultiIndex::new(
            |_bz, lock: &Lock| -> (_, _, _) {
                (lock.owner.clone(), lock.coin.denom.clone(), lock.end_block)
            },
            "locks",
            "addr_denom_end",
        ),
        denom_start: MultiIndex::new(
            |_bz, lock: &Lock| -> (_, _) {
                (lock.coin.denom.clone(), lock.start_block)
            },
            "locks",
            "denom_start",
        ),

        denom_end: MultiIndex::new(
            |_bz, lock: &Lock| -> (_, _) {
                (lock.coin.denom.clone(), lock.end_block)
            },
            "locks",
            "denom_end",
        ),
        addr_denom_start: MultiIndex::new(
            |_bz, lock: &Lock| -> (_, _, _) {
                (
                    lock.owner.clone(),
                    lock.coin.denom.clone(),
                    lock.start_block,
                )
            },
            "locks",
            "addr_denom_start",
        ),
    };

    return IndexedMap::new("locks", indexes);
}
