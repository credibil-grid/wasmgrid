# Vault Example

Build the example guest:

```bash
cargo build --package vault --target wasm32-wasip2 --release
```

Run the example guest:

```bash
# compile and run
cargo run -- run ./target/wasm32-wasip2/release/vault.wasm

# pre-compile
cargo run -- compile  ./target/wasm32-wasip2/release/vault.wasm --output ./vault.bin
cargo run -- run ./vault.bin
```

In a separate console, send some messages to the guest:

```bash
curl -d '{"text":"hello"}' http://localhost:8080
```
