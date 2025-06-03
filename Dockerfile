# syntax=docker/dockerfile:1
FROM rust:1.82 AS builder

WORKDIR /app
COPY . .
# Add SQLx query cache
COPY .sqlx .sqlx
ENV SQLX_OFFLINE=true
# Install required dependencies (for example, openssl for diesel)
RUN apt-get update && apt-get install -y pkg-config libssl-dev

# Create a dummy build to cache dependencies
RUN cargo build --release

# Final image — switch to Bookworm for OpenSSL 3 support
FROM debian:bookworm-slim

WORKDIR /app
COPY --from=builder /app/target/release/voting_system .
COPY migrations ./migrations

# Install OpenSSL 3 runtime (libssl.so.3) and PostgreSQL client lib
RUN apt-get update && apt-get install -y libssl3 libpq-dev ca-certificates

CMD ["./voting_system"]
