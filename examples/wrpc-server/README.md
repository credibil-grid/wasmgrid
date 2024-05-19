# Http-Msg Example

Build the example guest:

```bash
cargo component build --package wrpc-server --release
```

Run a guest using the runtime:

```bash
cargo run -- ./target/wasm32-wasi/release/wrpc_server.wasm
```

In a separate console, start the wrpc-client example and have it call the server.
