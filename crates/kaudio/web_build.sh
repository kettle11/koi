RUSTFLAGS='-C target-feature=+atomics,+bulk-memory,+mutable-globals' \
  cargo build --target wasm32-unknown-unknown --release -Z build-std=std,panic_abort --example sine
cp target/wasm32-unknown-unknown/release/examples/sine.wasm examples/web_build/sine.wasm