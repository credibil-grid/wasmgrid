# Key Value Example

Build the example guest:

```bash
cargo component build -Z build-std=panic_abort,std --package keyvalue --release
```

Run the guest using the `wasmgrid` runtime:

```bash
cargo run -- ./target/wasm32-wasip1/release/keyvalue.wasm
```

In a separate console, send some messages to the guest:

```bash
curl -d '{"text":"hello"}' http://localhost:8080
```
