# Build stage
FROM rust:1.80-slim as builder

WORKDIR /app
COPY . .

RUN cargo build --release

# Final stage
FROM debian:bookworm-slim

WORKDIR /app
COPY --from=builder /app/target/release/rustcrew /app/rustcrew

# Install CA certificates for https requests
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

EXPOSE 8080

CMD ["./rustcrew"]
