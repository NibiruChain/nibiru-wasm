# Contracts Cookbook

This file describes the different messages that can be sent as queries or transactions
to the contracts of this repository with a description of the expected behavior.

- [Contracts Cookbook](#contracts-cookbook)
  - [Core shifter](#core-shifter)
    - [Instantiate](#instantiate)
    - [Execute](#execute)
    - [Query](#query)
  - [Core token vesting](#core-token-vesting)
    - [Instantiate](#instantiate-1)
    - [Execute](#execute-1)
    - [Query](#query-1)
  - [4. Nibi Stargate](#4-nibi-stargate)
    - [4.1 Instantiate](#41-instantiate)
    - [4.2 Execute](#42-execute)
  - [5. Nibi Stargate Perp](#5-nibi-stargate-perp)
    - [5.1 Instantiate](#51-instantiate)
    - [5.2 Execute](#52-execute)
  - [6. Nusd Valuator](#6-nusd-valuator)
    - [6.1 Instantiate](#61-instantiate)
    - [6.2 Execute](#62-execute)
    - [6.3 Query](#63-query)
  - [7. Airdrop token vesting](#7-airdrop-token-vesting)
    - [7.1 Instantiate](#71-instantiate)
    - [7.2 Execute](#72-execute)
    - [7.3 Query](#73-query)
  - [8. Auto compounder](#8-auto-compounder)
    - [8.1 Instantiate](#81-instantiate)
    - [8.2 Execute](#82-execute)
      - [Admin functions](#admin-functions)
      - [Manager functions](#manager-functions)
    - [8.3 Query](#83-query)

## Core shifter

Shifter is a simple contract that can execute peg and depth shift to any markets in the x/perp module of Nibiru.
The contract holds a whitelist of addressses that are allowed to execute the shift.

### Instantiate

The instantiation defines just the onwer of the contract, who wil be able to add and remove addresses from the whitelist, and execute the shifts.

```js
{"owner": "cosmos1..."}
```

### Execute

- **ShiftSwapInvariant** executes a depth shift in a market.

```js
{
  "shift_swap_invariant": {
    "pair": "uusd:usdr",
    "new_swap_invariant": "1000000"
  }
}
```

- **ShiftPegMultiplier** executes a depth shift on a market. It can be executed by anyone.

```js
{
  "shift_peg_multiplier": {
    "pair": "ubtc:unusd",
    "new_peg_mult": "20420.69"
  }
}
```

- **EditOpers** adds or removes addresses from the whitelist. It can be executed by the owner.

```js
{
  "edit_opers": {
    "add_oper": {"addr": "cosmos1..."},
    "remove_oper": {"addr": "cosmos1..."},
  }
}
```

### Query

The queries have to do with checking permissions of addresses.

- **HasPerms** checks if an address has permissions to execute shifts.

```js
{
  "has_perms": {
    "address": "cosmos1..."
  }
}
```

- **Perms** query the contract owner and set of operators.

```js
{
  "perms": {},
}
```

## Core token vesting

This contract implements vesting accounts for the CW20 and native tokens.

### Instantiate

There's no instantiation message.

```js
{
}
```

### Execute

- **Receive**

```js
{
  "receive": {
    "sender": "cosmos1...",
    "amount": "1000000",
    "msg": "eyJ2ZXN0X2lkIjoxLCJ2ZXN0X3R5cGUiOiJ2ZXN0In0=",
  }
}
```

- **RegisterVestingAccount** registers a vesting account

```js
{
  "register_vesting_account": {
    "address": "cosmos1...",
    "master_address": "cosmos1...",
    "vesting_schedule": {
      "linear_vesting": {
        "start_time": "1703772805",
        "end_time": "1703872805",
        "vesting_amount": "1000000"
      }
    }
  }
}
```

- **DeregisterVestingAccount** deregisters a vesting account

```js
{
  "deregister_vesting_account": {
    "address": "cosmos1...",
    "denom": "uusd",
    "vested_token_recipient": "cosmos1...", // address that will receive the vested tokens after deregistration. If None, tokens are received by the owner address.
    "left_vested_token_recipient": "cosmos1...", // address that will receive the left vesting tokens after deregistration.
  }
}
```

- **Claim** allows to claim vested tokens

```js
{
  "claim": {
    "denom": "uusd",
    "recipient": "cosmos1...",
  }
}
```

### Query

- **VestingAccount** returns the vesting account details for a given address.

```js
{
  "vesting_account": {
    "address": "cosmos1...",
  }
}
```

## 4. Nibi Stargate

This smart contract showcases usage examples for certain Nibiru-specific and Cosmos-SDK-specific.

### 4.1 Instantiate

There's no instantiation message.

```js
{
}
```

### 4.2 Execute

- **CreateDenom** creates a new denom

```js
{
  "create_denom": { "subdenom": "zzz" }
}
```

- **Mint** mints tokens

```js
{
  "mint": {
    "coin": { "amount": "[amount]", "denom": "tf/[contract-addr]/[subdenom]" },
    "mint_to": "[mint-to-addr]"
  }
}
```

- **Burn** burns tokens

```js
{
  "burn": {
    "coin": { "amount": "[amount]", "denom": "tf/[contract-addr]/[subdenom]" },
    "burn_from": "[burn-from-addr]"
  }
}
```

- **ChangeAdmin** changes the admin of a denom

```js
{
  "change_admin": {
    "denom": "tf/[contract-addr]/[subdenom]",
    "new_admin": "[ADDR]"
  }
}
```

## 5. Nibi Stargate Perp

This smart contract showcases usage examples for certain Nibiru-specific for the perp market.

### 5.1 Instantiate

The instantiation defines the owner of the contract, who will be able to add and remove addresses from the whitelist, and execute the shifts.

```js
{
  "admin": "cosmos1...",
}
```

### 5.2 Execute

- **MarketOrder** places a market order for a specified trading pair. `pair` indicates the trading pair, `is_long` determines if it's a long or short order, `quote_amount` is the amount in the quote currency, `leverage` specifies the leverage to apply, and `base_amount_limit` sets a limit for the amount in the base currency.

```js
{
  "market_order": {
    "pair": "BTC/USDT",
    "is_long": true,
    "quote_amount": "1000000",
    "leverage": "2.0",
    "base_amount_limit": "5000000"
  }
}
```

- **ClosePosition** closes an open position for a specified trading pair.

```js
{
  "close_position": {
    "pair": "BTC/USDT"
  }
}
```

- **AddMargin** adds margin to an existing position for a specified trading pair. `margin` is the amount of additional margin to add.

```js
{
  "add_margin": {
    "pair": "BTC/USDT",
    "margin": {"denom": "usdt", "amount": "100000"}
  }
}
```

- **RemoveMargin** removes margin from an existing position for a specified trading pair. `margin` is the amount of margin to remove.

```js
{
  "remove_margin": {
    "pair": "BTC/USDT",
    "margin": {"denom": "usdt", "amount": "50000"}
  }
}
```

- **MultiLiquidate** triggers multiple liquidations based on the provided arguments. `liquidations` is a list of liquidation arguments specifying the details for each liquidation.

```js
{
  "multi_liquidate": {
    "liquidations": [
      {
        "pair": "BTC/USDT",
        "trader": "cosmos1...",
      },
      {
        "pair": "BTC/USDT",
        "trader": "cosmos1...",
      }
    ]
  }
}
```

- **DonateToInsuranceFund** allows donation to the insurance fund. `donation` is the coin and amount to donate.

```js
{
  "donate_to_insurance_fund": {
    "donation": {"denom": "usdt", "amount": "100000"}
  }
}
```

- **Claim** facilitates the claiming of funds. `funds` is an optional field specifying a particular coin and amount to claim, `claim_all` is an optional flag to claim all funds, and `to` is the address to which the funds will be sent.

```js
{
  "claim": {
    "funds": {"denom": "usdt", "amount": "100000"},
    "claim_all": true,
    "to": "cosmos1..."
  }
}
```

This format aligns with the style of your previous documentation, ensuring consistency and clarity in the explanation of each function and its parameters.

## 6. Nusd Valuator

This smart contract is a simple valuator for the nusd token, which takes one collateral.

### 6.1 Instantiate

The owner is the only one who can execute messages in the contract

```js
{
  "owner": "cosmos1...",
  "accepted_denoms": "uusdc",
}
```

### 6.2 Execute

- **ChangeDenom** updates the accepted denoms

```js
{
  "change_denom": {
    "from: "uusdc",
    "to": "uusd",
  }
}
```

- **AddDenom** adds a new accepted denom

```js
{
  "add_denom": {
    "denom": "uusd",
  }
}
```

- **RemoveDenom** removes an accepted denom

```js
{
  "remove_denom": {
    "denom": "uusd",
  }
}
```

### 6.3 Query

- **Mintable** queries the amount of μNUSD that can be minted in exchange for the specified set of `from_coins`.

```js
{
  "mintable": {
    "from_coins": ["BTC", "ETH"]
  }
}
```

- **Redeemable** calculates the amount of a specified `to_denom` currency that is redeemable for a given `redeem_amount` of μNUSD.

```js
{
  "redeemable": {
    "redeem_amount": "1000000",
    "to_denom": "usdt"
  }
}
```

- **AcceptedDenoms** retrieves the set of token denominations that are accepted as collateral.

```js
{
  "accepted_denoms": {}
}
```

- **RedeemableChoices** provides a set of possible redeemable coin options that could be received when redeeming a specified `redeem_amount` of μNUSD.

```js
{
  "redeemable_choices": {
    "redeem_amount": "1000000"
  }
}
```

## 7. Airdrop token vesting

This contract implements vesting accounts for the native tokens.

### 7.1 Instantiate

We need to specify admin and managers

```javascript
{
  "admin": "cosmos1...",
  "managers": ["cosmos1...", "cosmos1..."]
}
```

### 7.2 Execute

- **RewardUsers** registers several vesting contracts

```javascript
{
  "reward_users": {
    "rewards": [
      {
        "user_address": "cosmos1...",
        "vesting_amount": "1000000",
        "cliff_amount": "100000", // Only needed if vesting schedule is linear with cliff
      }
    ],
    "vesting_schedule": {
      "linear_vesting": {
        "start_time": "1703772805",
        "end_time": "1703872805",
        "vesting_amount": "0" // This amount does not matter
      }
    }
  }
}
```

- **DeregisterVestingAccount** deregisters a vesting account

```javascript
{
  "deregister_vesting_account": {
    "address": "cosmos1...",
    "vested_token_recipient": "cosmos1...", // address that will receive the vested tokens after deregistration. If None, tokens are received by the owner address.
    "left_vested_token_recipient": "cosmos1...", // address that will receive the left vesting tokens after deregistration.
  }
}
```

- **Claim** allows to claim vested tokens

```javascript
{
  "claim": {
    "recipient": "cosmos1...",
  }
}
```

### 7.3 Query

- **VestingAccount** returns the vesting account details for a given address.

```javascript
{
  "vesting_account": {
    "address": "cosmos1...",
  }
}
```

## 8. Auto compounder

This contract manages staking re-delegation processes securely, allowing for auto-compounding of staked funds.

### 8.1 Instantiate

We need to specify admin and managers

```javascript
{
  "admin": "cosmos1...",
  "managers": ["cosmos1...", "cosmos1..."]
}
```

### 8.2 Execute

#### Admin functions

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

#### Manager functions

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

### 8.3 Query

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