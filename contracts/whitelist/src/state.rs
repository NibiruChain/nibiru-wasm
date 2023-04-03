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


mod tests {
    use cosmwasm_std::testing::MockStorage;

    use crate::contract::init;

    use super::*;

    fn init_mock_whitelist() -> Whitelist {

        let members: HashSet<String> = HashSet::from_iter(vec![
            "brock".to_string(), 
            "david".to_string(), 
        ]);
        let admins: HashSet<String> = HashSet::from_iter(vec![
            "alice".to_string(), 
            "cait".to_string(), 
        ]);
        return Whitelist { members, admins }
    }

    #[test]
    fn whitelist_is_admin() {
        let whitelist  = init_mock_whitelist();
        assert!(whitelist.is_admin("alice"));
        assert!(whitelist.is_admin("cait"));
        assert!(!whitelist.is_admin("david"));
        assert!(!whitelist.is_admin("brock"));
    }

    #[test]
    fn whitelist_has() {
        let whitelist  = init_mock_whitelist();
        assert!(whitelist.has("alice"));
        assert!(whitelist.has("brock"));
        assert!(whitelist.has("cait"));
        assert!(whitelist.has("david"));

        assert!(!whitelist.has("xxx"));
        assert!(!whitelist.has("not-whitelisted"));
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