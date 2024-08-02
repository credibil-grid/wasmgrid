# RPC Example

This example demonstrates how to use the `wrpc` crate to create a simple
client-server application.

Build the client and server:

```bash
cargo component build -Z build-std=panic_abort,std --package rpc-client --release
cargo component build -Z build-std=panic_abort,std --package rpc-server --release
```

Run both client and server in separate consoles:

```bash
cargo run -- ./target/wasm32-wasip1/release/rpc_client.wasm
cargo run -- ./target/wasm32-wasip1/release/rpc_server.wasm
```

In a separate console, call the client:

```bash
curl -d '{"text":"hello"}' http://localhost:8080
```
