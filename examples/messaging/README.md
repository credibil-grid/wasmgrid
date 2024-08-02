# Messaging Example

Build the example guest:

```bash
cargo component build -Z build-std=panic_abort,std --package msg --release
```

Run the guest using the `wasmgrid` runtime:

```bash
cargo run -- ./target/wasm32-wasip1/release/msg.wasm
```

In a separate console, send some messages to the guest:

```bash
cargo test --test messaging
```
