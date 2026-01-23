# Multi-stage build for click-tracker
FROM rust:1.75-slim as builder

# Install system dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Set working directory
WORKDIR /app

# Copy workspace manifests
COPY Cargo.toml Cargo.lock ./

# Copy all workspace members (needed for workspace resolution)
COPY click-router/ ./click-router/
COPY click-router-api/ ./click-router-api/
COPY click-tracker/ ./click-tracker/
COPY click-aggregator/ ./click-aggregator/
COPY click-aggregator-api/ ./click-aggregator-api/
COPY infra/domains/ ./infra/domains/

# Build the application
RUN cargo build --release -p click-tracker

# Runtime stage
FROM debian:bookworm-slim

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
COPY --from=builder /app/target/release/click-tracker /app/
COPY --from=builder /app/click-tracker/config/ ./config/

# Change ownership
RUN chown -R shortas:shortas /app

# Switch to non-root user
USER shortas

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:8080/health || exit 1

# Run the application
CMD ["./click-tracker", "-r", "production"]
