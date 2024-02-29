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

#### By admin and managers

#### By admin only

### Deployed Contract Info

TODO for mainnet/testnet

| Field         | Value |
| ------------- | ----- |
| code_id       | ...   |
| contract_addr | ...   |
| rpc_url       | ...   |
| chain_id      | ...   |

### Testing Against a Live Chain
