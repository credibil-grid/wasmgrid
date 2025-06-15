# Key Value Example

Build the example guest:

```bash
cargo build --package keyvalue --target wasm32-wasip2 --release
```

Run the example guest:

```bash
# compile and run
cargo run -- run ./target/wasm32-wasip2/release/keyvalue.wasm

# pre-compile
cargo run -- compile  ./target/wasm32-wasip2/release/keyvalue.wasm --output ./keyvalue.bin
cargo run -- run ./keyvalue.bin
```

In a separate console, send a messages to the guest:

```bash
curl -d '{"text":"hello"}' http://localhost:8080
```
