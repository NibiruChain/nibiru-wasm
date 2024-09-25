use cosmwasm_std::StdError;
use std::collections::BTreeSet;

use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("serde_json error: {0}")]
    SerdeJson(String),

    #[error("{0}")]
    Ownership(#[from] nibiru_ownable::OwnershipError),

    // #[error("serde_json error: {err:?}")]
    // SerdeJson { err: serde_json::error::Error },
    #[error("not implemented")]
    NotImplemented,

    #[error("unknown request")]
    UnknownRequest,

    #[error("no need to remove denom {denom} from set {denom_set:?}")]
    RemoveNonexistentDenom {
        denom: String,
        denom_set: BTreeSet<String>,
    },

    #[error("no need to add denom {denom} to set {denom_set:?}")]
    AddExistentDenom {
        denom: String,
        denom_set: BTreeSet<String>,
    },
}

impl From<serde_json::Error> for ContractError {
    fn from(err: serde_json::Error) -> Self {
        ContractError::SerdeJson(err.to_string())
    }
}
