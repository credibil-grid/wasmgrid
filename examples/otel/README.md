# OTel Example

Build the example guest:

```bash
cargo build --package otel --target wasm32-wasip2 --release
```

Run the example guest:

```bash
# compile and run
cargo run -- run ./target/wasm32-wasip2/release/otel.wasm

# pre-compile
cargo run -- compile  ./target/wasm32-wasip2/release/otel.wasm --output ./otel.bin
cargo run -- run ./otel.bin
```

In a separate console, call the guest:

```bash
curl -d '{"text":"hello"}' http://localhost:8080
```