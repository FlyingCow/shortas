# Multi-stage build for click-router-api
ARG RUST_VERSION=1.94.0
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

# Copy workspace manifests from redirect repo
COPY redirect/Cargo.toml redirect/Cargo.lock ./redirect/

# Copy all workspace members (needed for workspace resolution)
COPY redirect/cert-bot/ ./redirect/cert-bot/
COPY redirect/click-router/ ./redirect/click-router/
COPY redirect/click-router-api/ ./redirect/click-router-api/
COPY redirect/click-tracker/ ./redirect/click-tracker/
COPY redirect/click-aggregator/ ./redirect/click-aggregator/
COPY redirect/click-aggregator-api/ ./redirect/click-aggregator-api/
COPY redirect/domain-verifier/ ./redirect/domain-verifier/
COPY redirect/route-icon-worker/ ./redirect/route-icon-worker/
COPY redirect/route-verifier/ ./redirect/route-verifier/

# Copy external salvo dependency (needed for workspace resolution)
COPY salvo/ ./salvo/

# Build the application from the redirect workspace
WORKDIR /app/redirect
RUN cargo build --release -p click-router-api

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
COPY --from=builder /app/redirect/target/release/click-router-api /app/
COPY --from=builder /app/redirect/click-router-api/config/ ./config/

# Change ownership
RUN chown -R shortas:shortas /app

# Switch to non-root user
USER shortas

# Expose port
EXPOSE 5810

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:5810/health || exit 1

# Run the application
CMD ["./click-router-api", "-r", "production"]
