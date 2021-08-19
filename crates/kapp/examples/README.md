# How to run

## Native

```cargo run --example hello```

## Web

From the kapp root folder run:

```bash
cargo build --example hello --target wasm32-unknown-unknown
wasm-bindgen target/wasm32-unknown-unknown/debug/examples/hello.wasm --out-dir examples/web_build --out-name example --no-typescript --target web
```

If you do not already have wasm-bindgen installed you can install it with:

```bash
cargo install wasm-bindgen-cli
```

Then use a local development server to host the contents of the `examples/web_build/` folder.
