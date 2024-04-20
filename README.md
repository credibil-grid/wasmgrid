# Wasm Runtime

An interim runtime for running WASI WebAssembly components.

This wasm runtime is a stop-gap solution until production-grade runtimes such as [`wasmtime`](https://github.com/bytecodealliance/wasmtime)
can support components based on the [`wasi-cloud-core`](https://github.com/WebAssembly/wasi-cloud-core) specifications.

## Quick Start

Install `cargo-component`:

```bash
cargo install cargo-component
```

### Messaging Example

Build the example guest:

```bash
cargo component build --package messaging-guest --release
```

Run a guest using the runtime:

```bash
cargo run -- --wasm ./target/wasm32-wasi/release/messaging_guest.wasm
```

In a separate console, send some messages to the guest:

```bash
cargo test --test publish
```

### Http Example

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