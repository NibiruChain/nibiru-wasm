# cw-nibiru/contracts/nibi-stargate

This smart contract showcases usage examples for certain Nibiru-specific and
Cosmos-SDK-specific: 

1.  **Stargate messages**: Instances of the [`CosmosMsg::Stargate` variant in
    cosmwasm-std](https://docs.rs/cosmwasm-std/1.4.1/cosmwasm_std/enum.CosmosMsg.html). These correspond to
    [StargateMsg in CosmWasm/wasmvm](https://pkg.go.dev/github.com/CosmWasm/wasmvm@v1.4.1/types#StargateMsg) 
2.  **Stargate queries**: Instances of the [`QueryRequest::Stargate` variant in
    cosmwasm-std](https://docs.rs/cosmwasm-std/1.4.1/cosmwasm_std/enum.QueryRequest.html). These correspond to
    [StargateQuery from CosmWasm/wasmvm](https://pkg.go.dev/github.com/CosmWasm/wasmvm@v1.4.1/types#StargateMsg)

Table of Contents

- [Examples](#examples)
- [Stargate Types](#stargate-types)
  - [cosmwasm-std: CosmosMsg::Stargate](#cosmwasm-std-cosmosmsgstargate)
  - [wasmvm: StargateMsg](#wasmvm-stargatemsg)
  - [cosmwasm-std: QueryRequest::Stargate](#cosmwasm-std-queryrequeststargate)
  - [wasmvm: StargateQuery](#wasmvm-stargatequery)

## Examples

`ExecuteMsg::CreateDenom`

```json
{
  "create_denom": { "subdenom": "zzz" }
}
```

`ExecuteMsg::Mint`

```json
{ 
  "mint": { 
    "coin": { "amount": "[amount]", "denom": "tf/[contract-addr]/[subdenom]" }, 
    "mint_to": "[mint-to-addr]" 
  } 
}
```

`ExecuteMsg::Burn`

```json
{ 
  "burn": { 
    "coin": { "amount": "[amount]", "denom": "tf/[contract-addr]/[subdenom]" }, 
    "burn_from": "[burn-from-addr]" 
  } 
}
```

`ExecuteMsg::ChangeAdmin`

```json
{ 
  "change_admin": { 
    "denom": "tf/[contract-addr]/[subdenom]", 
    "new_admin": "[ADDR]" 
  } 
}
```

## Stargate Types

### cosmwasm-std: CosmosMsg::Stargate

```rust
pub enum CosmosMsg<T = Empty> {
    /// ... other variants like Bank, Custom, Staking, Ibc, Wasm
    Stargate {
        type_url: String,
        value: Binary,
    },
}
```

### wasmvm: StargateMsg

```go
type StargateMsg struct {
	TypeURL string `json:"type_url"`
	Value   []byte `json:"value"`
}
```

### cosmwasm-std: QueryRequest::Stargate

```rust
pub enum QueryRequest<C> {
    /// ... other variants like Bank, Custom, Staking, Ibc, Wasm
    Stargate {
        /// this is the fully qualified service path used for routing,
        /// eg. custom/cosmos_sdk.x.bank.v1.Query/QueryBalance
        path: String,
        /// this is the expected protobuf message type (not any), binary encoded
        data: Binary,
    },
}
```

### wasmvm: StargateQuery

```go
// StargateQuery is encoded the same way as abci_query, with path and protobuf
// encoded request data. The format is defined in
// [ADR-21](https://github.com/cosmos/cosmos-sdk/blob/master/docs/architecture/adr-021-protobuf-query-encoding.md).
// The response is protobuf encoded data directly without a JSON response
// wrapper. The caller is responsible for compiling the proper protobuf
// definitions for both requests and responses.
type StargateQuery struct {
	// this is the fully qualified service path used for routing,
	// eg. custom/cosmos_sdk.x.bank.v1.Query/QueryBalance
	Path string `json:"path"`
	// this is the expected protobuf message type (not any), binary encoded
	Data []byte `json:"data"`
}
```