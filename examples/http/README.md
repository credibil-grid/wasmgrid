# Http Example

Build the example guest:

```bash
cargo component build --package http@0.1.0 --release
```

Run a guest using the runtime:

```bash
cargo run -- ./target/wasm32-wasi/release/http.wasm
```

In a separate console, call the guest:

```bash
# simple request
curl -d '{"text":"hello"}' http://localhost:8080/hello

# proxy outbound request (https://jsonplaceholder.cypress.io)
curl -d '{"title": "foo","body": "bar", "userId": 1}' http://localhost:8080/out_post
```