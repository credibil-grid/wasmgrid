# Messaging Example

Build the example guest:

```bash
cargo build --package jsondb --target wasm32-wasip2 --release
```

Run the guest using the `wasmgrid` runtime:

```bash
cargo run -- ./target/wasm32-wasip2/release/jsondb.wasm
```

In a separate console, send some messages to the guest:

```bash
curl -d '{"text":"hello"}' http://localhost:8080
```
