## Compounder

This contract handles permissions so we can compound safely the funds of the
multisig.

The contract has 2 modes, defined by the autocompounder_on flag. When it is
true, managers can call the contract to stake the balance of the contract.

Admin can:

- unstake funds from validators
- toggle on/off the autocompounder
- withdraw funds to the multisig

Managers (and admin) can:

- stake funds to validators in the proportion given

This way, only the multisig can maange the funds, and the seed keys of the
managers can be public with no risk to the funds of the treasury.

### Master Operations

#### Instantiate

We need to specify admin and managers

```javascript
{
  "admin": "cosmos1...",
  "managers": ["cosmos1...", "cosmos1..."]
}
```

#### Execute

##### Admin functions

- **SetAutoCompounderMode** sets the auto compounder mode

```javascript
{
  "set_auto_compounder_mode": {
    "mode": "true" // true or false
  }
}
```

- **Withdraw** allows to withdraw the funds from the contract

  ```javascript
  {
    "withdraw": {
      "amount": "1000000"
      "recipient": "cosmos1..."
    }
  }
  ```

- **unstakes** allows to unstake the funds from the contract

  ```javascript
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

- **update managers** allows to update the managers of the contract

```javascript
{
  "update_managers": {
    "managers": ["cosmos1...", "cosmos1..."]
  }
}
```

##### Manager functions

- **stake** allows to stake the funds from the contract. The shares are normalized

```javascript
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
    ]
  },
  "amount": "1000000"
}
```

#### Query

- **auto compounder mode** returns wether the auto compounder mode is enabled or not

```javascript
{
  "auto_compounder_mode": {}
}
```

- **AdminAndManagers** returns the admin and managers of the contract

```javascript
{
  "admin_and_managers": {}
}
```

### Deployed Contract Info

TODO for mainnet/testnet

| Field         | Value |
| ------------- | ----- |
| code_id       | ...   |
| contract_addr | ...   |
| rpc_url       | ...   |
| chain_id      | ...   |

### Testing Against a Live Chain
