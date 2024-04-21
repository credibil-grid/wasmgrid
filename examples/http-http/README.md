# Http Example

Build the example guest:

```bash
cargo component build --package http-http --release
```

Run a guest using the runtime:

```bash
cargo run -- --wasm ./target/wasm32-wasi/release/http_http.wasm
```

In a separate console, call the guest:

```bash
curl http://localhost:8081
```