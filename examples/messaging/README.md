# Messaging Example

Build the example guest:

```bash
cargo build --package messaging --target wasm32-wasip2 --release
```

Run the example guest:

```bash
# compile and run
cargo run -- run ./target/wasm32-wasip2/release/messaging.wasm

# pre-compile
cargo run -- compile  ./target/wasm32-wasip2/release/messaging.wasm --output ./messaging.bin
cargo run -- run ./http.bin
```

In a separate console, send some messages to the guest:

```bash
cargo test -p services --test messaging
```
