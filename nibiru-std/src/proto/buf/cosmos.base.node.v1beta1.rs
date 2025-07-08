// @generated
/// ConfigRequest defines the request structure for the Config gRPC query.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConfigRequest {
}
/// ConfigResponse defines the response structure for the Config gRPC query.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConfigResponse {
    #[prost(string, tag="1")]
    pub minimum_gas_price: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub pruning_keep_recent: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub pruning_interval: ::prost::alloc::string::String,
    #[prost(uint64, tag="4")]
    pub halt_height: u64,
}
/// StateRequest defines the request structure for the status of a node.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StatusRequest {
}
/// StateResponse defines the response structure for the status of a node.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StatusResponse {
    /// earliest block height available in the store
    #[prost(uint64, tag="1")]
    pub earliest_store_height: u64,
    /// current block height
    #[prost(uint64, tag="2")]
    pub height: u64,
    /// block height timestamp
    #[prost(message, optional, tag="3")]
    pub timestamp: ::core::option::Option<::prost_types::Timestamp>,
    /// app hash of the current block
    #[prost(bytes="bytes", tag="4")]
    pub app_hash: ::prost::bytes::Bytes,
    /// validator hash provided by the consensus header
    #[prost(bytes="bytes", tag="5")]
    pub validator_hash: ::prost::bytes::Bytes,
}
// @@protoc_insertion_point(module)
