use cosmwasm_std::{QuerierWrapper, StdResult, Uint256};

use crate::query::{
    QueryPerpMsg, AllMarketsResponse, BasePriceResponse, MetricsResponse,
    ModuleAccountsResponse, ModuleParamsResponse, PositionResponse,
    PositionsResponse, PremiumFractionResponse, ReservesResponse,
};

/// NibiriQuerier makes it easy to export the functions that correspond to each
/// request without needing to know as much about the underlying types.
pub struct NibiruQuerier<'a> {
    querier: &'a QuerierWrapper<'a, QueryPerpMsg>,
}

impl<'a> NibiruQuerier<'a> {
    pub fn new(querier: &'a QuerierWrapper<QueryPerpMsg>) -> Self {
        NibiruQuerier { querier }
    }

    pub fn all_markets(&self) -> StdResult<AllMarketsResponse> {
        let request = QueryPerpMsg::AllMarkets {};
        self.querier.query(&request.into())
    }

    pub fn base_price(
        &self,
        pair: String,
        is_long: bool,
        base_amount: Uint256,
    ) -> StdResult<BasePriceResponse> {
        let request = QueryPerpMsg::BasePrice {
            pair,
            is_long,
            base_amount,
        };

        self.querier.query(&request.into())
    }

    pub fn position(
        &self,
        trader: String,
        pair: String,
    ) -> StdResult<PositionResponse> {
        let request = QueryPerpMsg::Position { trader, pair };

        self.querier.query(&request.into())
    }

    pub fn positions(&self, trader: String) -> StdResult<PositionsResponse> {
        let request = QueryPerpMsg::Positions { trader };

        self.querier.query(&request.into())
    }

    pub fn reserves(&self, pair: String) -> StdResult<ReservesResponse> {
        let request = QueryPerpMsg::Reserves { pair };

        self.querier.query(&request.into())
    }

    pub fn premium_fraction(
        &self,
        pair: String,
    ) -> StdResult<PremiumFractionResponse> {
        let request = QueryPerpMsg::PremiumFraction { pair };

        self.querier.query(&request.into())
    }

    pub fn metrics(&self, pair: String) -> StdResult<MetricsResponse> {
        let request = QueryPerpMsg::Metrics { pair };

        self.querier.query(&request.into())
    }

    pub fn module_params(&self) -> StdResult<ModuleParamsResponse> {
        let request = QueryPerpMsg::ModuleParams {};

        self.querier.query(&request.into())
    }

    pub fn module_accounts(&self) -> StdResult<ModuleAccountsResponse> {
        let request = QueryPerpMsg::ModuleAccounts {};

        self.querier.query(&request.into())
    }
}

