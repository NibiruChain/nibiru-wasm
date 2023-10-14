use thiserror::Error;

use cosmwasm_std as cw;

#[derive(Error, Debug, PartialEq)]
pub enum NibiruError {
    #[error("{0}")]
    CwStd(#[from] cw::StdError),

    #[error("no prost::Name implementation for type {}, where prost::Name.type_url() is needed.", type_name)]
    NoTypeUrl { type_name: String },
}

impl From<NibiruError> for cw::StdError {
    fn from(err: NibiruError) -> cw::StdError {
        match err {
            NibiruError::CwStd(e) => e,
            e => cw::StdError::generic_err(e.to_string())
        }
    }
}