use thiserror::Error;

#[derive(Debug, Error)]
pub enum MsigError {
    #[error("msig error: bash cmd failed with error \"{}\"", err)]
    BashCmdFailed { err: BashError },

    #[error("Keyring operation failed.")]
    KeyringOperationFailed,

    #[error("Failed to parse data.")]
    ParseError,

    #[error("MsigError: {}", err_msg)]
    General { err_msg: &'static str },
}

#[derive(Debug, thiserror::Error)]
pub enum BashError {
    #[error("bash command '{}' failed with error: {}", cmd, err)]
    BashCmdFailed { cmd: String, err: anyhow::Error },

    #[error("BashError: {}", err_msg)]
    General { err_msg: String },
}

impl From<BashError> for MsigError {
    fn from(err: BashError) -> Self {
        MsigError::BashCmdFailed { err }
    }
}
