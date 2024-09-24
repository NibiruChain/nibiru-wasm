use thiserror::Error;

use cosmwasm_std as cw;

/// Shorthand for an empty anyhow::Result. Useful for idiomatic tests.
pub type TestResult = anyhow::Result<()>;

pub type NibiruResult<T> = Result<T, NibiruError>;

#[derive(Error, Debug, PartialEq)]
pub enum NibiruError {
    #[error("{0}")]
    CwStd(#[from] cw::StdError),

    #[error("no prost::Name implementation for type {}, where prost::Name.type_url() is needed.", type_name)]
    NoTypeUrl { type_name: String },

    #[error("prost::Name::type_url {} does not correspond to a QueryRequest::Stargate path.", type_url)]
    ProstNameisNotQuery { type_url: String },

    #[error("prost::Name::type_url {} does not correspond to a CosmosMsg::Stargate type_url.", type_url)]
    ProstNameisNotMsg { type_url: String },

    #[error("{0}")]
    MathError(#[from] MathError),
}

#[derive(Error, Debug, PartialEq)]
pub enum MathError {
    #[error("division by zero not well defined")]
    DivisionByZero,

    #[error("could not parse decimal from string \"{}\": {}", dec_str, err)]
    CwDecParseError { dec_str: String, err: cw::StdError },

    #[error("could not parse to cosmosdk.io/math.LegacyDec: {0}")]
    SdkDecError(String),
}

impl From<NibiruError> for cw::StdError {
    fn from(err: NibiruError) -> cw::StdError {
        match err {
            NibiruError::CwStd(e) => e,
            e => cw::StdError::generic_err(e.to_string()),
        }
    }
}
