# shifter 

"Shifter" is a simple contract that can be used to execute peg shift and
depth shifts in the x/perp module of Nibiru. The contract stores a whitelist
of addresses, managed by an admin. This whitelist design takes inspiration
from cw-plus/contracts/cw1-whitelist.

The contract initializes with an admin address and allows the admin to add
or remove addresses from the whitelist. Users can query whether an address
is whitelisted or not.

### Contained Functionality

1. Initialize the contract with an admin address.
2. Allow the admin to add or remove addresses from the whitelist.
3. Allow anyone to query if an address is on the whitelist.
4. Members of the whitelist set can execute permissioned calls on the Nibiru
   x/perp module for dynamic optimizations like peg shift and depth shift.

### Entry Points

- InitMsg: Initializes the contract with the admin address.

Save the following `shifter-0-inst.json`:
```json
{ "owner": "nibi1zaavvzxez0elundtn32qnk9lkm8kmcsz44g7xl" }
```

```bash
# FOR anyone
alias tx="jq -rcs '.[0].txhash' | { read txhash; sleep 3; nibid q tx \$txhash | jq '{txhash, height, code, logs, tx, gas_wanted, gas_used}' >> out.json}"
```

```bash
nibid tx wasm inst 1 "$(cat shifter-0-inst.json)" --label="Crazy tester" --no-admin --from=$KEY_NAME -y | tx
```

```bash
# FOR ME
alias tx="jq -rcs '.[0].txhash' | { read txhash; sleep 3; nibid q tx \$txhash | jq '{txhash, height, code, logs, gas_used, gas_wanted, tx}' | vv}"
```

CONTRACT=nibi14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9ssa9gcs
CONTRACT=nibi1yyca08xqdgvjz0psg56z67ejh9xms6l436u8y58m82npdqqhmmtqzvacjr


- ExecuteMsg: Enum for executing msgs
  - ExecuteMsg::DepthShift


sudo-add.json
```json
{
  "action": "add_contracts",
  "contracts": [
      "nibi14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9ssa9gcs",
      "nibi1yyca08xqdgvjz0psg56z67ejh9xms6l436u8y58m82npdqqhmmtqzvacjr"
  ],
  "sender": "nibi1zaavvzxez0elundtn32qnk9lkm8kmcsz44g7xl"
}
```

```bash
nibid tx sudo edit sudo-add.json -y --from=$KEY_NAME
```



32E82D3CF2BBD48EDFFFDA260E4EF78D1E2B865A6111985B86BD74CDE8BA921B

Save the following `shifter-1-exec-shift-peg.json`:
```json
{
  "shift_peg_multiplier": {
    "pair": "ubtc:unusd",
    "new_peg_mult": "400001"
  }
}
```

```bash
nibid tx wasm exec $CONTRACT "$(cat shifter-1-exec-shift-peg.json)"  --from=$KEY_NAME -y | tx
```

```bash
nibid 
```


  - ExecuteMsg::PegShift
  - ExecuteMsg::AddMember adds an address to the whitelist
  - ExecuteMsg::RemoveMember removes and address from the whitelist.
  - ExecuteMsg::ChangeAdmin lets the current admin set a new one.