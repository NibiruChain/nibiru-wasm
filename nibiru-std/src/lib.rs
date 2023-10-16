/// bindings/mod.rs: Custom Nibiru module bindings for smart contracts.
///
/// Bindings refer to a layer of code that allows two different programming
/// languages or systems to communicate with each other.
/// In the context of CosmWasm smart contracts and the Cosmos SDK, bindings
/// allow the Go-based Cosmos SDK to interact with and execute smart contracts,
/// which are written in Rust and compiled to WebAssembly (Wasm).
pub mod bindings;

/// proto/mod.rs: Protobuf types defined in NibiruChain/nibiru/proto.
pub mod proto;

pub mod wasm;

pub mod errors;

pub const VERSION_COSMOS_SDK: &str = "v0.47.5";
pub const VERSION_NIBIRU: &str = "240c7fba3ef38ac066c9a543a9028d6484d6374f";
