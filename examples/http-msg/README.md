# Http-Msg Example

Build the example guest:

```bash
cargo component build --package http-msg --release
```

Run a guest using the runtime:

```bash
cargo run -- ./target/wasm32-wasi/release/http_msg.wasm
```

In a separate console, call the guest:

```bash
curl -d '{"text":"hello"}' http://localhost:8080
```

Or, send some messages:

```bash
cargo test --test publish
```