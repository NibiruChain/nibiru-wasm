use cosmwasm_std::{QuerierWrapper, QueryRequest, StdResult};

use crate::bindings::{
    NibiruQuery, PositionResponse, PositionsResponse
};

/// This is a helper wrapper to easily use our custom queries
pub struct NibiruQuerier<'a> {
    querier: &'a QuerierWrapper<'a, NibiruQuery>,
}

impl<'a> NibiruQuerier<'a> {
    pub fn new(querier: &'a QuerierWrapper<NibiruQuery>) -> Self {
        NibiruQuerier { querier }
    }

    pub fn position(
        &self,
        trader: String,
        pair: String,
    ) -> StdResult<PositionResponse> {
        let position_query = NibiruQuery::Position {
            trader,
            pair,
        };
        let request: QueryRequest<NibiruQuery> = NibiruQuery::into(position_query);
        self.querier.query(&request)
    }

    pub fn positions(
        &self,
        trader: String,
    ) -> StdResult<PositionsResponse> {
        let positions_query = NibiruQuery::Positions {
            trader,
        };
        let request: QueryRequest<NibiruQuery> = NibiruQuery::into(positions_query);
        self.querier.query(&request)
    }
}
