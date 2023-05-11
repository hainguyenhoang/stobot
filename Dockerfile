FROM rust:alpine3.17 as builder
ENV RUST_BACKTRACE=1
RUN apk add --no-cache musl-dev pkgconfig openssl-dev perl make
WORKDIR /usr/src/stobot
COPY . .
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/src/stobot/target \
    cargo install --path .

FROM alpine:3.17
COPY --from=builder /usr/local/cargo/bin/stobot /usr/local/bin/stobot
ENTRYPOINT ["stobot"]
