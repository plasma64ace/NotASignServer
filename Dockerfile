FROM rust:1.90.0-trixie AS builder

WORKDIR /build

# 复制源代码
COPY Cargo.lock Cargo.toml build.rs sign.config.toml symbols.c .
COPY src/ ./src/

RUN gcc -std=c99 -shared -fPIC -o libsymbols.so symbols.c && \
    cargo build --release

FROM debian:trixie-slim

WORKDIR /app

COPY --from=builder /build/libsymbols.so /usr/local/lib/libsymbols.so
COPY --from=builder /build/target/release/sign /usr/local/bin/sign

RUN ldconfig

COPY sign.config.toml wrapper.node .

VOLUME ["/app"]

ENTRYPOINT ["sign"]
