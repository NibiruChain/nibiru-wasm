// @generated
/// ListenDeliverBlockRequest is the request type for the ListenDeliverBlock RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListenDeliverBlockRequest {
    #[prost(int64, tag="1")]
    pub block_height: i64,
    #[prost(bytes="bytes", repeated, tag="2")]
    pub txs: ::prost::alloc::vec::Vec<::prost::bytes::Bytes>,
    #[prost(message, repeated, tag="3")]
    pub events: ::prost::alloc::vec::Vec<Event>,
    #[prost(message, repeated, tag="4")]
    pub tx_results: ::prost::alloc::vec::Vec<ExecTxResult>,
}
/// ListenDeliverBlockResponse is the response type for the ListenDeliverBlock RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListenDeliverBlockResponse {
}
/// ListenStateChangesRequest is the request type for the ListenStateChanges RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListenStateChangesRequest {
    #[prost(int64, tag="1")]
    pub block_height: i64,
    #[prost(message, repeated, tag="2")]
    pub change_set: ::prost::alloc::vec::Vec<StoreKvPair>,
    #[prost(bytes="bytes", tag="3")]
    pub app_hash: ::prost::bytes::Bytes,
}
/// ListenStateChangesResponse is the response type for the ListenStateChanges RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListenStateChangesResponse {
}
/// StoreKVPair is a single key-value pair, associated with a store.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StoreKvPair {
    /// address defines the address of the account the state changes are coming from.
    /// In case of modules you can expect a stringified
    #[prost(bytes="bytes", tag="1")]
    pub address: ::prost::bytes::Bytes,
    /// key defines the key of the address that changed.
    #[prost(bytes="bytes", tag="2")]
    pub key: ::prost::bytes::Bytes,
    /// value defines the value that changed, empty in case of removal.
    #[prost(bytes="bytes", tag="3")]
    pub value: ::prost::bytes::Bytes,
    /// delete defines if the key was removed.
    ///
    /// true indicates a delete operation, false indicates a set operation
    #[prost(bool, tag="4")]
    pub delete: bool,
}
/// Event is a single event, associated with a transaction.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Event {
    #[prost(string, tag="1")]
    pub r#type: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="2")]
    pub attributes: ::prost::alloc::vec::Vec<EventAttribute>,
}
/// EventAttribute is a single key-value pair, associated with an event.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventAttribute {
    #[prost(string, tag="1")]
    pub key: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub value: ::prost::alloc::string::String,
}
/// ExecTxResult contains results of executing one individual transaction.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecTxResult {
    #[prost(uint32, tag="1")]
    pub code: u32,
    #[prost(bytes="bytes", tag="2")]
    pub data: ::prost::bytes::Bytes,
    #[prost(string, tag="3")]
    pub log: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub info: ::prost::alloc::string::String,
    #[prost(int64, tag="5")]
    pub gas_wanted: i64,
    #[prost(int64, tag="6")]
    pub gas_used: i64,
    #[prost(message, repeated, tag="7")]
    pub events: ::prost::alloc::vec::Vec<Event>,
    #[prost(string, tag="8")]
    pub codespace: ::prost::alloc::string::String,
}
// @@protoc_insertion_point(module)
