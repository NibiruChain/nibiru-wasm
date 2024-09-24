## contracts/broker-staking

This smart contract handles account abstraction to enable certain staking transaction messages to be called by a subset of "operators", while the funds can only be withdrawn by the contract owner.

This is useful if you want a multisig to manage a large allocation of funds while permitting certain bots to safely make calls to stake or unstake, as is the case for Nibiru's Foundation Delegation Program.

Table of Contents:

- [contracts/broker-staking](#contractsbroker-staking)
- [Overview](#overview)
  - [Master Operations](#master-operations)
    - [Instantiate](#instantiate)
    - [Execute](#execute)
      - [Admin Functions](#admin-functions)
      - [Manager Functions](#manager-functions)
    - [Query](#query)
  - [Deployed Contract Info](#deployed-contract-info)
  - [Testing Against a Live Chain](#testing-against-a-live-chain)

## Overview

The contract has 2 modes, defined by the `autocompounder_on` flag. When it is true, managers can call the contract to stake the balance of the contract.

### Master Operations

#### Instantiate

We need to specify the admin and managers.

```json
{
  "owner": "cosmos1...",
  "to_addrs": ["cosmos1...", "cosmos1..."],
  "opers": ["cosmos1...", "cosmos1..."]
}
```

#### Execute

##### Admin Functions

- **ToggleHalt** allows the admin to halt or resume operator actions, effectively disabling or enabling non-owner permissions.

  ```json
  {
    "toggle_halt": {}
  }
  ```

- **Withdraw** allows the admin to withdraw specific denominations from the contract.

  ```json
  {
    "withdraw": {
      "denoms": ["uatom", "uusd"],
      "to": "cosmos1..."
    }
  }
  ```

- **WithdrawAll** allows the admin to withdraw all funds from the contract.

  ```json
  {
    "withdraw_all": {
      "to": "cosmos1..."
    }
  }
  ```

- **Unstake** allows the admin to unstake the funds from the contract.

  ```json
  {
    "unstake": {
      "unstake_msgs": [
        {
          "validator": "cosmosvaloper1...",
          "amount": "1000000"
        },
        {
          "validator": "cosmosvaloper1...",
          "amount": "1000000"
        }
      ]
    }
  }
  ```

- **UpdateManagers** allows the admin to update the managers of the contract.

  ```json
  {
    "edit_opers": {
      "action": {
        "AddOper": {
          "address": "cosmos1..."
        }
      }
    }
  }
  ```

- **ClaimRewards** allows the admin or managers to claim all staking rewards from the contract’s delegations.

  ```json
  {
    "claim_rewards": {}
  }
  ```

##### Manager Functions

- **Stake** allows managers to stake funds from the contract. The shares are normalized.

  ```json
  {
    "stake": {
      "stake_msgs": [
        {
          "validator": "cosmosvaloper1...",
          "share": "1000000"
        },
        {
          "validator": "cosmosvaloper1...",
          "share": "1000000"
        }
      ],
      "amount": "1000000"
    }
  }
  ```

- **ClaimRewards** allows managers to claim all staking rewards from the contract’s delegations.

  ```json
  {
    "claim_rewards": {}
  }
  ```

#### Query

- **Perms** returns the current permissions status of the contract, including the owner and the operators.

  ```json
  {
    "perms": {}
  }
  ```

- **Ownership** returns the ownership status of the contract.

  ```json
  {
    "ownership": {}
  }
  ```

### Deployed Contract Info

Testnet:

| Field         | Value                                                           |
| ------------- | --------------------------------------------------------------- |
| code_id       | 124                                                             |
| contract_addr | nibi16znhpjugl5cc3dqhu75tytmzqj58herzdg3r4xnkeqpqrwdqeqcq2eshjl |

### Testing Against a Live Chain

---
