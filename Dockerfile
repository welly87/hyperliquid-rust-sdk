# Build stage
FROM rust:1.70-slim as builder

WORKDIR /usr/src/hyperliquid-nats

# Install required system dependencies
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy the source code
COPY . .

# Build the application
RUN cargo build --release --bin nats_service

# Runtime stage
FROM debian:bullseye-slim

# Install runtime dependencies
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy the binary from the builder stage
COPY --from=builder /usr/src/hyperliquid-nats/target/release/nats_service /usr/local/bin/

# Set environment variables with defaults
ENV NATS_URL=nats://localhost:4222
ENV NATS_SUBJECT=hyperliquid.orders
ENV HYPERLIQUID_API_URL=https://api.hyperliquid.xyz

# Run the service
CMD ["nats_service"]
