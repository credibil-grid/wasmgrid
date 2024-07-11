# Http Example

Build the example guest:

```bash
cargo component build --package http@0.1.0 --release
```

Run a guest using the runtime:

```bash
cargo run -- ./target/wasm32-wasip1/release/http.wasm
```

In a separate console, call the guest:

```bash
curl -d '{"text":"hello"}' http://localhost:8080
```