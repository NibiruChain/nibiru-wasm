//! nibiru-std::proto - traits.rs : Implements extensions for prost::Message
//! types for easy conversion to types needed for CosmWasm smart contracts.

use cosmwasm_std::{Binary, CosmosMsg};

pub trait NibiruProstMsg: prost::Message {
    /// Serialize this protobuf message as a byte vector
    fn to_bytes(&self) -> Vec<u8>;
    fn to_binary(&self) -> Binary;
    /// A type implementing prost::Message is not guaranteed to implement
    /// prost::Name and have a `Name.type_url()` function. This method attempts
    /// to downcast the message to prost::Name, and if successful, constructs a
    /// `CosmosMsg::Stargate` object corresponding to the type.
    fn try_into_stargate_msg(&self, type_url: &str) -> CosmosMsg {
        let value = self.to_binary();
        CosmosMsg::Stargate {
            type_url: type_url.to_string(),
            value,
        }
    }

    /// Parse into this protobuf type from `prost_types::Any`.
    fn from_any(any: &prost_types::Any) -> Result<Self, prost::DecodeError>
    where
        Self: Default + prost::Name + Sized,
    {
        any.to_msg()
    }
}

impl<M> NibiruProstMsg for M
where
    M: prost::Message,
{
    fn to_bytes(&self) -> Vec<u8> {
        self.encode_to_vec()
    }

    fn to_binary(&self) -> Binary {
        Binary::from(self.encode_to_vec())
    }
}

pub trait NibiruStargateMsg: prost::Message + prost::Name {
    #![allow(clippy::wrong_self_convention)]
    fn into_stargate_msg(&self) -> CosmosMsg;

    fn type_url(&self) -> String;
}

impl<M> NibiruStargateMsg for M
where
    M: prost::Message + prost::Name,
{
    /// Returns the `prost::Message` as a `CosmosMsg::Stargate` object.
    fn into_stargate_msg(&self) -> CosmosMsg {
        CosmosMsg::Stargate {
            type_url: self.type_url(),
            value: self.to_binary(),
        }
    }

    /// Workaround for backwards type_url implementation.
    /// This is fixed in a future version of prost but isn't yet available in
    /// v0.12.1. See https://github.com/tokio-rs/prost/pull/923
    fn type_url(&self) -> String {
        format!("/{}.{}", Self::PACKAGE, Self::NAME)
    }
}
