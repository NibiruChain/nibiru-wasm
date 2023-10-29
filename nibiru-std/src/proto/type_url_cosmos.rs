//! Implements the prost::Name trait for cosmos protobuf types, which defines
//! the prost::Message.type_url function needed for CosmWasm smart contracts.

use prost::Name;

use crate::proto::cosmos;

const PACKAGE_BANK: &str = "cosmos.bank.v1beta1";

impl Name for cosmos::bank::v1beta1::QuerySupplyOfRequest {
    const NAME: &'static str = "QuerySupplyOfRequest";
    const PACKAGE: &'static str = PACKAGE_BANK;
}

impl Name for cosmos::bank::v1beta1::QueryBalanceRequest {
    const NAME: &'static str = "QueryBalanceRequest";
    const PACKAGE: &'static str = PACKAGE_BANK;
}

impl Name for cosmos::bank::v1beta1::QueryAllBalancesRequest {
    const NAME: &'static str = "QueryAllBalancesRequest";
    const PACKAGE: &'static str = PACKAGE_BANK;
}

impl Name for cosmos::bank::v1beta1::QueryDenomMetadataRequest {
    const NAME: &'static str = "QueryDenomMetadataRequest";
    const PACKAGE: &'static str = PACKAGE_BANK;
}

impl Name for cosmos::bank::v1beta1::MsgSend {
    const NAME: &'static str = "MsgSend";
    const PACKAGE: &'static str = PACKAGE_BANK;
}

impl Name for cosmos::bank::v1beta1::MsgMultiSend {
    const NAME: &'static str = "MsgMultiSend";
    const PACKAGE: &'static str = PACKAGE_BANK;
}

impl Name for cosmos::bank::v1beta1::MsgUpdateParams {
    const NAME: &'static str = "MsgUpdateParams";
    const PACKAGE: &'static str = PACKAGE_BANK;
}

impl Name for cosmos::bank::v1beta1::MsgSetSendEnabled {
    const NAME: &'static str = "MsgSetSendEnabled";
    const PACKAGE: &'static str = PACKAGE_BANK;
}

#[cfg(test)]
mod tests {

    use cosmwasm_std::{Empty, QueryRequest};

    use crate::{
        errors::{NibiruResult, TestResult},
        proto::{cosmos, NibiruStargateQuery},
    };

    #[test]
    fn stargate_query_conversion() -> TestResult {
        let test_cases: Vec<(&str, NibiruResult<QueryRequest<Empty>>)> = vec![
            (
                "/cosmos.bank.v1beta1.Query/SupplyOf",
                cosmos::bank::v1beta1::QuerySupplyOfRequest {
                    denom: String::from("some_denom"),
                }
                .into_stargate_query(),
            ),
            (
                "/cosmos.bank.v1beta1.Query/Balance",
                cosmos::bank::v1beta1::QueryBalanceRequest {
                    address: String::from("some_address"),
                    denom: String::from("some_denom"),
                }
                .into_stargate_query(),
            ),
            (
                "/cosmos.bank.v1beta1.Query/DenomMetadata",
                cosmos::bank::v1beta1::QueryDenomMetadataRequest {
                    denom: String::from("some_denom"),
                }
                .into_stargate_query(),
            ),
        ];

        for test_case in test_cases {
            let test_case_path = test_case.0;
            let pb_query = test_case.1?;
            if let QueryRequest::Stargate { path, data: _data } = &pb_query {
                assert_eq!(test_case_path, path)
            } else {
                panic!("failed test on case: {:#?} ", test_case_path)
            }
        }

        Ok(())
    }
}
