# Messaging Example

Build the example guest:

```bash
cargo component build --package messaging-guest --release
```

Run the guest using the `wasmrun` runtime:

```bash
cargo run -- --wasm ./target/wasm32-wasi/release/messaging_guest.wasm
```

In a separate console, send some messages to the guest:

```bash
cargo test --test publish
```
