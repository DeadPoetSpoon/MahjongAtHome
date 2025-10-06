FROM rust:latest AS builder

RUN apt-get update && \
    apt-get install -y libpq-dev && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /mahjong_at_home

COPY Cargo.toml Cargo.lock ./

RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src && \
    rm ./target/release/mahjong-at-home-api

COPY src ./src
COPY migrations ./migrations
COPY Rocket.toml ./Rocket.toml

RUN touch src/main.rs && cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && \
    apt-get install -y libpq5 && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /mahjong_at_home

COPY --from=builder /mahjong_at_home/target/release/mahjong-at-home-api ./
COPY --from=builder /mahjong_at_home/Rocket.toml ./Rocket.toml

EXPOSE 8000

CMD ["./mahjong-at-home-api"]
