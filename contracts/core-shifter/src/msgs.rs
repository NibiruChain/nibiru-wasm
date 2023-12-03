use cosmwasm_schema::{cw_serde, QueryResponses};
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
    EditOpers(operator_perms::Action),
}

pub mod operator_perms {
    use cosmwasm_schema::cw_serde;

    #[cw_serde]
    pub enum Action {
        AddOper { address: String },
        RemoveOper { address: String },
    }
}

/// QueryMsg specifies the args for the query entry point of the contract.
#[derive(QueryResponses)]
#[cw_serde]
pub enum QueryMsg {
    /// HasPerms: Query whether the given address has operator permissions.
    /// The query response showcases the contract owner and set of operators.
    #[returns(HasPermsResponse)]
    HasPerms { address: String },
    /// Perms: Query the contract owner and set of operators.
    #[returns(PermsResponse)]
    Perms {},
}

#[cw_serde]
pub struct HasPermsResponse {
    pub has_perms: bool,
    pub addr: String,
    pub perms: Permissions,
}

#[cw_serde]
pub struct PermsResponse {
    pub perms: Permissions,
}
