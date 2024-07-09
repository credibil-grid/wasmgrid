# Http-Http Example

This example demonstrates how to make an outgoing http request to a downstream service (the `http` example).

## Running the example

Build the example guest:

```bash
cargo component build --package http-http --release
```

Run a guest using the runtime:

```bash
cargo run -- --http-addr localhost:8080 ./target/wasm32-wasi/release/http_http.wasm
```

In a separate console, call the guest which will in turn call the downstream service
at <https://jsonplaceholder.cypress.io>:

```bash
curl -d '{"title": "foo","body": "bar", "userId": 1}' http://localhost:8080/post
```