# HTTP-P2P Example

This component will listen to a message on http then store the message on a local Iroh file-based node. Then it will attempt to read it back out and then finally delete it. You will need to provide some environment variables which you can do in a `.env` file in the root of the project. For
example:

```bash
RUST_LOG=debug
IROH_DATA_DIR="/Users/goldie/Library/Application Support/io.credibil.wallet"
IROH_PORT=11204
IROH_AUTHOR=zsv3g2zcqsyicbte
```

* The data directory can be anywhere locally available to the runtime.
* The port is the port that the Iroh node will listen on. If omitted, it will default to 11204.
* If the author ID is omitted or can't be found, a new author will be created and trace-written to
the console for you to use on subsequent runs.

Build the example guest:

```bash
cargo component build --package http-p2p --release
```

Run a guest using the runtime:

```bash
cargo run -- ./target/wasm32-wasi/release/http_p2p.wasm
```

In a separate console, call the guest:

```bash
curl -d '{"entries": [{"key": "text", "data": "hello"}]}' http://localhost:8080
```


