# Build stage
FROM rust:1.93.0-slim-trixie AS builder

WORKDIR /usr/src/app

# Install build dependencies
RUN apt-get update && apt-get install -y pkg-config libssl-dev

# Install sqlx-cli
RUN cargo install sqlx-cli --no-default-features --features postgres && \
    rustup component add rustfmt clippy

# Copy the source code
COPY . .

# Build the application
ENV SQLX_OFFLINE=true
RUN cargo build --release

# Runtime stage
FROM debian:13.3-slim

WORKDIR /usr/local/bin

# Install runtime dependencies
RUN apt-get update && apt-get install -y libssl3 ca-certificates && rm -rf /var/lib/apt/lists/*

# Copy the binary from the builder stage
COPY --from=builder /usr/src/app/target/release/best-seeker .

# Set the default command
CMD ["./best-seeker"]
