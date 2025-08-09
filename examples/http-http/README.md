# Http-Http Example

This example demonstrates how to make an outgoing http request to a downstream service (the `http` example).

## Running the example

Build the example guest:

```bash
cargo build --package http-http --target wasm32-wasip2 --release
```

Run a guest using the runtime:

```bash
# compile and run
cargo run -- run ./target/wasm32-wasip2/release/http_http.wasm

# pre-compile
cargo run -- compile  ./target/wasm32-wasip2/release/http_http.wasm --output ./http_http.bin
cargo run -- run ./http_http.bin
```


In a separate console, call the guest which will in turn call the downstream service
at <https://jsonplaceholder.cypress.io>:

```bash
# get
curl http://localhost:8080

# post
curl -d '{"title": "foo","body": "bar", "userId": 1}' http://localhost:8080
```