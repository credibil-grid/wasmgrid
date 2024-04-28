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

Build and run the Messaging host and example guest.

[examples/messaging/README.md](examples/messaging/README.md).

### Http Example

Build and run the Http host and example guest.

See [examples/http/README.md](examples/http/README.md).


## Docker Build

In order to use the scratch image we need to build a statically linked (elf) binary. This can be done by using the `x86_64-unknown-linux-musl` target.

To build the docker image:

```bash
docker build -t acrcredibil.azurecr.io/demo/wasmgrid .
```

To test/debug build locally:

```bash
# brew install FiloSottile/musl-cross/musl-cross
brew tap messense/macos-cross-toolchains
brew install x86_64-unknown-linux-musl
```

```bash
cargo build --package wasmgrid --target x86_64-unknown-linux-musl --release
```

