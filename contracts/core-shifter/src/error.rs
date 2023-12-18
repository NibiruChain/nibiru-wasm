use cosmwasm_std::StdError;

use nibiru_std::errors;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0}")]
    Ownership(#[from] cw_ownable::OwnershipError),

    #[error("insufficient permissions: sender is not a contract operator ({sender:?})")]
    NoOperatorPerms { sender: String },

    #[error("{0}")]
    MathError(#[from] errors::MathError),
}
