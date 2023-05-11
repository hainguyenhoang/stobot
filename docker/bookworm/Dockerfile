FROM rust:bookworm as builder
ENV RUST_BACKTRACE=1
WORKDIR /usr/src/stobot
COPY . .
RUN --mount=type=cache,target=/usr/local/cargo/registry,id=registry \
    --mount=type=cache,target=/usr/src/stobot/target,id=bookworm_target \
    ls && cargo install --path .

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/stobot /usr/local/bin/stobot
ENTRYPOINT ["stobot"]