workspaces := "./packages ./contracts "

# Displays available recipes by running `just -l`.
setup:
  #!/usr/bin/env bash
  just -l

install:
  # https://crates.io/crates/clippy
  rustup component add clippy
  # https://crates.io/crates/cargo-llvm-cov
  cargo install cargo-llvm-cov

wasm-all:
  bash scripts/wasm-out.sh

# Move binding artifacts to teh local nibiru wasmbin
wasm-export:
  bash scripts/wasm-export.sh

# Compiles a single CW contract to wasm bytecode.
# wasm-single:
#   bash scripts/wasm-out.sh --single

# Runs rustfmt
fmt:
  cargo fmt --all

# Runs rustfmt without updating
fmt-check:
  cargo fmt --all -- --check

# Compiles Rust code
build:
  cargo build

build-update:
  cargo update
  cargo build

# Clean target files and temp files
clean:
  cargo clean

# Run linter + fix
clippy:
  cargo clippy --fix --allow-dirty --allow-staged

# Run linter + check only
clippy-check:
  cargo clippy

# Test a specific package or contract
test *pkg:
  #!/usr/bin/env bash
  set -e;
  if [ -z "{{pkg}}" ]; then
    just test-all
  else
    RUST_BACKGTRACE="1" cargo test --package "{{pkg}}"
  fi

# Test everything in the workspace.
test-all:
  cargo test

# Test everything and output coverage report.
test-coverage:
  cargo llvm-cov --lcov --output-path lcov.info \
    --ignore-filename-regex .*buf\/[^\/]+\.rs$

# Format, lint, and test
tidy:
  just fmt
  just clippy
  just test

# Format, lint, update dependencies, and test
tidy-update: build-update
  just tidy