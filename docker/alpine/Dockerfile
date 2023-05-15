FROM rust:alpine3.17 as builder
ENV RUST_BACKTRACE=full
ENV RUSTFLAGS="-C target-feature=-crt-static"
RUN apk add --no-cache musl-dev openssl-dev
WORKDIR /usr/src/stobot
COPY . .
RUN --mount=type=cache,target=/usr/local/cargo/registry,id=registry \
    --mount=type=cache,target=/usr/src/stobot/target,id=alpine_target \
    cargo install --path .

FROM alpine:3.17
RUN apk add --no-cache libgcc
COPY --from=builder /usr/local/cargo/bin/stobot /usr/local/bin/stobot
ENTRYPOINT ["stobot"]
