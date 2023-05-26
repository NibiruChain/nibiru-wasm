#[cfg(test)]
pub mod integration_tests {
    use crate::msg::InitMsg;
    use cosmwasm_std::{coins, Decimal, Response};
    use cosmwasm_vm::testing::{
        instantiate, mock_env, mock_info, mock_instance,
    };
    use std::str::FromStr;

    // TODO test that the file exists
    static WASM: &[u8] = include_bytes!("../../../artifacts/bindings_perp.wasm");

    #[test]
    fn msg_init() {
        let mut deps = mock_instance(WASM, &[]);
        let sender = String::from("sender");
        let info = mock_info(&sender, &coins(1000, "unibi"));
        let inst_msg = InitMsg {};
        let result: Response =
            instantiate(&mut deps, mock_env(), info, inst_msg).unwrap();
        assert_eq!(result.messages.len(), 0);
    }

    #[test]
    fn negative_decimal_not_possible() {
        let neg = Decimal::from_str("-420");
        assert!(neg.is_err())
    }

    // Example integration test for a custom query
    // TODO This requires writing a test querier that registers the custom enum
    //
    // const DESERIALIZATION_LIMIT: usize = 20_000;
    //
    // #[test]
    // fn query_reserves() {
    //     let mut deps = mock_instance(WASM, &[]);
    //     let sender = String::from("sender");
    //     let info = mock_info(&sender, &coins(1000, "unibi"));
    //     let inst_msg = InitMsg {};
    //     let result: Response =
    //         instantiate(&mut deps, mock_env(), info, inst_msg).unwrap();
    //     assert_eq!(result.messages.len(), 0);

    //     let pair = String::from("ueth:unusd");
    //     let query_msg = NibiruQuery::Reserves { pair };
    //     let raw_resp = query(&mut deps, mock_env(), query_msg);
    //     assert!(raw_resp.is_err(), "err: {}", raw_resp.unwrap_err());
    //     let resp: ReservesResponse =
    //         from_slice(&raw_resp.unwrap(), DESERIALIZATION_LIMIT).unwrap();
    //     assert_eq!(resp.pair, pair)
    // }
}