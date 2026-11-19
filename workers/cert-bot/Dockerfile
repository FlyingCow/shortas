# Multi-stage build for cert-bot
ARG RUST_VERSION=1.95
ARG DEBIAN_VERSION=trixie

FROM rust:${RUST_VERSION}-slim as builder

# Install system dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    ca-certificates \
    g++ \
    make \
    cmake \
    && rm -rf /var/lib/apt/lists/*

# Set working directory
WORKDIR /app

# Copy the root workspace (cargo needs every member's manifest present to
# resolve the workspace, even though only one crate is built here)
COPY Cargo.toml Cargo.lock ./
COPY api/ ./api/
COPY workers/ ./workers/
COPY redirect/shortas-common/ ./redirect/shortas-common/

# Build the application
RUN cargo build --release -p cert-bot

# Runtime stage
FROM debian:${DEBIAN_VERSION}-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Create non-root user
RUN useradd -r -s /bin/false shortas

# Set working directory
WORKDIR /app

# Copy binary and config from builder
COPY --from=builder /app/target/release/cert-bot /app/
COPY --from=builder /app/workers/cert-bot/config/ ./config/

# Create directory for account key
RUN mkdir -p /etc/cert-bot && chown -R shortas:shortas /etc/cert-bot

# Change ownership
RUN chown -R shortas:shortas /app

# Switch to non-root user
USER shortas

# Run the application
CMD ["./cert-bot", "-r", "production"]
