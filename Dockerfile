FROM rust:1.91-bookworm AS builder
WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY migration ./migration

RUN cargo build --release -p gps_tracker

FROM debian:bookworm-slim AS runtime
WORKDIR /app

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/gps_tracker /usr/local/bin/gps_tracker

ENV BIND_ADDR=0.0.0.0:3000
EXPOSE 3000

CMD ["gps_tracker"]
