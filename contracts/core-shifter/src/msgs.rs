use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Decimal, Uint256};

use crate::state::Permissions;

/// InitMsg specifies the args for the instantiate entry point of the contract.
#[cw_serde]
pub struct InitMsg {
    pub owner: String,
}

/// ExecuteMsg specifies the args for the execute entry point of the contract.
#[cw_ownable::cw_ownable_execute]
#[cw_serde]
pub enum ExecuteMsg {
    ShiftSwapInvariant {
        pair: String,
        new_swap_invariant: Uint256,
    },
    ShiftPegMultiplier {
        pair: String,
        new_peg_mult: Decimal,
    },
    AddMember {
        address: String,
    },
    RemoveMember {
        address: String,
    },
    ChangeAdmin {
        address: String,
    },
}

/// QueryMsg specifies the args for the query entry point of the contract.
#[cw_serde]
pub enum QueryMsg {
    HasPerms { address: String },
    Perms {},
}

#[cw_serde]
pub struct HasPermsResponse {
    pub has_perms: bool,
    pub addr: String,
    pub perms: Permissions,
}

#[cw_serde]
pub struct PermissionsResponse {
    pub perms: Permissions,
}
