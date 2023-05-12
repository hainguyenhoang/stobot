FROM rust:alpine3.17 as builder
ENV RUST_BACKTRACE=full
RUN apk add --no-cache musl-dev perl make
WORKDIR /usr/src/stobot
COPY . .
RUN --mount=type=cache,target=/usr/local/cargo/registry,id=registry \
    --mount=type=cache,target=/usr/src/stobot/target,id=alpine_target \
    ls && cargo install --path .

FROM alpine:3.17
COPY --from=builder /usr/local/cargo/bin/stobot /usr/local/bin/stobot
ENTRYPOINT ["stobot"]
