#[cfg(test)]
mod tests {
    use std::{collections::HashSet, marker::PhantomData, str::FromStr};
    use nibiru_bindings::query::{
        QueryPerpMsg,AllMarketsResponse, BasePriceResponse, MetricsResponse,
        ModuleAccountsResponse, ModuleParamsResponse,
        PremiumFractionResponse, ReservesResponse,
    };

    use cosmwasm_std::{
        testing::{MockApi, MockQuerier, MockStorage, MOCK_CONTRACT_ADDR},
        to_binary, Addr, Binary, Coin, ContractResult, Decimal, OwnedDeps,
        QuerierWrapper, QueryRequest, SystemResult, Uint256, Uint64,
    };
    

    use crate::query::{
        dummy::{self, dec_420, dec_69},
    };

    pub fn mock_dependencies_with_custom_querier(
        contract_balance: &[Coin],
    ) -> OwnedDeps<MockStorage, MockApi, MockQuerier<QueryPerpMsg>, QueryPerpMsg>
    {
        let mock_querier: MockQuerier<QueryPerpMsg> =
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
        query_msg: &QueryPerpMsg,
    ) -> ContractResult<Binary> {
        match query_msg {
            QueryPerpMsg::AllMarkets {} => {
                to_binary(&dummy::all_markets_response()).into()
            }
            QueryPerpMsg::BasePrice {
                pair: _,
                is_long: _,
                base_amount: _,
            } => to_binary(&dummy::base_price_response()).into(),
            QueryPerpMsg::Position { trader: _, pair: _ } => {
                to_binary(&dummy::position_response()).into()
            }
            QueryPerpMsg::Positions { trader: _ } => {
                to_binary(&dummy::positions_response()).into()
            }
            QueryPerpMsg::Metrics { pair: _ } => {
                to_binary(&dummy::metrics_response()).into()
            }
            QueryPerpMsg::ModuleAccounts {} => {
                to_binary(&dummy::module_accounts_response()).into()
            }
            QueryPerpMsg::ModuleParams {} => {
                to_binary(&dummy::module_params_response()).into()
            }
            QueryPerpMsg::PremiumFraction { pair: _ } => {
                to_binary(&dummy::premium_fraction_response()).into()
            }
            QueryPerpMsg::Reserves { pair: _ } => {
                to_binary(&dummy::reserves_response()).into()
            }
        }
    }

    #[test]
    fn all_markets_query() {
        let deps = mock_dependencies_with_custom_querier(&[]);

        // Call the query
        let req: QueryRequest<QueryPerpMsg> = QueryPerpMsg::AllMarkets {}.into();
        let querier_wrapper = QuerierWrapper::new(&deps.querier);
        let resp: AllMarketsResponse = querier_wrapper.query(&req).unwrap();

        // Check the result
        let market = resp.market_map.get("ETH:USD").unwrap();
        // assert_eq!(resp.pair, "ETH:USD");
        assert_eq!(market.base_reserve, dec_69());
        assert_eq!(market.quote_reserve, dec_69());
        assert_eq!(market.sqrt_depth, dec_69());
        assert_eq!(market.depth, Uint256::from(69u64 * 69u64));
        assert_eq!(market.peg_mult, dec_420());
    }

    #[test]
    fn reserves_query() {
        let deps = mock_dependencies_with_custom_querier(&[]);

        // Call the query
        let req: QueryRequest<QueryPerpMsg> = QueryPerpMsg::Reserves {
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
        let req: QueryRequest<QueryPerpMsg> = QueryPerpMsg::PremiumFraction {
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
        let req: QueryRequest<QueryPerpMsg> = QueryPerpMsg::ModuleParams {}.into();
        let querier_wrapper = QuerierWrapper::new(&deps.querier);
        let resp: ModuleParamsResponse = querier_wrapper.query(&req).unwrap();

        // Check the result
        assert!(!resp.module_params.stopped);
        assert_eq!(resp.module_params.fee_pool_fee_ratio, dec_420());
        assert_eq!(resp.module_params.ecosystem_fund_fee_ratio, dec_69());
        assert_eq!(resp.module_params.liquidation_fee_ratio, dec_69());
        assert_eq!(
            resp.module_params.partial_liquidation_ratio,
            Decimal::zero()
        );
        assert_eq!(resp.module_params.funding_rate_interval, "1h".to_string());
        assert_eq!(
            resp.module_params.twap_lookback_window,
            Uint64::from(60u64 * 60u64)
        );
        assert_eq!(
            resp.module_params.whitelisted_liquidators,
            HashSet::from_iter(
                vec![
                    "nibi1ah8gqrtjllhc5ld4rxgl4uglvwl93ag0sh6e6v",
                    "nibi1zaavvzxez0elundtn32qnk9lkm8kmcsz44g7xl"
                ]
                .iter()
                .map(|s_ptr| s_ptr.to_string())
            ),
        );
    }

    #[test]
    fn module_accounts_query() {
        let deps = mock_dependencies_with_custom_querier(&[]);

        // Call the query
        let req: QueryRequest<QueryPerpMsg> =
            QueryPerpMsg::ModuleAccounts {}.into();
        let querier_wrapper = QuerierWrapper::new(&deps.querier);
        let resp: ModuleAccountsResponse = querier_wrapper.query(&req).unwrap();

        // Check the result
        assert_eq!(resp.module_accounts.len(), 1);
        assert_eq!(resp.module_accounts.get("acc1").unwrap().name, "acc1");
        assert_eq!(
            resp.module_accounts.get("acc1").unwrap().addr,
            Addr::unchecked(String::from(
                "nibi1x5zknk8va44th5vjpg0fagf0lxx0rvurpmp8gs"
            ))
        );
    }

    #[test]
    fn metrics_query() {
        let deps = mock_dependencies_with_custom_querier(&[]);

        // Call the query
        let req: QueryRequest<QueryPerpMsg> = QueryPerpMsg::Metrics {
            pair: "ETH:USD".to_string(),
        }
        .into();
        let querier_wrapper = QuerierWrapper::new(&deps.querier);
        let resp: MetricsResponse = querier_wrapper.query(&req).unwrap();

        // Check the result
        assert_eq!(resp.metrics.pair, "ETH:USD");
        assert_eq!(resp.metrics.net_size, dec_420());
        assert_eq!(resp.metrics.volume_quote, Decimal::one());
        assert_eq!(resp.metrics.volume_base, dec_420());
        assert_eq!(resp.metrics.block_number, Uint64::new(42u64));
    }

    #[test]
    fn base_price_query() {
        let deps = mock_dependencies_with_custom_querier(&[]);

        // Call the query
        let req: QueryRequest<QueryPerpMsg> = QueryPerpMsg::BasePrice {
            pair: String::from("ETH:USD"),
            is_long: true,
            base_amount: Uint256::from_str("123").unwrap(),
        }
        .into();
        let querier_wrapper = QuerierWrapper::new(&deps.querier);
        let resp: BasePriceResponse = querier_wrapper.query(&req).unwrap();

        // Check the result
        assert_eq!(resp.pair, "ETH:USD");
        assert_eq!(resp.base_amount, Decimal::one());
        assert_eq!(resp.quote_amount, dec_420());
        assert!(!resp.is_long);
    }
}
