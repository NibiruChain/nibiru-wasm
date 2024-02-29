use cosmwasm_schema::cw_serde;
use cw_storage_plus::Item;
use std::collections::HashSet;

pub const WHITELIST: Item<Whitelist> = Item::new("whitelist");
pub const COMPOUNDER_ON: Item<bool> = Item::new("compounder_on");

#[cw_serde]
pub struct Whitelist {
    pub managers: HashSet<String>,
    pub admin: String,
}

impl Whitelist {
    pub fn is_admin(&self, addr: impl AsRef<str>) -> bool {
        let addr = addr.as_ref();
        self.admin == addr
    }

    pub fn is_manager(&self, addr: impl AsRef<str>) -> bool {
        let addr = addr.as_ref();
        self.managers.contains(addr)
    }
}
