pub mod msgs;
pub mod queries;

#[cfg(not(feature = "library"))]
// When imported with the "library" feature, contract.rs will not be compiled.
// This prevents errors related to entry the smart contract's entrypoints,
// enabling its use as a library.
pub mod contract;
pub mod error;
pub mod events;
pub mod state;

#[cfg(test)]
pub mod testing;
