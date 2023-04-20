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

#[cfg(test)]
mod tests {

    use std::{marker::PhantomData, collections::HashSet, str::FromStr};

    use cosmwasm_std::{
        testing::{MockApi, MockQuerier, MockStorage, MOCK_CONTRACT_ADDR},
        to_binary, Binary, Coin, ContractResult, Decimal, OwnedDeps,
        QuerierWrapper, QueryRequest, SystemResult, Addr, Uint64, Uint256,
    };
    use cw_utils::Duration;

    use crate::query::{
        dummy::{self, dec_420, dec_69},
        NibiruQuery, PremiumFractionResponse, ReservesResponse, ModuleParamsResponse, ModuleAccountsResponse, MetricsResponse, BasePriceResponse,
    };

    pub fn mock_dependencies_with_custom_querier(
        contract_balance: &[Coin],
    ) -> OwnedDeps<MockStorage, MockApi, MockQuerier<NibiruQuery>, NibiruQuery>
    {
        let mock_querier: MockQuerier<NibiruQuery> =
            MockQuerier::new(&[(MOCK_CONTRACT_ADDR, contract_balance)])
                .with_custom_handler(|query| {
                    SystemResult::Ok(mock_query_execute(query))
                });
        OwnedDeps {
            storage: MockStorage::default(),
            api: MockApi::default(),
            querier: mock_querier,
            custom_query_type: PhantomData,
        }
    }

    pub fn mock_query_execute(
        query_msg: &NibiruQuery,
    ) -> ContractResult<Binary> {
        match query_msg {
            NibiruQuery::AllMarkets {} => {
                to_binary(&dummy::all_markets_response()).into()
            }
            NibiruQuery::BasePrice {
                pair: _,
                is_long: _,
                base_amount: _,
            } => to_binary(&dummy::base_price_response()).into(),
            NibiruQuery::Position { trader: _, pair: _ } => {
                to_binary(&dummy::position_response()).into()
            }
            NibiruQuery::Positions { trader: _ } => {
                to_binary(&dummy::positions_response()).into()
            }
            NibiruQuery::Metrics { pair: _ } => {
                to_binary(&dummy::metrics_response()).into()
            }
            NibiruQuery::ModuleAccounts {} => {
                to_binary(&dummy::module_accounts_response()).into()
            }
            NibiruQuery::ModuleParams {} => {
                to_binary(&dummy::module_params_response()).into()
            }
            NibiruQuery::PremiumFraction { pair: _ } => {
                to_binary(&dummy::premium_fraction_response()).into()
            }
            NibiruQuery::Reserves { pair: _ } => {
                to_binary(&dummy::reserves_response()).into()
            }
        }
    }

    #[test]
    fn reserves_query() {
        let deps = mock_dependencies_with_custom_querier(&[]);

        // Call the query
        let req: QueryRequest<NibiruQuery> = NibiruQuery::Reserves {
            pair: String::from("ETH:USD"),
        }
        .into();
        let querier_wrapper = QuerierWrapper::new(&deps.querier);
        let resp: ReservesResponse = querier_wrapper.query(&req).unwrap();

        // Check the result
        assert_eq!(resp.pair, "ETH:USD");
        assert_eq!(resp.base_reserve, dec_420());
        assert_eq!(resp.quote_reserve, dec_69());
    }

    #[test]
    fn premium_fraction_query() {
        let deps = mock_dependencies_with_custom_querier(&[]);

        // Call the query
        let req: QueryRequest<NibiruQuery> = NibiruQuery::PremiumFraction {
            pair: String::from("ETH:USD"),
        }
        .into();
        let querier_wrapper = QuerierWrapper::new(&deps.querier);
        let resp: PremiumFractionResponse = querier_wrapper.query(&req).unwrap();

        // Check the result
        assert_eq!(resp.pair, "ETH:USD");
        assert_eq!(resp.cpf, Decimal::zero());
        assert_eq!(resp.estimated_next_cpf, dec_69());
    }

    #[test]
    fn module_params_query() {
        let deps = mock_dependencies_with_custom_querier(&[]);

        // Call the query
        let req: QueryRequest<NibiruQuery> = NibiruQuery::ModuleParams {}.into();
        let querier_wrapper = QuerierWrapper::new(&deps.querier);
        let resp: ModuleParamsResponse = querier_wrapper.query(&req).unwrap();

        // Check the result
        assert_eq!(resp.module_params.stopped, false);
        assert_eq!(resp.module_params.fee_pool_fee_ratio, dec_420());
        assert_eq!(resp.module_params.ecosystem_fund_fee_ratio, dec_69());
        assert_eq!(resp.module_params.liquidation_fee_ratio, dec_69());
        assert_eq!(resp.module_params.partial_liquidation_ratio, Decimal::zero());
        assert_eq!(resp.module_params.funding_rate_interval, "1h".to_string());
        assert_eq!(
            resp.module_params.twap_lookback_window,
            Duration::Time(60 * 60)
        );
        assert_eq!(
            resp.module_params.whitelisted_liquidators,
            HashSet::from_iter(
                vec!["nibi123", "nibiabc"].iter().map(|s_ptr| s_ptr.to_string())),
        );
    }

    #[test]
    fn module_accounts_query() {
        let deps = mock_dependencies_with_custom_querier(&[]);

        // Call the query
        let req: QueryRequest<NibiruQuery> =
            NibiruQuery::ModuleAccounts {}.into();
        let querier_wrapper = QuerierWrapper::new(&deps.querier);
        let resp: ModuleAccountsResponse = querier_wrapper.query(&req).unwrap();

        // Check the result
        assert_eq!(resp.module_accounts.len(), 1);
        assert_eq!(
            resp.module_accounts.get("acc1").unwrap().name,
            "acc1"
        );
        assert_eq!(
            resp.module_accounts.get("acc1").unwrap().addr,
            Addr::unchecked(String::from("nibiacc1"))
        );
    }

}
