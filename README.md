# Wasm Grid Runtime

An interim runtime for running WASI WebAssembly components.

This wasm runtime is a stop-gap solution until production-grade runtimes such as [`wasmtime`](https://github.com/bytecodealliance/wasmtime)
can support components based on the [`wasi-cloud-core`](https://github.com/WebAssembly/wasi-cloud-core) specifications.

## Quick Start

Add a `.env` file in the root of the project (see `.env.example`)

In order for a connection to Azure Key Vault to be made use the Azure CLI to authenticate before running examples:

```shell
az login
```

### Messaging Example

[examples/pub-sub/README.md](examples/pub-sub/README.md).
[examples/request-reply/README.md](examples/request-reply/README.md).

### Http Example

See [examples/http/README.md](examples/http/README.md).

## Docker

### Build and Run

```bash
docker build --platform=linux/arm64 -t acrcredibil.azurecr.io/demo/wasmgrid .
docker compose up
```

### Compiling a wasm component

```bash
docker run \
	-v ./target/wasm32-wasip2/release/keyvalue.wasm:/app.wasm \
	acrcredibil.azurecr.io/demo/wasmgrid \
	/app/wasmgrid compile /app.wasm
```

## Troubleshooting

### MUSL build

To test the MUSL build locally:

```bash
# brew install FiloSottile/musl-cross/musl-cross
brew tap messense/macos-cross-toolchains
brew install aarch64-unknown-linux-musl
brew install openssl
```

```bash
cargo build --package wasmgrid --target aarch64-unknown-linux-musl --release
```

See <https://docs.wasmtime.dev/examples-minimal.html/> for more information on 
optimising embedded `wasmtime`builds.

### Debugging bindgen! macro

```bash
cargo expand --manifest-path crates/services/Cargo.toml keyvalue::generated > expanded.rs
```

## Nex

```bash
cp target/aarch64-unknown-linux-musl/release/wasmgrid ~/Downloads/wasmgrid
cp target/wasm32-wasi/release/http.wasm ~/Downloads/http.wasm

sudo cp /mnt/macos/wasmgrid /home/ubuntu/nex/wasmgrid
sudo cp /mnt/macos/http.wasm /home/ubuntu/nex/http.wasm

```