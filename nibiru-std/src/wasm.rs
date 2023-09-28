// crate::wasm.rs

use cosmwasm_std::{to_binary, QueryRequest, StdResult, WasmQuery};

/// Generic helper for constructing WasmQuery::Smart query requests.
pub fn wasm_query_smart<CosmosMsg>(
    contract: impl Into<String>,
    msg: &impl serde::Serialize,
) -> StdResult<QueryRequest<CosmosMsg>> {
    let smart_query_msg = to_binary(msg)?;
    Ok(QueryRequest::Wasm(WasmQuery::Smart {
        contract_addr: contract.into(),
        msg: smart_query_msg,
    }))
}

/// Generic helper for constructing WasmQuery::Raw query requests.
pub fn wasm_query_raw<CosmosMsg>(
    contract: impl Into<String>,
    key: &str,
) -> StdResult<QueryRequest<CosmosMsg>> {
    Ok(QueryRequest::Wasm(WasmQuery::Raw {
        contract_addr: contract.into(),
        key: key.as_bytes().into(),
    }))
}


