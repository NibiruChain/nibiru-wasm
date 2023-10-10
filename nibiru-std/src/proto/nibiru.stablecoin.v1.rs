// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventTransfer {
    #[prost(message, optional, tag = "1")]
    pub coin: ::core::option::Option<crate::proto::cosmos::base::v1beta1::Coin>,
    #[prost(string, tag = "2")]
    pub from: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub to: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventMintStable {
    #[prost(string, tag = "1")]
    pub amount: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventBurnStable {
    #[prost(string, tag = "1")]
    pub amount: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventMintNibi {
    #[prost(string, tag = "1")]
    pub amount: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventBurnNibi {
    #[prost(string, tag = "1")]
    pub amount: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventRecollateralize {
    #[prost(string, tag = "1")]
    pub caller: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub in_coin:
        ::core::option::Option<crate::proto::cosmos::base::v1beta1::Coin>,
    #[prost(message, optional, tag = "3")]
    pub out_coin:
        ::core::option::Option<crate::proto::cosmos::base::v1beta1::Coin>,
    #[prost(string, tag = "4")]
    pub coll_ratio: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventBuyback {
    #[prost(string, tag = "1")]
    pub caller: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub in_coin:
        ::core::option::Option<crate::proto::cosmos::base::v1beta1::Coin>,
    #[prost(message, optional, tag = "3")]
    pub out_coin:
        ::core::option::Option<crate::proto::cosmos::base::v1beta1::Coin>,
    #[prost(string, tag = "4")]
    pub coll_ratio: ::prost::alloc::string::String,
}
/// Params defines the parameters for the module.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    /// collRatio is the ratio needed as collateral to exchange for stables
    #[prost(int64, tag = "1")]
    pub coll_ratio: i64,
    /// feeRatio is the ratio taken as fees when minting or burning stables
    #[prost(int64, tag = "2")]
    pub fee_ratio: i64,
    /// efFeeRatio is the ratio taken from the fees that goes to Ecosystem Fund
    #[prost(int64, tag = "3")]
    pub ef_fee_ratio: i64,
    /// BonusRateRecoll is the percentage of extra stablecoin value given to the
    /// caller of 'Recollateralize' in units of governance tokens.
    #[prost(int64, tag = "4")]
    pub bonus_rate_recoll: i64,
    /// distr_epoch_identifier defines the frequnecy of update for the collateral
    /// ratio
    #[prost(string, tag = "5")]
    pub distr_epoch_identifier: ::prost::alloc::string::String,
    /// adjustmentStep is the size of the step taken when updating the collateral
    /// ratio
    #[prost(int64, tag = "6")]
    pub adjustment_step: i64,
    /// priceLowerBound is the lower bound for the stable coin to trigger a
    /// collateral ratio update
    #[prost(int64, tag = "7")]
    pub price_lower_bound: i64,
    /// priceUpperBound is the upper bound for the stable coin to trigger a
    /// collateral ratio update
    #[prost(int64, tag = "8")]
    pub price_upper_bound: i64,
    /// isCollateralRatioValid checks if the collateral ratio is correctly updated
    #[prost(bool, tag = "9")]
    pub is_collateral_ratio_valid: bool,
}
/// GenesisState defines the stablecoin module's genesis state.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
    #[prost(message, optional, tag = "2")]
    pub module_account_balance:
        ::core::option::Option<crate::proto::cosmos::base::v1beta1::Coin>,
}
// ---------------------------------------- Params

/// QueryParamsRequest is request type for the Query/Params RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsRequest {}
/// QueryParamsResponse is response type for the Query/Params RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsResponse {
    /// params holds all the parameters of this module.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
// ---------------------------------------- ModuleAccountBalances

/// QueryModuleAccountBalances is the request type for the balance of the
/// x/stablecoin module account.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryModuleAccountBalances {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryModuleAccountBalancesResponse {
    /// ModuleAccountBalances is the balance of all coins in the x/stablecoin
    /// module.
    #[prost(message, repeated, tag = "1")]
    pub module_account_balances:
        ::prost::alloc::vec::Vec<crate::proto::cosmos::base::v1beta1::Coin>,
}
// ---------------------------------------- CirculatingSupplies

/// QueryCirculatingSupplies is the request type for the circulating supply of
/// both NIBI and NUSD.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryCirculatingSupplies {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryCirculatingSuppliesResponse {
    #[prost(message, optional, tag = "1")]
    pub nibi: ::core::option::Option<crate::proto::cosmos::base::v1beta1::Coin>,
    #[prost(message, optional, tag = "2")]
    pub nusd: ::core::option::Option<crate::proto::cosmos::base::v1beta1::Coin>,
}
// ---------------------------------------- GovToMintStable

/// QueryGovToMintStable is the request type for the Query/GovToMintStable RPC
/// method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGovToMintStable {
    #[prost(message, optional, tag = "1")]
    pub collateral:
        ::core::option::Option<crate::proto::cosmos::base::v1beta1::Coin>,
}
/// QueryGovToMintStableResponse is the response type for 'QueryGovToMintStable'
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGovToMintStableResponse {
    #[prost(message, optional, tag = "1")]
    pub gov: ::core::option::Option<crate::proto::cosmos::base::v1beta1::Coin>,
}
// ---------------------------------------- Liquidity Ratio Info

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LiquidityRatioInfo {
    #[prost(string, tag = "1")]
    pub liquidity_ratio: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub upper_band: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub lower_band: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryLiquidityRatioInfoRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryLiquidityRatioInfoResponse {
    #[prost(message, optional, tag = "1")]
    pub info: ::core::option::Option<LiquidityRatioInfo>,
}
///
/// MsgMintStable: Msg to mint NUSD. A user deposits NIBI and collateral and gets
/// NUSD in return. The amount of NUSD received depends on the current price set
/// by the oracle library and the current collateral ratio for the protocol.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgMintStable {
    #[prost(string, tag = "1")]
    pub creator: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub stable:
        ::core::option::Option<crate::proto::cosmos::base::v1beta1::Coin>,
}
/// MsgMintStableResponse specifies the amount of NUSD token the user will
/// receive after their mint transaction
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgMintStableResponse {
    #[prost(message, optional, tag = "1")]
    pub stable:
        ::core::option::Option<crate::proto::cosmos::base::v1beta1::Coin>,
    #[prost(message, repeated, tag = "2")]
    pub used_coins:
        ::prost::alloc::vec::Vec<crate::proto::cosmos::base::v1beta1::Coin>,
    #[prost(message, repeated, tag = "3")]
    pub fees_payed:
        ::prost::alloc::vec::Vec<crate::proto::cosmos::base::v1beta1::Coin>,
}
///
/// MsgBurnStable allows users to burn NUSD in exchange for NIBI and collateral.
/// The amount of NIBI and Collateral received depends on the current price set by
/// the x/oracle library and the current collateral ratio.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgBurnStable {
    #[prost(string, tag = "1")]
    pub creator: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub stable:
        ::core::option::Option<crate::proto::cosmos::base::v1beta1::Coin>,
}
/// MsgBurnStableResponse specifies the amount of collateral and governance
/// token the user will receive after their burn transaction.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgBurnStableResponse {
    #[prost(message, optional, tag = "1")]
    pub collateral:
        ::core::option::Option<crate::proto::cosmos::base::v1beta1::Coin>,
    #[prost(message, optional, tag = "2")]
    pub gov: ::core::option::Option<crate::proto::cosmos::base::v1beta1::Coin>,
    #[prost(message, repeated, tag = "3")]
    pub fees_payed:
        ::prost::alloc::vec::Vec<crate::proto::cosmos::base::v1beta1::Coin>,
}
/// MsgRecollateralize
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRecollateralize {
    #[prost(string, tag = "1")]
    pub creator: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub coll: ::core::option::Option<crate::proto::cosmos::base::v1beta1::Coin>,
}
/// MsgRecollateralizeResponse is the output of a successful 'Recollateralize'
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRecollateralizeResponse {
    /// Gov (sdk.Coin): Tokens rewarded to the caller in exchange for her
    /// collateral.
    #[prost(message, optional, tag = "1")]
    pub gov: ::core::option::Option<crate::proto::cosmos::base::v1beta1::Coin>,
}
/// MsgBuyback
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgBuyback {
    #[prost(string, tag = "1")]
    pub creator: ::prost::alloc::string::String,
    /// Gov (sdk.Coin): Tokens the caller wants to sell to the protocol in exchange
    /// for collateral.
    #[prost(message, optional, tag = "2")]
    pub gov: ::core::option::Option<crate::proto::cosmos::base::v1beta1::Coin>,
}
/// MsgBuybackResponse is the output of a successful 'Buyback'
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgBuybackResponse {
    /// Coll (sdk.Coin): Tokens sold to the caller in exchange for her collateral.
    #[prost(message, optional, tag = "1")]
    pub coll: ::core::option::Option<crate::proto::cosmos::base::v1beta1::Coin>,
}
// @@protoc_insertion_point(module)