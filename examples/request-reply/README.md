# Pub/Sub Example

This example sends a request to a topic and waits for the reply. 

An HTTP request is used to initiate the request and return the response from 
the messaging server.

## Build and run

Build the example guest:

```bash
cargo build --package request-reply --target wasm32-wasip2 --release
```

Run the example guest:

```bash
# compile and run
cargo run -- run ./target/wasm32-wasip2/release/request_reply.wasm

# pre-compile
cargo run -- compile  ./target/wasm32-wasip2/release/request-reply.wasm --output ./request_reply.bin
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