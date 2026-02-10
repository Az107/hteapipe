FROM rust:latest AS builder

WORKDIR /app

COPY . .

RUN cargo build --release

FROM debian:bookworm-slim

COPY --from=builder /app/target/release/hteapipe /usr/local/bin/hteapipe

RUN chmod +x /usr/local/bin/hteapipe

ENTRYPOINT ["/usr/local/bin/hteapipe", "8080"]
