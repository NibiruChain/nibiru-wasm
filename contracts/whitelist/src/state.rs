use std::collections::HashSet;

use cw_storage_plus::Item;
use schemars::JsonSchema;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, JsonSchema, Debug, Default)]
pub struct Whitelist {
    members: HashSet<String>,
    admins: HashSet<String>,
}

impl Whitelist {
    pub fn has(&self, addr: impl AsRef<str>) -> bool {
        let addr = addr.as_ref();
        return self.members.contains(addr) || self.admins.contains(addr); 
    }

    pub fn is_admin(&self, addr: impl AsRef<str>) -> bool {
        let addr = addr.as_ref();
        return self.admins.contains(addr);
    }
}

pub const WHITELIST: Item<Whitelist> = Item::new("whitelist");


