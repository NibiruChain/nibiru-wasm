# NibiruChain/cw-nibiru

Smart contract prototyping sandbox for Nibiru Chain.

```bash
⚡ NibiruChain/cw-nibiru
├── 📂 artifacts-cw-plus # compiled .wasm binaries from the cw-plus contracts
├── 📂 contracts         # Contracts for Nibiru Chain
    └── 📂 incentives    # Generalized incentives over time for locked tokens
    └── 📂 lockup        # For locking and unlocking tokens like LP tokens
    └── 📂 pricefeed     # CosmWasm version of the (now deprecated) x/pricefeed module.
    └── 📂 whitelist     # Whitelist contract for managing `members` from a set of `admins`.
├── Cargo.toml
├── Cargo.lock
├── README.md
```

🚧 Work in progress 🚧


---

### Scratch paper - whitelist contract:

- [ ] Use a new `Member` struct to turn the members into an incentivized testnet reward recipient.
- [ ] test(whitelist): `IsWhitelisted`
- [ ] feat(whitelist): integrate the whitelist item into contract.rs
- [ ] feat(whitelist): perform address validation before adding to any whitelist
- [ ] test(whitelist): query that returns all admins
