cargo +nightly build --target wasm32-unknown-unknown --example $1 ${@:2} --release
cp target/wasm32-unknown-unknown/release/examples/$1.wasm web_build/wasm.wasm
cp crates/kwasm/js/kwasm.js web_build/kwasm.js