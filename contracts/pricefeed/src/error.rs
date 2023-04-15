use cosmwasm_std::{Addr, StdError};
#[cfg(feature = "backtraces")]
use std::backtrace::Backtrace;

use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("not implemented")]
    NotImplemented,

    #[error("unknown request")]
    UnknownRequest,

    #[error("invalid asset pair: {0}")]
    InvalidAssetPair(String),

    #[error("action unauthorized: for {0}")]
    Unauthorized(Addr),

    #[error("expired")]
    Expired,

    #[error("negative price")]
    NegativePrice,

    #[error("no valid price")]
    NoValidPrice,
}
