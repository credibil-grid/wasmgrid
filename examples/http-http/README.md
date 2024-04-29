# Http-Http Example

This example demonstrates how to make an outgoing http request to a downstream service (the `http` example).

## Running the example

First, start the downstream service:

```bash
# build the http guest
cargo component build --package http --release

# build the `wasmgrid` binary
cargo build

# run the binary (on localhost:8080)
./target/debug/wasmgrid --http-addr localhost:8080 ./target/wasm32-wasi/release/http.wasm
```

Build the example guest:

```bash
cargo component build --package http-http --release
```

Run a guest using the runtime:

```bash
cargo run -- --http-addr localhost:8081 ./target/wasm32-wasi/release/http_http.wasm
```

In a separate console, call the guest:

```bash
curl -d '{"text":"hello"}' http://localhost:8081
```