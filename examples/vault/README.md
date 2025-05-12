# Vault Example

Build the example guest:

```bash
cargo build --package vault --target wasm32-wasip2 --release
```

Run the guest using the `wasmgrid` runtime:

```bash
cargo run -- compile  ./target/wasm32-wasip2/release/vault.wasm
cargo run -- run ./vault.bin
# OR
cargo run -- run --compile ./target/wasm32-wasip2/release/vault.wasm
```

In a separate console, send some messages to the guest:

```bash
curl -d '{"text":"hello"}' http://localhost:8080
```
