// @generated
/// ListenEndBlockRequest is the request type for the ListenEndBlock RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListenFinalizeBlockRequest {
    #[prost(message, optional, tag="1")]
    pub req: ::core::option::Option<super::super::super::super::cometbft::abci::v1::FinalizeBlockRequest>,
    #[prost(message, optional, tag="2")]
    pub res: ::core::option::Option<super::super::super::super::cometbft::abci::v1::FinalizeBlockResponse>,
}
/// ListenEndBlockResponse is the response type for the ListenEndBlock RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListenFinalizeBlockResponse {
}
/// ListenCommitRequest is the request type for the ListenCommit RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListenCommitRequest {
    /// explicitly pass in block height as ResponseCommit does not contain this
    /// info
    #[prost(int64, tag="1")]
    pub block_height: i64,
    #[prost(message, optional, tag="2")]
    pub res: ::core::option::Option<super::super::super::super::cometbft::abci::v1::CommitResponse>,
    #[prost(message, repeated, tag="3")]
    pub change_set: ::prost::alloc::vec::Vec<super::super::v1beta1::StoreKvPair>,
}
/// ListenCommitResponse is the response type for the ListenCommit RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListenCommitResponse {
}
// @@protoc_insertion_point(module)
