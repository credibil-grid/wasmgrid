# Messaging Example

Build the example guest:

```bash
cargo component build --package messaging --release
```

Run the guest using the `wasmrun` runtime:

```bash
cargo run -- ./target/wasm32-wasi/release/messaging.wasm
```

In a separate console, send some messages to the guest:

```bash
cargo test --test publish
```
