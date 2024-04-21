# Http Example

Build the example guest:

```bash
cargo component build --package http-guest --release
```

Run a guest using the runtime:

```bash
cargo run -- --wasm ./target/wasm32-wasi/release/http_guest.wasm
```

In a separate console, call the guest:

```bash
curl -d '{"text":"hello"}' http://localhost:8080
```