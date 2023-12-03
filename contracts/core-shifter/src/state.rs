use std::collections::BTreeSet;

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Api, Storage};
use cw_storage_plus::Item;

use crate::error::ContractError;

pub const OPERATORS: Item<BTreeSet<String>> = Item::new("operators");

#[cw_serde]
pub struct Permissions {
    pub owner: Option<String>,
    pub operators: BTreeSet<String>,
}

impl Permissions {
    pub fn has(&self, addr: impl AsRef<str>) -> bool {
        let addr = addr.as_ref();
        self.operators.contains(addr) || self.is_owner(addr)
    }

    pub fn is_owner(&self, addr: impl AsRef<str>) -> bool {
        let addr = addr.as_ref();
        if let Some(owner) = &self.owner {
            owner == addr
        } else {
            false
        }
    }

    pub fn is_operator(&self, addr: impl AsRef<str>) -> bool {
        let addr = addr.as_ref();
        self.operators.contains(addr)
    }

    pub fn load(storage: &dyn Storage) -> Result<Self, ContractError> {
        let owner = cw_ownable::get_ownership(storage)?.owner;
        let opers = OPERATORS.load(storage)?;
        Ok(Permissions {
            owner: owner.map(|addr| addr.into_string()),
            operators: opers,
        })
    }
}

/// Set the given address as the contract owner and initialize the
/// 'OPERATORS' and 'OWNERSHIP' state. This function is only intended to be used only
/// during contract instantiation.
pub fn instantiate_perms(
    owner: Option<&str>,
    storage: &mut dyn Storage,
    api: &dyn Api,
) -> Result<(), ContractError> {
    cw_ownable::initialize_owner(storage, api, owner)?;
    Ok(OPERATORS.save(storage, &BTreeSet::default())?)
}

#[cfg(test)]
pub mod tests {
    use crate::contract::tests::TestResult;
    use cosmwasm_std::testing::MockStorage;

    use super::*;

    pub fn init_mock_perms() -> Permissions {
        let member_names = ["alice", "brock", "david"];
        let members: BTreeSet<String> =
            member_names.iter().map(|&s| s.to_string()).collect();
        let admin: String = "cait".to_string();
        Permissions {
            operators: members,
            owner: Some(admin),
        }
    }

    #[test]
    fn perms_is_owner() {
        let perms = init_mock_perms();
        assert!(!perms.is_owner("alice"));
        assert!(perms.is_owner("cait"));
        assert!(!perms.is_owner("david"));
        assert!(!perms.is_owner("brock"));
    }

    #[test]
    fn perms_is_member() {
        let perms = init_mock_perms();
        assert!(perms.is_operator("alice"));
        assert!(!perms.is_operator("cait"));
        assert!(perms.is_operator("david"));
        assert!(perms.is_operator("brock"));
    }

    #[test]
    fn perms_has() {
        let perms = init_mock_perms();

        let permsed_names = ["alice", "brock", "cait", "david"];
        for name in permsed_names.iter() {
            assert!(perms.has(name));
        }

        let other_names = ["xxx", "not-permsed"];
        for name in other_names.iter() {
            assert!(!perms.has(name));
        }
    }

    #[test]
    fn save_and_load() -> TestResult {
        let mut store = MockStorage::new();

        // Store should start out empty
        assert!(OPERATORS.load(&store).is_err());
        assert_eq!(OPERATORS.may_load(&store)?, None);

        // save to store
        let perms = init_mock_perms();
        let opers = perms.operators;
        let res = OPERATORS.save(&mut store, &opers);
        assert!(res.is_ok());

        // load from store
        assert_eq!(opers, OPERATORS.load(&store)?);
        Ok(())
    }
}