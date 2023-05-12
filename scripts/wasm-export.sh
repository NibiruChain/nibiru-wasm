# wasm-export.sh
wasm_binding_dir="../nibiru/x/wasm/binding"

copy_artifacts() {
  local wasmbin="$wasm_binding_dir/wasmbin"
  cp artifacts/bindings_perp.wasm $wasmbin/bindings_perp.wasm
  cp artifacts/shifter.wasm $wasmbin/shifter.wasm
}

copy_json_examples() {
  local json_example_dir="packages/dummy"
  cp $json_example_dir/query_resp.json $wasm_binding_dir/cw_struct/query_resp.json
  cp $json_example_dir/execute_msg.json $wasm_binding_dir/cw_struct/execute_msg.json
}

# main
copy_artifacts
copy_json_examples