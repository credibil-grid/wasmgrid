# Messaging Example

Build the example guest:

```bash
cargo build --package messaging --target wasm32-wasip2 --release
```

Run the guest using the `wasmgrid` runtime:

```bash
cargo run -- ./target/wasm32-wasip2/release/messaging.wasm
```

In a separate console, send some messages to the guest:

```bash
cargo test --test messaging
```
