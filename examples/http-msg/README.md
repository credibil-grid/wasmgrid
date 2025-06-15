# Http-Msg Example

Build the example guest:

```bash
cargo build --package http-msg --target wasm32-wasip2 --release
```

Run the example guest:

```bash
# compile and run
cargo run -- run ./target/wasm32-wasip2/release/http_msg.wasm

# pre-compile
cargo run -- compile  ./target/wasm32-wasip2/release/http-msg.wasm --output ./http_msg.bin
cargo run -- run ./http.bin
```

In a separate console, call the guest:

```bash
curl -d '{"text":"hello"}' http://localhost:8080
```

Or, send some messages:

```bash
cargo test --test publish
```