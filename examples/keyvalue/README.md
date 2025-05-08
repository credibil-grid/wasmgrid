# Key Value Example

Build the example guest:

```bash
cargo build --package keyvalue --target wasm32-wasip2 --release
```

Run the guest using the `wasmgrid` runtime:

```bash
cargo run -- compile  ./target/wasm32-wasip2/release/keyvalue.wasm
cargo run -- run ./keyvalue.bin
# OR
cargo run -- run --compile ./target/wasm32-wasip2/release/keyvalue.wasm
```

In a separate console, send a messages to the guest:

```bash
curl -d '{"text":"hello"}' http://localhost:8080
```
