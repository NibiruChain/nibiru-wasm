// @generated
// This is duplicated from the base kv directory to avoid a circular dependency
// with the cosmos-sdk

/// Pairs defines a repeated slice of Pair objects.
///
/// Deprecated: Store v1 is deprecated as of v0.50.x, please use Store v2 types
/// instead.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Pairs {
    #[prost(message, repeated, tag="1")]
    pub pairs: ::prost::alloc::vec::Vec<Pair>,
}
/// Pair defines a key/value bytes tuple.
///
/// Deprecated: Store v1 is deprecated as of v0.50.x, please use Store v2 types
/// instead.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Pair {
    #[prost(bytes="bytes", tag="1")]
    pub key: ::prost::bytes::Bytes,
    #[prost(bytes="bytes", tag="2")]
    pub value: ::prost::bytes::Bytes,
}
// @@protoc_insertion_point(module)
