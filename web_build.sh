RUSTFLAGS='-C target-feature=+atomics,+bulk-memory,+mutable-globals' \
  cargo build --target wasm32-unknown-unknown -Z build-std=std,panic_abort --example $1 ${@:2}
cp target/wasm32-unknown-unknown/debug/examples/$1.wasm web_build/wasm.wasm
cp crates/kwasm/js/kwasm.js web_build/kwasm.js