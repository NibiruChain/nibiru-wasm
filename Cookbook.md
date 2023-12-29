# Contracts Cookbook

This file describes the different messages that can be sent as queries or transactions
to the contracts of this repository with a description of the expected behavior.

## 1. Core cw3 flex multisig

This contract is a multisig contract that is backed by a cw4 (group) contract, which independently maintains the voter set.

### 1.1 Instantiate

```javascript
{
    "group_addr": "cosmos1...", // this is the group contract that contains the member list
    "threshold": {
        "absolute_count": {"weight": 2},
        "absolute_percentage": {"percentage": 0.5},
        "threshold_quorum": { "threshold": 0.1, "quorum": 0.2 }
    },
    "max_voting_period": "3600s",
    // who is able to execute passed proposals
    // None means that anyone can execute
    "executor": {},
    /// The cost of creating a proposal (if any).
    "proposal_deposit": {
        "denom": "uusd",
        "amount": "1000000"
    },
}
```

### 1.2 Execute

- **Propose** creates a message to be executed by the multisig. It can be executed by anyone.

```javascript
{
  "propose": {
    "title": "My proposal",
    "description": "This is a proposal",
    "msgs": [
      {
        "bank": {
          "send": {
            "from_address": "cosmos1...",
            "to_address": "cosmos1...",
            "amount": [{ "denom": "uusd", "amount": "1000000" }]
          }
        }
      }
    ],
    "latest": {
      "at_height": 123456
    }
  }
}
```

- **Vote** adds a vote to an existing proposal. It can be executed by anyone.

```javascript
{
  "vote": {
    "proposal_id": 1,
    "vote": "yes"
  }
}
```

- **Execute** executes a passed proposal. It can be executed by anyone.

```javascript
{
  "execute": {
    "proposal_id": 1
  }
}
```

- **Close** closes an expired proposal. It can be executed by anyone.

```javascript
{
  "close": {
    "proposal_id": 1
  }
}
```

### 1.3 Query

- **Threshold** returns the current threshold necessary for a proposal to be executed.

```javascript
{
  "threshold": {}
}
```

- **Proposal** fetches the details of a specific proposal given its ID.

```javascript
{
  "proposal": {
    "proposal_id": 1
  }
}
```

- **ListProposals** lists proposals with optional pagination. `start_after` specifies the ID after which to start listing, and `limit` sets the maximum number of proposals to return.

```javascript
{
  "list_proposals": {
    "start_after": 1,
    "limit": 10
  }
}
```

- **ReverseProposals** lists proposals in reverse order with optional pagination. `start_before` specifies the ID before which to start listing in reverse, and `limit` sets the maximum number of proposals to return.

```javascript
{
  "reverse_proposals": {
    "start_before": 10,
    "limit": 10
  }
}
```

- **Vote** retrieves the vote details for a given proposal ID and voter address.

```javascript
{
  "vote": {
    "proposal_id": 1,
    "voter": "cosmos1..."
  }
}
```

- **ListVotes** lists votes for a given proposal, with optional pagination. `start_after` specifies the address after which to start listing votes, and `limit` sets the maximum number of votes to return.

```javascript
{
  "list_votes": {
    "proposal_id": 1,
    "start_after": "cosmos1...",
    "limit": 10
  }
}
```

- **Voter** fetches details about a specific voter by their address.

```javascript
{
  "voter": {
    "address": "cosmos1..."
  }
}
```

- **ListVoters** lists voters with optional pagination. `start_after` specifies the address after which to start listing voters, and `limit` sets the maximum number of voters to return.

```javascript
{
  "list_voters": {
    "start_after": "cosmos1...",
    "limit": 10
  }
}
```

- **Config** retrieves the current configuration of the system.

```javascript
{
  "config": {}
}
```

## 2. Core shifter

Shifter is a simple contract that can execute peg and depth shift to any markets in the x/perp module of Nibiru.
The contract holds a whitelist of addressses that are allowed to execute the shift.

### 2.1 Instantiate

The instantiation defines just the onwer of the contract, who wil be able to add and remove addresses from the whitelist, and execute the shifts.

```javascript
{"owner": "cosmos1..."}
```

### 2.2 Execute

- **ShiftSwapInvariant** executes a depth shift in a market.

```javascript
{
  "shift_swap_invariant": {
    "pair": "uusd:usdr",
    "new_swap_invariant": "1000000"
  }
}
```

- **ShiftPegMultiplier** executes a depth shift on a market. It can be executed by anyone.

```javascript
{
  "shift_peg_multiplier": {
    "pair": "ubtc:unusd",
    "new_peg_mult": "20420.69"
  }
}
```

- **EditOpers** adds or removes addresses from the whitelist. It can be executed by the owner.

```javascript
{
  "edit_opers": {
    "add_oper": {"addr": "cosmos1..."},
    "remove_oper": {"addr": "cosmos1..."},
  }
}
```

