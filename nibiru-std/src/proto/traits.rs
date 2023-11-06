//! nibiru-std::proto - traits.rs : Implements extensions for prost::Message
//! types for easy conversion to types needed for CosmWasm smart contracts.

use cosmwasm_std::{Binary, CosmosMsg, QueryRequest};

use crate::errors::{NibiruError, NibiruResult};

use crate::proto::cosmos;

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

    /// The "type URL" in the context of protobuf is used with a feature
    /// called "Any", a type that allows one to serialize and embed proto
    /// message (prost::Message) objects without as opaque values without having
    /// to predefine the type in the original message declaration.
    ///
    /// For example, a protobuf definition like:
    /// ```proto
    /// message CustomProtoMsg { string name = 1; }
    /// ```
    /// might have a type URL like "googleapis.com/package.name.CustomProtoMsg".
    /// Usage of `Any` with type URLs enables dynamic message composition and
    /// flexibility.
    ///
    /// We use these type URLs in CosmWasm and the Cosmos-SDK to classify
    /// gRPC messages for transactions and queries because Tendermint ABCI
    /// messages are protobuf objects.
    fn type_url(&self) -> String {
        format!("/{}.{}", Self::PACKAGE, Self::NAME)
    }
}

pub trait NibiruStargateQuery: prost::Message + prost::Name {
    #![allow(clippy::wrong_self_convention)]
    fn into_stargate_query(
        &self,
    ) -> NibiruResult<QueryRequest<cosmwasm_std::Empty>>;

    fn path(&self) -> String;
}

impl<M> NibiruStargateQuery for M
where
    M: prost::Message + prost::Name,
{
    /// Returns the `prost::Message` as a `QueryRequest::Stargate` object.
    /// Errors if the `prost::Name::type_url` does not indicate the type is a
    /// query.
    fn into_stargate_query(
        &self,
    ) -> NibiruResult<QueryRequest<cosmwasm_std::Empty>> {
        if !self.type_url().contains("Query") {
            return Err(NibiruError::ProstNameisNotQuery {
                type_url: self.type_url(),
            });
        }
        Ok(QueryRequest::Stargate {
            path: self.path(),
            data: self.to_binary(),
        })
    }

    /// Fully qualified gRPC service path used for routing.
    /// Ex.: "/cosmos.bank.v1beta1.Query/SupplyOf"
    fn path(&self) -> String {
        let service_name = format!(
            "Query/{}",
            Self::NAME
                .trim_start_matches("Query")
                .trim_end_matches("Request")
        );
        format!("/{}.{}", Self::PACKAGE, service_name)
    }
}

impl From<cosmwasm_std::Coin> for cosmos::base::v1beta1::Coin {
    fn from(cw_coin: cosmwasm_std::Coin) -> Self {
        cosmos::base::v1beta1::Coin {
            denom: cw_coin.denom,
            amount: cw_coin.amount.to_string(),
        }
    }
}
