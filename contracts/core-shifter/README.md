# shifter 

"Shifter" is a simple contract that can be used to execute peg shift and
depth shifts in the x/perp module of Nibiru. The contract stores a whitelist
of addresses, managed by an admin. This whitelist design takes inspiration
from cw-plus/contracts/cw1-whitelist.

The contract initializes with an admin address and allows the admin to add
or remove addresses from the whitelist. Users can query whether an address
is whitelisted or not.

- [Start Here: Localnet Guide](#start-here-localnet-guide)
  - [Set environment vars.](#set-environment-vars)
  - [Deploy the contract](#deploy-the-contract)
  - [Instantiate](#instantiate)
  - [Contract must be x/sudo state](#contract-must-be-xsudo-state)
- [Execute/Invoke the contract](#executeinvoke-the-contract)
- [TODO: Concepts](#todo-concepts)
  - [Entry Points](#entry-points)

### Contained Functionality

1. Initialize the contract with an admin address.
2. Allow the admin to add or remove addresses from the whitelist.
3. Allow anyone to query if an address is on the whitelist.
4. Members of the whitelist set can execute permissioned calls on the Nibiru
   x/perp module for dynamic optimizations like peg shift and depth shift.


## Start Here: Localnet Guide

### Set environment vars.
```bash
KEYNAME="validator"
nibid keys list | jq # Verify the name is in your keyring.

# This "tx" alias will help with reading the tx responses.
alias tx="jq -rcs '.[0].txhash' | { read txhash; sleep 3; nibid q tx \$txhash | jq '{txhash, height, code, logs, tx, gas_wanted, gas_used}' >> out.json}"
```

### Deploy the contract

```bash
nibid tx wasm store ../../artifacts/shifter.wasm --from="$KEYNAME" --gas=2000999 -y | tx 
```

Inside the response, you should see the store code where your wasm bytecode is
saved on the chain. It's likely "1" if you're working on a fresh local network.


```json
"events": [
  {
    "type": "message",
    "attributes": [
      {
        "key": "action",
        "value": "/cosmwasm.wasm.v1.MsgStoreCode"
      },
      {
        "key": "sender",
        "value": "nibi1zaavvzxez0elundtn32qnk9lkm8kmcsz44g7xl"
      },
      {
        "key": "module",
        "value": "wasm"
      }
    ]
  },
  {
    "type": "store_code",
    "attributes": [
      {
        "key": "code_checksum",
        "value": "ce1ff9cd1e5127ae94797f5d013074ef2d9fb4d6c41f7488ead3361626687d5d"
      },
      {
        "key": "code_id",
        "value": "2"
      }
    ]
  }
]
```

In this example, the store code is "2". To make a smart contract instance, you
need to broadcast a Wasm instantiate transaction for this contract bytecode.

### Instantiate

Save the following `shifter-0-inst.json`:

```json
{ "owner": "nibi1zaavvzxez0elundtn32qnk9lkm8kmcsz44g7xl" }
```

Then, use this command to broadcast the `InstantiateMsg`. The 1 is the store code.

```bash
nibid tx wasm inst 1 "$(cat shifter-0-inst.json)" --label="Crazy tester" --no-admin --from="$KEYNAME" -y | tx
```

### Contract must be x/sudo state


Save the following as `sudo-add.json`, but put your contract address inside the
list insted of the address below:
```json
{
  "action": "add_contracts",
  "contracts": [
      "nibi14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9ssa9gcs",
      "nibi1suhgf5svhu4usrurvxzlgn54ksxmn8gljarjtxqnapv8kjnp4nrs0gfase"
  ],
  "sender": "nibi1zaavvzxez0elundtn32qnk9lkm8kmcsz44g7xl"
}
```

Broadcast:

```bash
nibid tx sudo edit sudo-add.json -y --from=$KEYNAME
```

## Execute/Invoke the contract

Set env var for `$CONTRACT`.
```
CONTRACT="nibi1suhgf5svhu4usrurvxzlgn54ksxmn8gljarjtxqnapv8kjnp4nrs0gfase"
```

Save the following `shifter-1-exec-shift-peg.json`:
```json
{
  "shift_peg_multiplier": {
    "pair": "ubtc:unusd",
    "new_peg_mult": "400001"
  }
}
```

Broadcast:
```bash
nibid tx wasm exec $CONTRACT "$(cat shifter-1-exec-shift-peg.json)"  --from=$KEYNAME -y | tx
```





<!-- ```bash -->
<!-- # FOR ME -->
<!-- alias tx="jq -rcs '.[0].txhash' | { read txhash; sleep 3; nibid q tx \$txhash | jq '{txhash, height, code, logs, gas_used, gas_wanted, tx}' | vv}" -->
<!-- ``` -->


## TODO: Concepts

### Entry Points

- InitMsg: Initializes the contract with the admin address.
- ExecuteMsg: Enum for executing msgs
  - ExecuteMsg::DepthShift
  - ExecuteMsg::PegShift
  - ExecuteMsg::AddMember adds an address to the whitelist
  - ExecuteMsg::RemoveMember removes and address from the whitelist.
  - ExecuteMsg::ChangeAdmin lets the current admin set a new one.