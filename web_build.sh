RUSTFLAGS='-C target-feature=+atomics,+bulk-memory,+mutable-globals -Clink-arg=--max-memory=4294967296' \
  cargo build --target wasm32-unknown-unknown -Z build-std=std,panic_abort --example $1 ${@:2} --release
cp target/wasm32-unknown-unknown/release/examples/$1.wasm web_build/wasm.wasm
cp crates/kwasm/js/kwasm.js web_build/kwasm.js