# Blobstore Example

Build the example guest:

```bash
cargo build --package blobstore --target wasm32-wasip2 --release
```

Run the example guest:

```bash
# compile and run
cargo run -- run ./target/wasm32-wasip2/release/blobstore.wasm

# pre-compile
cargo run -- compile  ./target/wasm32-wasip2/release/blobstore.wasm --output ./blobstore.bin
cargo run -- run ./blobstore.bin
```

In a separate console, send some messages to the guest:

```bash
curl -d '{"text":"hello"}' http://localhost:8080
```
