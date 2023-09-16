#!/bin/sh
#
# wasm-out.sh
#
# Compile contracts into .wasm bytecode using rust-optimizer
# Optimize to reduce gas
#
# Ref: https://github.com/CosmWasm/rust-optimizer

# Compiles CosmWasm smart contracts to WebAssembly bytecode (.wasm)
wasm() {
  local image="$1"
  local image_version="0.14.0"
  docker run --rm -v "$(pwd)":/code \
    --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
    --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
    cosmwasm/$image:$image_version
}

# wasm - single contract
wasm_single() {
  echo "Running rust-optimizer for a single contract (cosmwasm/rust-optimizer)"
  wasm rust-optimizer
}

# wasm - all workspace contracts
wasm_all() {
  echo "Running rust-optimizer for all contracts (cosmwasm/workspace-optimizer)"
  wasm workspace-optimizer
}


main() {
  # Check for the "--single" flag to run the appropriate function.
  if [ "$1" = "--single" ]; then
    wasm_single
  else
    wasm_all
  fi
}

if main; then
  echo "🔥 Compiled all smart contracts successfully. "
else
  echo "❌ Compilation failed."
fi