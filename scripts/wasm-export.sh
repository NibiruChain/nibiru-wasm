#!/usr/bin/env bash
# wasm-export.sh
wasm_binding_dir="../nibiru/x/wasm/binding"

copy_artifacts() {
  local wasmbin="$wasm_binding_dir/wasmbin"
  copy_artifact() {
    local contract_binary="$1"
    cp artifacts/$contract_binary $wasmbin/$contract_binary
  }

  copy_artifact bindings_perp.wasm
  copy_artifact shifter.wasm
  copy_artifact controller.wasm
}

copy_json_examples() {
  local json_example_dir="packages/dummy"
  cp $json_example_dir/query_resp.json $wasm_binding_dir/cw_struct/query_resp.json
  cp $json_example_dir/execute_msg.json $wasm_binding_dir/cw_struct/execute_msg.json
}

# main
copy_artifacts
copy_json_examples