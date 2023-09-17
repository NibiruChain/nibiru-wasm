use std::collections::HashSet;

use cosmwasm_schema::cw_serde;
use cw_storage_plus::Item;

pub const WHITELIST: Item<Whitelist> = Item::new("whitelist");

#[cw_serde]
pub struct Whitelist {
    pub members: HashSet<String>,
    pub admin: String,
}

impl Whitelist {
    pub fn has(&self, addr: impl AsRef<str>) -> bool {
        let addr = addr.as_ref();
        self.members.contains(addr) || self.admin == addr
    }

    pub fn is_admin(&self, addr: impl AsRef<str>) -> bool {
        let addr = addr.as_ref();
        self.admin == addr
    }

    pub fn is_member(&self, addr: impl AsRef<str>) -> bool {
        let addr = addr.as_ref();
        self.members.contains(addr)
    }
}

#[cfg(test)]
pub mod tests {
    use cosmwasm_std::testing::MockStorage;

    use super::*;

    pub fn init_mock_whitelist() -> Whitelist {
        let member_names = ["alice", "brock", "david"];
        let members: HashSet<String> =
            member_names.iter().map(|&s| s.to_string()).collect();
        let admin: String = "cait".to_string();
        Whitelist { members, admin }
    }

    #[test]
    fn whitelist_is_admin() {
        let whitelist = init_mock_whitelist();
        assert!(!whitelist.is_admin("alice"));
        assert!(whitelist.is_admin("cait"));
        assert!(!whitelist.is_admin("david"));
        assert!(!whitelist.is_admin("brock"));
    }

    #[test]
    fn whitelist_is_member() {
        let whitelist = init_mock_whitelist();
        assert!(whitelist.is_member("alice"));
        assert!(!whitelist.is_member("cait"));
        assert!(whitelist.is_member("david"));
        assert!(whitelist.is_member("brock"));
    }

    #[test]
    fn whitelist_has() {
        let whitelist = init_mock_whitelist();

        let whitelisted_names = ["alice", "brock", "cait", "david"];
        for name in whitelisted_names.iter() {
            assert!(whitelist.has(name));
        }

        let other_names = ["xxx", "not-whitelisted"];
        for name in other_names.iter() {
            assert!(!whitelist.has(name));
        }
    }

    #[test]
    fn save_and_load() {
        let mut store = MockStorage::new();

        // Store should start out empty
        assert!(WHITELIST.load(&store).is_err());
        assert_eq!(WHITELIST.may_load(&store).unwrap(), None);

        // save to store
        let whitelist = init_mock_whitelist();
        let res = WHITELIST.save(&mut store, &whitelist);
        assert!(res.is_ok());

        // load from store
        assert_eq!(whitelist, WHITELIST.load(&store).unwrap());
    }
}
