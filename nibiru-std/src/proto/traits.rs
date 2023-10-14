//! nibiru-std::proto - traits.rs : Implements extensions for prost::Message
//! types for easy conversion to types needed for CosmWasm smart contracts.

use cosmwasm_std::{to_binary, Binary, CosmosMsg, StdResult};

pub trait NibiruProstMsg: prost::Message {
    /// Serialize this protobuf message as a byte vector
    fn to_bytes(&self) -> Vec<u8>;
    fn to_binary(&self) -> StdResult<Binary>;
    /// A type implementing prost::Message is not guaranteed to implement
    /// prost::Name and have a `Name.type_url()` function. This method attempts
    /// to downcast the message to prost::Name, and if successful, constructs the
    /// `CosmosMsg` corresponding to the type.
    fn try_into_stargate_msg(&self, type_url: &str) -> StdResult<CosmosMsg> {
        let value = self.to_binary()?;
        Ok(CosmosMsg::Stargate {
            type_url: type_url.to_string(),
            value,
        })
    }
}

impl<M> NibiruProstMsg for M
where
    M: prost::Message,
{
    fn to_bytes(&self) -> Vec<u8> {
        self.encode_to_vec()
    }

    fn to_binary(&self) -> StdResult<Binary> {
        to_binary(&self.encode_to_vec())
    }
}

pub trait NibiruStargateMsg: prost::Message + prost::Name {
    #![allow(clippy::wrong_self_convention)]
    fn into_stargate_msg(&self) -> StdResult<CosmosMsg>;

    fn type_url(&self) -> String;
}


impl<M> NibiruStargateMsg for M
where
    M: prost::Message + prost::Name,
{
    fn into_stargate_msg(&self) -> StdResult<CosmosMsg> {
        Ok(CosmosMsg::Stargate {
            type_url: self.type_url(),
            value: self.to_binary()?,
        })
    }

    /// Workaround for backwards type_url implementation.
    /// This is fixed in a future version of prost but isn't yet available in
    /// v0.12.1. See https://github.com/tokio-rs/prost/pull/923
    fn type_url(&self) -> String {
        format!("/{}.{}", Self::PACKAGE, Self::NAME)
    }

}