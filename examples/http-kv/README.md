# Messaging Example

Build the example guest:

```bash
cargo component build --package http-kv --release
```

Run the guest using the `wasmgrid` runtime:

```bash
cargo run -- ./target/wasm32-wasi/release/http_kv.wasm
```

In a separate console, send some messages to the guest:

```bash
curl -d '{"text":"hello"}' http://localhost:8080
```
