use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error(transparent)]
    Std(#[from] cosmwasm_std::StdError),

    #[error("unauthorized")]
    Unauthorized(),

    #[error("invalid stake shares")]
    InvalidStakeShares(),
}
