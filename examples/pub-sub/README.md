# Pub/Sub Example

This example subscribes to 2 topics, `a` and `b`. 

An HTTP request initiates the process of publishing to topic `a` which
subsequently publishes a message to topic `b`.

## Build and run

Build the example guest:

```bash
cargo build --package pub-sub --target wasm32-wasip2 --release
```

Run the example guest:

```bash
# compile and run
cargo run -- run ./target/wasm32-wasip2/release/pub_sub.wasm

# pre-compile
cargo run -- compile  ./target/wasm32-wasip2/release/pub-sub.wasm --output ./pub_sub.bin
cargo run -- run ./http.bin
```

In a separate console, call the guest:

```bash
curl -d '{"text":"hello"}' http://localhost:8080
```

Or, send some messages:

```bash
cargo test --test publish
```