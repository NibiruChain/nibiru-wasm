use cosmwasm_std::{StdError};
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Invalid pair: {0}")]
    InvalidPair (String),

    #[error("Invalid quote asset amount: {0}")]
    InvalidQuoteAssetAmount(String),

    #[error("Invalid base asset amount limit: {0}")]
    InvalidBaseAssetAmountLimit(String),

    #[error("Unauthorized")]
    Unauthorized {},
}
