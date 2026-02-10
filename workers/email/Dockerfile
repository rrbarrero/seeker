# Build stage
FROM rust:1.93.0-slim-trixie AS builder

WORKDIR /usr/src/app

# Install build dependencies
RUN apt-get update && apt-get install -y pkg-config libssl-dev

# 1. Pre-build dependencies to cache them
# Copy only the dependency definitions
COPY Cargo.toml Cargo.lock ./

# Create a dummy main.rs to allow cargo to build dependencies
RUN mkdir src && \
    echo "fn main() {println!(\"dummy\")}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

# 2. Build the actual application
# Copy the source code and other necessary files
COPY . .

# Touch the main file to ensure it gets rebuilt
RUN touch src/main.rs

# Build the application
# We use SQLX_OFFLINE=true to avoid needing a DB connection during build
ENV SQLX_OFFLINE=true
RUN cargo build --release

# Runtime stage
FROM debian:13.3-slim

WORKDIR /usr/local/bin

# Install runtime dependencies
RUN apt-get update && apt-get install -y libssl3 ca-certificates && rm -rf /var/lib/apt/lists/*

# Copy the binary from the builder stage
COPY --from=builder /usr/src/app/target/release/email-worker .

# Set the default command
CMD ["./email-worker"]
