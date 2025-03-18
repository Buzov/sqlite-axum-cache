FROM rust:1.78 as builder
LABEL authors="Artur Buzov"

RUN apt-get update && apt-get install -y musl-tools

WORKDIR /app
COPY . .

# Set Rust to use musl
RUN rustup target add x86_64-unknown-linux-musl

# Build the Rust application using musl
RUN cargo build --release --target x86_64-unknown-linux-musl

# Minimal Final Runtime Image (No `glibc` required!)
FROM alpine:latest
WORKDIR /app
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/sqlite-axum-cache .
#COPY --from=builder /app/.env .env
EXPOSE 3000

ENTRYPOINT ["/app/sqlite-axum-cache"]