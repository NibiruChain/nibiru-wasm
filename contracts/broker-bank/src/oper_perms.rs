use std::collections::BTreeSet;

use cosmwasm_schema::cw_serde;
use cosmwasm_std::Storage;

use crate::{error::ContractError, state::OPERATORS};

#[cw_serde]
pub enum Action {
    AddOper { address: String },
    RemoveOper { address: String },
}

#[cw_serde]
pub struct HasPermsResponse {
    pub has_perms: bool,
    pub addr: String,
    pub perms: Permissions,
}

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
        let owner = nibiru_ownable::get_ownership(storage)?.owner;
        let opers = OPERATORS.load(storage)?;
        Ok(Permissions {
            owner: owner.map(|addr| addr.to_string()),
            operators: opers,
        })
    }

    pub fn assert_operator(
        storage: &dyn Storage,
        addr: String,
    ) -> Result<Self, ContractError> {
        let perms = Self::load(storage)?;
        match perms.is_operator(&addr) || perms.is_owner(&addr) {
            true => Ok(perms),
            false => Err(ContractError::NoOperatorPerms { addr }),
        }
    }
}
