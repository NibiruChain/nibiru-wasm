## Token Vesting

This contract implements vesting accounts for the CW20 and native tokens.

### Master Operations

#### By admin and members

```rust
  RewardUsers {
    rewards: Vec<RewardUserRequest>,
    master_address: Option<String>, // if given, the vesting account can be unregistered
    vesting_schedule: VestingSchedule,
  },
```

This creates a set of vesting accounts for the given users. The `master_address` is used to enable the deregister feature.
If no `master_address` is given, the deregister feature is disabled.

#### By admin only

```rust
  Withdraw {
    amount: Uint128,
    recipient: String,
  },
```

This allows to get part or all of the unallocated amount from the contract and sends it to the `recipient`. Unallocated is equal to the
amount sent on instantiation minus the already rewarded to users.

#### By master_address only

```rust
  DeregisterVestingAccount {
    address: String,
    denom: Denom,
    vested_token_recipient: Option<String>,
    left_vesting_token_recipient: Option<String>,
  },
```

* DeregisterVestingAccount  - deregister vesting account
  * This interface only executable from the `master_address` of a vesting account.
  * It will compute `claimable_amount` and `left_vesting_amount`. Each amount respectively sent to (`vested_token_recipient` or `vesting_account`) 
    and (`left_vesting_token_recipient` or `master_address`).

```rust
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Cw20HookMsg {
    /// Register vesting account with token transfer
    RegisterVestingAccount {
        master_address: Option<String>, // if given, the vesting account can be unregistered
        address: String,
        vesting_schedule: VestingSchedule,
    },
}
```

### Vesting Account Operations

```rust
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    ////////////////////////
    /// VestingAccount Operations ///
    ////////////////////////
    Claim {
        denoms: Vec<Denom>,
        recipient: Option<String>,
    },
}
```

* Sends newly vested token to the (`recipient` or `vesting_account`). The `claim_amount` is computed 
  as (`vested_amount` - `claimed_amount`) and `claimed_amount` is updated to `vested_amount`.

  If everything is claimed, the vesting account is removed from the contract.

### Deployed Contract Info

TODO for mainnet/testnet

| Field         | Value  |
| ------------- | ------ |
| code_id       | ...  |
| contract_addr | ... |
| rpc_url       | ... |
| chain_id      | ... |
