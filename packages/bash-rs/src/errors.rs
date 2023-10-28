use std::io;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum BashError {
    #[error("bash command failed with error: {}", err)]
    BashCmdFailed { err: String },

    #[error("BashError: {msg}")]
    General { msg: String },

    #[error("IO error: {0}")]
    IO(#[from] io::Error),
}
