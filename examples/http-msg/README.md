# Http-Msg Example

Build the example guest:

```bash
cargo build --package http-msg --target wasm32-wasip2 --release
```

Run a guest using the runtime:

```bash
cargo run -- compile  ./target/wasm32-wasip2/release/http_msg.wasm
cargo run -- run ./http_msg.bin
# OR
cargo run -- run --compile ./target/wasm32-wasip2/release/http_msg.wasm
```

In a separate console, call the guest:

```bash
curl -d '{"text":"hello"}' http://localhost:8080
```

Or, send some messages:

```bash
cargo test --test publish
```