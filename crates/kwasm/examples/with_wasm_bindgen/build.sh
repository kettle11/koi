  cargo build --target wasm32-unknown-unknown -Z build-std=std,panic_abort
wasm-bindgen target/wasm32-unknown-unknown/debug/with_wasm_bindgen.wasm --out-dir web_build --out-name with_wasm_bindgen --no-typescript --target web
cp index.html web_build/index.html