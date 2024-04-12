# Wasi Messaging

Build guest wasm module:

```bash
cargo component build --package guest --release
```

Run messaging host using guest from previous step:

```bash
cargo run -- --wasm ./target/wasm32-wasi/release/guest.wasm
```


