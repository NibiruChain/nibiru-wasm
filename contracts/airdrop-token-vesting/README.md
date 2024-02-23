## Token Vesting

This contract implements vesting accounts for the native tokens.

Admin and managers are defined at the instantiation of the contracts. Both can
reward users and de-register vesting accounts, but only the admin can withdraw
the unallocated amount from the contract.

### Master Operations

#### By admin and managers

```rust
  RewardUsers {
    rewards: Vec<RewardUserRequest>,
    vesting_schedule: VestingSchedule,
  },
```

This creates a set of vesting accounts for the given users.

#### By admin only

```rust
  Withdraw {
    amount: Uint128,
    recipient: String,
  },
```

This allows to get part or all of the unallocated amount from the contract and sends it to the `recipient`. Unallocated is equal to the
amount sent on instantiation minus the already rewarded to users.

#### By admin and managers

```rust
  DeregisterVestingAccount {
    address: String,
    vested_token_recipient: Option<String>,
    left_vesting_token_recipient: Option<String>,
  },
```

- DeregisterVestingAccount - deregister vesting account
  - It will compute `claimable_amount` and `left_vesting_amount`. Each amount respectively sent to (`vested_token_recipient` or `vesting_account`)
    and (`left_vesting_token_recipient` or `master_address`).

### Vesting Account Operations

```rust
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    ////////////////////////
    /// VestingAccount Operations ///
    ////////////////////////
    Claim {
        recipient: Option<String>,
    },
}
```

- Sends newly vested token to the (`recipient` or `vesting_account`). The `claim_amount` is computed
  as (`vested_amount` - `claimed_amount`) and `claimed_amount` is updated to `vested_amount`.

  If everything is claimed, the vesting account is removed from the contract.

### Deployed Contract Info

TODO for mainnet/testnet

| Field         | Value |
| ------------- | ----- |
| code_id       | ...   |
| contract_addr | ...   |
| rpc_url       | ...   |
| chain_id      | ...   |
