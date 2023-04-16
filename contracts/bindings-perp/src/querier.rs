use cosmwasm_std::{QuerierWrapper, QueryRequest, StdResult, Uint256};

use crate::query::{
    AllMarketsResponse, BasePriceResponse, MetricsResponse,
    ModuleAccountsResponse, ModuleParamsResponse, NibiruQuery, PositionResponse,
    PositionsResponse, PremiumFractionResponse, ReservesResponse,
};

/// NibiriQuerier makes it easy to export the functions that correspond to each
/// request without needing to know as much about the underlying types.
pub struct NibiruQuerier<'a> {
    querier: &'a QuerierWrapper<'a, NibiruQuery>,
}

impl<'a> NibiruQuerier<'a> {
    pub fn new(querier: &'a QuerierWrapper<NibiruQuery>) -> Self {
        NibiruQuerier { querier }
    }

    pub fn all_markets(&self) -> StdResult<AllMarketsResponse> {
        let query_json = NibiruQuery::AllMarkets {};
        let request: QueryRequest<NibiruQuery> = NibiruQuery::into(query_json);
        self.querier.query(&request)
    }

    pub fn base_price(
        &self,
        pair: String,
        is_long: bool,
        base_amount: Uint256,
    ) -> StdResult<BasePriceResponse> {
        let query_json = NibiruQuery::BasePrice {
            pair,
            is_long,
            base_amount,
        };
        let request: QueryRequest<NibiruQuery> = NibiruQuery::into(query_json);
        self.querier.query(&request)
    }

    pub fn position(
        &self,
        trader: String,
        pair: String,
    ) -> StdResult<PositionResponse> {
        let query_json = NibiruQuery::Position { trader, pair };
        let request: QueryRequest<NibiruQuery> = NibiruQuery::into(query_json);
        self.querier.query(&request)
    }

    pub fn positions(&self, trader: String) -> StdResult<PositionsResponse> {
        let query_json = NibiruQuery::Positions { trader };
        let request: QueryRequest<NibiruQuery> = NibiruQuery::into(query_json);
        self.querier.query(&request)
    }

    pub fn reserves(&self, pair: String) -> StdResult<ReservesResponse> {
        let query_json = NibiruQuery::Reserves { pair };
        let request: QueryRequest<NibiruQuery> = NibiruQuery::into(query_json);
        self.querier.query(&request)
    }

    pub fn premium_fraction(
        &self,
        pair: String,
    ) -> StdResult<PremiumFractionResponse> {
        let query_json = NibiruQuery::PremiumFraction { pair };
        let request: QueryRequest<NibiruQuery> = NibiruQuery::into(query_json);
        self.querier.query(&request)
    }

    pub fn metrics(&self, pair: String) -> StdResult<MetricsResponse> {
        let query_json = NibiruQuery::Metrics { pair };
        let request: QueryRequest<NibiruQuery> = NibiruQuery::into(query_json);
        self.querier.query(&request)
    }

    pub fn module_params(&self) -> StdResult<ModuleParamsResponse> {
        let query_json = NibiruQuery::ModuleParams {};
        let request: QueryRequest<NibiruQuery> = NibiruQuery::into(query_json);
        self.querier.query(&request)
    }

    pub fn module_accounts(&self) -> StdResult<ModuleAccountsResponse> {
        let query_json = NibiruQuery::ModuleAccounts {};
        let request: QueryRequest<NibiruQuery> = NibiruQuery::into(query_json);
        self.querier.query(&request)
    }
}