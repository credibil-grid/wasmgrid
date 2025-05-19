# Http Example

Build the example guest:

```bash
cargo build --package http@0.1.0 --target wasm32-wasip2 --release
```

Run a guest using the runtime:

```bash
cargo run -- compile  ./target/wasm32-wasip2/release/http.wasm --output ./http.bin
cargo run -- run ./http.bin
```

In a separate console, call the guest:

```bash
curl -d '{"text":"hello"}' http://localhost:8080
```