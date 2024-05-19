# Http-Msg Example

Build the client and server:

```bash
cargo component build --package wrpc-client --release
cargo component build --package wrpc-client --release
```

Run both client and server in separate consoles:

```bash
cargo run -- ./target/wasm32-wasi/release/wrpc_client.wasm
cargo run -- ./target/wasm32-wasi/release/wrpc_server.wasm
```

In a separate console, call the client:

```bash
curl -d '{"text":"hello"}' http://localhost:8080
```
