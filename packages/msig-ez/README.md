# cw-nibiru/packages/msig-ez

Easily create multisig wallets and signing multisig transactions.

- [Usage](#usage)
- [Desired Functionality](#desired-functionality)

## Usage

🚧 Work in progress 🚧

## Desired Functionality

- [ ] Add an input of pubkeys from a JSON file, `msmem.json`, to the keyring
- [ ] The tool should error if no `msmem.json` exists. 
- [x] Parse "keys show" cleanly
- [ ] Generate a multisig key from the non-multi keys of `msmem.json` and add it to the keyring with name "generated".
- [ ] The `threshold` should be an argument when running the script. 

- [ ] Add deterministic retrieval of the `nibid` binary for tests.
- [ ] Store current state of the CLI application in `~/.local/nibi_dev/msig-ez`. 
  This could include tx history, individually signed txs for msig txs.

- [ ] Create a user-friendly way to make unsigned tx JSONs by passing in  
- [ ] Sign a given msig tx.json using one of the keys on your keyring.
  - [ ] Offline means the key is not available.
  - [ ] Valid signers must have type "local"