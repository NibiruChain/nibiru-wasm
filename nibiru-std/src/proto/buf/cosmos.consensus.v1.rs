// @generated
/// ConsensusMsgParams is the Msg/Params request type. This is a consensus message that is sent from cometbft.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConsensusMsgParams {
    /// params defines the x/consensus parameters to be passed from comet.
    ///
    /// NOTE: All parameters must be supplied.
    #[prost(message, optional, tag="1")]
    pub version: ::core::option::Option<super::super::super::cometbft::types::v1::VersionParams>,
    #[prost(message, optional, tag="2")]
    pub block: ::core::option::Option<super::super::super::cometbft::types::v1::BlockParams>,
    #[prost(message, optional, tag="3")]
    pub evidence: ::core::option::Option<super::super::super::cometbft::types::v1::EvidenceParams>,
    #[prost(message, optional, tag="4")]
    pub validator: ::core::option::Option<super::super::super::cometbft::types::v1::ValidatorParams>,
    #[deprecated]
    #[prost(message, optional, tag="5")]
    pub abci: ::core::option::Option<super::super::super::cometbft::types::v1::AbciParams>,
    #[prost(message, optional, tag="6")]
    pub synchrony: ::core::option::Option<super::super::super::cometbft::types::v1::SynchronyParams>,
    #[prost(message, optional, tag="7")]
    pub feature: ::core::option::Option<super::super::super::cometbft::types::v1::FeatureParams>,
}
/// ConsensusMsgParamsResponse defines the response structure for executing a
/// ConsensusMsgParams message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConsensusMsgParamsResponse {
}
// @@protoc_insertion_point(module)
