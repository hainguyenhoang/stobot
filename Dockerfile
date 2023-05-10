FROM rust:bookworm as builder
WORKDIR /usr/src/stobot
COPY . .
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/src/stobot/target \
    cargo install --path .

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y libssl3 ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/stobot /usr/local/bin/stobot
ENTRYPOINT ["stobot"]
