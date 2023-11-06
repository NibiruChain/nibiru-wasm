use thiserror::Error;

use crate::{
    system_info::{Arch, OS},
    tools::Binary,
};

#[derive(Debug, Error)]
pub enum BashError {
    #[error("bash command '{}' failed with error: {}", cmd, err)]
    BashCmdFailed { cmd: String, err: String },

    #[error("BashError: {}", msg)]
    General { msg: String },
}

#[derive(Debug, Error)]
pub enum SystemInfoError {
    #[error("config error: {}", err_msg)]
    Std { err_msg: String },

    #[error("{}", bash_err)]
    BashError { bash_err: BashError },

    #[error("neither wget/curl are installed")]
    CurlVariantUnknown,

    #[error("No release artifact available for system with {{ os: {} , cpu_architecture: {} }}", os, cpu_arch)]
    NoReleaseArtifact { os: OS, cpu_arch: Arch },

    #[error("failed to fetch latest release for {}", binary)]
    FailedToFetchLatestRelease { binary: Binary },
}

#[derive(Debug, Error)]
pub enum LocalError {
    #[error("config error: {}", err)]
    Std { err: String },

    #[error("failed to find $HOME directory")]
    FailedToFindHomeDir,

    #[error("failed to create ~/.local/nibiru_dev directory: {}", err)]
    FailedToCreateRootDir { err: &'static str },

    #[error("inner local error: {}", err)]
    InnerError { err: anyhow::Error },
}
