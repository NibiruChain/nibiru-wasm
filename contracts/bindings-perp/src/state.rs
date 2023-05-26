use std::collections::HashSet;

use cosmwasm_schema::cw_serde;
use cw_storage_plus::Item;

pub const SUDOERS: Item<Sudoers> = Item::new("sudoers");

#[cw_serde]
pub struct Sudoers {
    pub members: HashSet<String>,
    pub admin: String,
}

impl Sudoers {
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

    pub fn init_mock_sudoers() -> Sudoers {
        let member_names = vec!["alice", "brock", "david"];
        let members: HashSet<String> =
            member_names.iter().map(|&s| s.to_string()).collect();
        let admin: String = "cait".to_string();
        Sudoers { members, admin }
    }

    #[test]
    fn sudoers_is_admin() {
        let sudoers = init_mock_sudoers();
        assert!(!sudoers.is_admin("alice"));
        assert!(sudoers.is_admin("cait"));
        assert!(!sudoers.is_admin("david"));
        assert!(!sudoers.is_admin("brock"));
    }

    #[test]
    fn sudoers_is_member() {
        let sudoers = init_mock_sudoers();
        assert!(sudoers.is_member("alice"));
        assert!(!sudoers.is_member("cait"));
        assert!(sudoers.is_member("david"));
        assert!(sudoers.is_member("brock"));
    }

    #[test]
    fn sudoers_has() {
        let sudoers = init_mock_sudoers();

        let sudoersed_names = ["alice", "brock", "cait", "david"];
        for name in sudoersed_names.iter() {
            assert!(sudoers.has(name));
        }

        let other_names = ["xxx", "not-sudoersed"];
        for name in other_names.iter() {
            assert!(!sudoers.has(name));
        }
    }

    #[test]
    fn save_and_load() {
        let mut store = MockStorage::new();

        // Store should start out empty
        assert!(SUDOERS.load(&store).is_err());
        assert_eq!(SUDOERS.may_load(&store).unwrap(), None);

        // save to store
        let sudoers = init_mock_sudoers();
        let res = SUDOERS.save(&mut store, &sudoers);
        assert!(res.is_ok());

        // load from store
        assert_eq!(sudoers, SUDOERS.load(&store).unwrap());
    }
}
