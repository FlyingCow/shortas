# Multi-stage build for notification-worker
ARG RUST_VERSION=1.95
ARG DEBIAN_VERSION=trixie

FROM rust:${RUST_VERSION}-slim AS builder

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
RUN cargo build --release -p notification-worker

# Runtime stage
FROM debian:${DEBIAN_VERSION}-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Create non-root user
RUN useradd -r -s /bin/false shortas

# Set working directory
WORKDIR /app

# Copy binary from builder
COPY --from=builder /app/target/release/notification-worker /app/

# Copy config files
COPY workers/notification-worker/config/ /app/config/

# Change ownership
RUN chown -R shortas:shortas /app

# Switch to non-root user
USER shortas

# Run the application
CMD ["./notification-worker"]