### 2.3 Query

The queries have to do with checking permissions of addresses.

- **HasPerms** checks if an address has permissions to execute shifts.

```javascript
{
  "has_perms": {
    "address": "cosmos1..."
  }
}
```

- **Perms** query the contract owner and set of operators.

```javascript
{
  "perms": {},
}
```

## 3. Core token vesting

This contract implements vesting accounts for the CW20 and native tokens.

### 3.1 Instantiate

There's no instantiation message.

```javascript
{}
```

### 3.2 Execute

- **Receive** 

```javascript
{
  "receive": {
    "sender": "cosmos1...",
    "amount": "1000000",
    "msg": "eyJ2ZXN0X2lkIjoxLCJ2ZXN0X3R5cGUiOiJ2ZXN0In0=",
  }
}
```

- **RegisterVestingAccount** registers a vesting account

```javascript
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

```javascript
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

```javascript
{
  "claim": {
    "denom": "uusd",
    "recipient": "cosmos1...",
  }
}
```

### 3.3 Query

- **VestingAccount** returns the vesting account details for a given address.

```javascript
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

```javascript
{}
```

### 4.2 Execute

- **CreateDenom** creates a new denom

```javascript
{
  "create_denom": { "subdenom": "zzz" }
}
```

- **Mint** mints tokens

```javascript
{ 
  "mint": { 
    "coin": { "amount": "[amount]", "denom": "tf/[contract-addr]/[subdenom]" }, 
    "mint_to": "[mint-to-addr]" 
  } 
}
```

- **Burn** burns tokens

```javascript
{ 
  "burn": { 
    "coin": { "amount": "[amount]", "denom": "tf/[contract-addr]/[subdenom]" }, 
    "burn_from": "[burn-from-addr]" 
  } 
}
```

- **ChangeAdmin** changes the admin of a denom

```javascript
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

```javascript
{
  "admin": "cosmos1...",
}
```
### 5.2 Execute

- **MarketOrder** places a market order for a specified trading pair. `pair` indicates the trading pair, `is_long` determines if it's a long or short order, `quote_amount` is the amount in the quote currency, `leverage` specifies the leverage to apply, and `base_amount_limit` sets a limit for the amount in the base currency.

```javascript
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

```javascript
{
  "close_position": {
    "pair": "BTC/USDT"
  }
}
```

- **AddMargin** adds margin to an existing position for a specified trading pair. `margin` is the amount of additional margin to add.

```javascript
{
  "add_margin": {
    "pair": "BTC/USDT",
    "margin": {"denom": "usdt", "amount": "100000"}
  }
}
```

- **RemoveMargin** removes margin from an existing position for a specified trading pair. `margin` is the amount of margin to remove.

```javascript
{
  "remove_margin": {
    "pair": "BTC/USDT",
    "margin": {"denom": "usdt", "amount": "50000"}
  }
}
```

- **MultiLiquidate** triggers multiple liquidations based on the provided arguments. `liquidations` is a list of liquidation arguments specifying the details for each liquidation.

```javascript
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

```javascript
{
  "donate_to_insurance_fund": {
    "donation": {"denom": "usdt", "amount": "100000"}
  }
}
```

- **Claim** facilitates the claiming of funds. `funds` is an optional field specifying a particular coin and amount to claim, `claim_all` is an optional flag to claim all funds, and `to` is the address to which the funds will be sent.

```javascript
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

```javascript
{
  "owner": "cosmos1...",
  "accepted_denoms": "uusdc",
}
```

### 6.2 Execute

- **ChangeDenom** updates the accepted denoms

```javascript
{
  "change_denom": {
    "from: "uusdc",
    "to": "uusd",
  }
}
```

- **AddDenom** adds a new accepted denom

```javascript
{
  "add_denom": {
    "denom": "uusd",
  }
}
```

- **RemoveDenom** removes an accepted denom

```javascript
{
  "remove_denom": {
    "denom": "uusd",
  }
}
```

### 6.3 Query


- **Mintable** queries the amount of μNUSD that can be minted in exchange for the specified set of `from_coins`.

```javascript
{
  "mintable": {
    "from_coins": ["BTC", "ETH"]
  }
}
```

- **Redeemable** calculates the amount of a specified `to_denom` currency that is redeemable for a given `redeem_amount` of μNUSD.

```javascript
{
  "redeemable": {
    "redeem_amount": "1000000",
    "to_denom": "usdt"
  }
}
```

- **AcceptedDenoms** retrieves the set of token denominations that are accepted as collateral.

```javascript
{
  "accepted_denoms": {}
}
```

- **RedeemableChoices** provides a set of possible redeemable coin options that could be received when redeeming a specified `redeem_amount` of μNUSD.

```javascript
{
  "redeemable_choices": {
    "redeem_amount": "1000000"
  }
}
```
