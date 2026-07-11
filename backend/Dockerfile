FROM rust:slim-bookworm AS builder
WORKDIR /app
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

COPY Cargo.toml Cargo.lock ./
COPY backend/Cargo.toml ./backend/Cargo.toml
RUN mkdir -p backend/src && echo "fn main() {}" > backend/src/main.rs
RUN cargo build --release --manifest-path backend/Cargo.toml
RUN rm -rf backend/src

COPY backend/src ./backend/src
COPY backend/migrations ./backend/migrations
COPY backend/.sqlx ./backend/.sqlx
ENV SQLX_OFFLINE=true
RUN cargo build --release --manifest-path backend/Cargo.toml

FROM debian:bookworm-slim
WORKDIR /app
RUN apt-get update && apt-get install -y ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/backend /app/backend
COPY backend/migrations ./migrations
EXPOSE 3000
CMD ["./backend"]
