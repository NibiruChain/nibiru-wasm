#!/usr/bin/env bash
#
# wasm-out.sh
#
# Compile contracts into .wasm bytecode using rust-optimizer
# Optimize to reduce gas
#
# Ref: https://github.com/CosmWasm/rust-optimizer

set -e
script_path=$(dirname "$(readlink -f "$0")")
# shellcheck source=./bashlib.sh
source "$script_path/bashlib.sh"

# Compiles CosmWasm smart contracts to WebAssembly bytecode (.wasm)
wasm() {
  local image_version="0.15.0"
  local image="workspace-optimizer"
  docker run --rm -v "$(pwd)":/code \
    --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
    --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
    cosmwasm/$image:$image_version
}

main() {
  echo "Running rust-optimizer for all contracts (cosmwasm/workspace-optimizer)"
  wasm
  echo "🔥 Compiled all smart contracts successfully. "
}

if ! main "$@"; then
  log_error "Compilation failed."
fi