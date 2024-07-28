# See https://shaneutt.com/blog/rust-fast-small-docker-image-builds

FROM rust:alpine3.19 AS builder

RUN rustup update && \
    rustup target add x86_64-unknown-linux-musl

RUN apk --update add --no-cache musl-dev ca-certificates

RUN adduser --disabled-password --gecos "" --home "/nonexistent" \
    --shell "/sbin/nologin" --no-create-home --uid 10001 "wasm"

WORKDIR /app

COPY /Cargo.toml /Cargo.toml
COPY /src /src
COPY /wit /wit
COPY /crates /crates
COPY /examples /examples
COPY /rust-toolchain.toml /rust-toolchain.toml

RUN cargo build --target x86_64-unknown-linux-musl --release

FROM scratch
COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/ca-certificates.crt
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group
COPY --from=builder --chown=wasm:wasm /target/x86_64-unknown-linux-musl/release/wasmgrid /app/wasmgrid

USER wasm:wasm
EXPOSE 8080
CMD ["/app/wasmgrid", "/app.wasm"]
