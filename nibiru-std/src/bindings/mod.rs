//! bindings/mod.rs: Nibiru `CosmosMsg::Custom<T>` module bindings for smart contracts.
//!
//! Bindings refer to a layer of code that allows two different programming
//! languages or systems to communicate with each other.
//! In the context of CosmWasm smart contracts and the Cosmos SDK, bindings
//! allow the Go-based Cosmos SDK to interact with and execute smart contracts,
//! which are written in Rust and compiled to WebAssembly (Wasm).

pub mod msg;
