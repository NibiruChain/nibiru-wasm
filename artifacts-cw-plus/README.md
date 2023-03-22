# cw-nibiru/artifacts-cw-plus

The CosmWasm team provided several production quality smart contracts in the [CosmWasm/cw-plus](https://github.com/CosmWasm/cw-plus) repo. This directory simply contains the compiled `.wasm` output from each of these contracts.

### CW1 Proxy Contracts

- [`cw1-whitelist`](https://github.com/CosmWasm/cw-plus/tree/v1.0.1/contracts/cw1-whitelist) a minimal implementation of `cw1` mainly designed for reference.
- [`cw1-subkeys`](https://github.com/CosmWasm/cw-plus/tree/v1.0.1/contracts/cw1-subkeys) a simple, but useful implementation, which lets us use a proxy contract to
  provide "allowances" for native tokens without modifying the `bank` module.

### CW3 Multisig

- [`cw3-fixed-multisig`](https://github.com/CosmWasm/cw-plus/tree/v1.0.1/contracts/cw3-fixed-multisig) a simple implementation of the
  [cw3 spec](https://github.com/CosmWasm/cw-plus/blob/main/packages/cw3/README.md). It is a multisig with a fixed set of addresses, created upon initialization.
  Each address may have the same weight (K of N), or some may have extra voting power. This works much like the native
  Cosmos SDK multisig, except that rather than aggregating the signatures off chain and submitting the final result, we
  aggregate the approvals on-chain.
- [`cw3-flex-multisig`](https://github.com/CosmWasm/cw-plus/tree/v1.0.1/contracts/cw3-flex-multisig) builds on cw3-fixed-multisig, with a more powerful implementation
  of the cw3 spec. It's a multisig contract backed by a cw4 (group) contract, which independently maintains the voter
  set.
  

### CW4 Group

- [`cw4-group`](https://github.com/CosmWasm/cw-plus/tree/v1.0.1/contracts/cw4-group) a basic implementation of the [cw4 spec](https://github.com/CosmWasm/cw-plus/blob/main/packages/cw4/README.md). It handles
  elected membership, by admin or multisig. It fulfills all elements of the spec, including raw query lookups, and is
  designed to be used as a backing storage for [cw3 compliant contracts](https://github.com/CosmWasm/cw-plus/blob/main/packages/cw3/README.md).
- [`cw4-stake`](https://github.com/CosmWasm/cw-plus/tree/v1.0.1/contracts/cw4-stake) a second implementation of the [cw4 spec](https://github.com/CosmWasm/cw-plus/tree/v1.0.1/packages/cw4/README.md). It fulfills
  all elements of the spec, including raw query lookups, and is designed to be used as a backing storage for
  [cw3 compliant contracts](https://github.com/CosmWasm/cw-plus/tree/v1.0.1/packages/cw3/README.md). It provides a similar API to [`cw4-group`], but rather than
  appointing members, their membership and weight are based on the number of staked tokens they have.

### CW20 Fungible Tokens

- [`cw20-base`](https://github.com/CosmWasm/cw-plus/tree/v1.0.1/contracts/cw20-base) a straightforward, but complete implementation of the cw20 spec along with all
  extensions. Can be deployed as-is, or imported by other contracts.

# Instructions for Reproducing the Binaries

1. Clone the [CosmWasm/cw-plus](https://github.com/CosmWasm/cw-plus) repository.

    ```bash
    git clone git@github.com:CosmWasm/cw-plus.git
    cd cw-plus # path to the repo.
    ```

2. Start the Docker daemon, and run the following to compile all the contracts:

    ```bash
    docker run --rm -v "$(pwd)":/code \
      --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
      --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
      cosmwasm/workspace-optimizer:0.12.11
    ```

    This will compile all packages in the `contracts` directory and output the stripped and optimized wasm code under the `artifacts` directory as output, along with a `checksums.txt` file.

3. If that (↑) worked, you're done. 
4. If you encounter any issues and want to debug, you can try to run the following in each contract directory: 
    ```bash
    RUSTFLAGS="-C link-arg=-s" cargo build --release --target=wasm32-unknown-unknown --locked
    ```
