# Wasm Grid Runtime

An interim runtime for running WASI WebAssembly components.

This wasm runtime is a stop-gap solution until production-grade runtimes such as [`wasmtime`](https://github.com/bytecodealliance/wasmtime)
can support components based on the [`wasi-cloud-core`](https://github.com/WebAssembly/wasi-cloud-core) specifications.

## Quick Start

Add a `.env` file in the root of the project with the following values:

```bash
# tracing log level, e.g. "wasmgrid=debug,async_nats=info,azure_core=info"
RUST_LOG="wasmgrid=debug" 
HTTP_ADDR="0.0.0.0:8080"
NATS_ADDR="demo.nats.io"
MGO_URI="<MongoDB connection string>"
AZURE_TENANT_ID=<Azure Tenant ID for key vault>
AZURE_CLIENT_ID=<Azure Client ID for key vault>
AZURE_CLIENT_SECRET=<Azure Client Secret for key vault>
# when using Synadia Cloud:
NATS_ADDR="tls://connect.ngs.global"
NATS_JWT="<user JWT>"
NATS_SEED="<user signing seed>"
WITH_CORS="true"
```

### Messaging Example

Build and run the Messaging host and example guest.

[examples/messaging/README.md](examples/messaging/README.md).

### Http Example

Build and run the Http host and example guest.

See [examples/http/README.md](examples/http/README.md).

## Docker Build and Run

Build 
Build the docker image:

```bash
docker build --platform=linux/amd64 -t acrcredibil.azurecr.io/demo/wasmgrid .
```

Push to Azure:

```bash
az acr login --name acrcredibil
docker push acrcredibil.azurecr.io/demo/wasmgrid
```

## Troubleshooting

### MUSL build

To test the MUSL build locally:

```bash
# brew install FiloSottile/musl-cross/musl-cross
brew tap messense/macos-cross-toolchains
brew install x86_64-unknown-linux-musl
brew install aarch64-unknown-linux-musl
brew install openssl
```

```bash
cargo build --package wasmgrid --target aarch64-unknown-linux-musl --release
# OPENSSL_DIR=/opt/homebrew/opt/openssl 
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