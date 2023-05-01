# NibiruChain/cw-nibiru

Smart contract prototyping sandbox for Nibiru Chain.

```bash
⚡ NibiruChain/cw-nibiru
├── 📂 artifacts-cw-plus # compiled .wasm binaries from the cw-plus contracts
├── 📂 contracts         # Contracts for Nibiru Chain
    └── 📂 bindings-perp # Exposes queries and messages of the x/perp module of Nibiru.
    └── 📂 incentives    # Generalized incentives over time for locked tokens
    └── 📂 lockup        # For locking and unlocking tokens like LP tokens
    └── 📂 pricefeed     # CosmWasm version of the (now deprecated) x/pricefeed module.
    └── 📂 whitelist     # Whitelist contract with `members` that can call peg an depth shift.
├── Cargo.toml
├── Cargo.lock
├── README.md
```

<!-- 🚧 Work in progress 🚧 -->
