# Stage 1: Builder
FROM rust:1.94-slim AS builder
WORKDIR /usr/src/app

# Install system dependencies (needed for some crates)
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy the project files
COPY . .

# Build the application in release mode
RUN cargo build --release

# Stage 2: Runtime
FROM debian:bookworm-slim
WORKDIR /usr/local/bin

# Install runtime dependencies (OpenSSL is often required)
RUN apt-get update && apt-get install -y \
    openssl \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy the binary from the builder stage
COPY --from=builder /usr/src/app/target/release/guestbook-rust .
# Copy templates directory
COPY --from=builder /usr/src/app/templates /usr/local/bin/templates

# Expose the application port
EXPOSE 3000

# Run the binary
CMD ["./guestbook-rust"]
