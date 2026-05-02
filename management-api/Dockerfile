# Multi-stage build for management-api
ARG RUST_VERSION=1.87.0
ARG DEBIAN_VERSION=bookworm

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

# Copy management-api
COPY management-api/ ./management-api/

# Copy shortas-common dependency
COPY redirect/shortas-common/ ./redirect/shortas-common/

# Build the application
WORKDIR /app/management-api
RUN cargo build --release

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

# Copy binary from builder
COPY --from=builder /app/management-api/target/release/management-api /app/

# Change ownership
RUN chown -R shortas:shortas /app

# Switch to non-root user
USER shortas

# Expose port
EXPOSE 8080

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:8080/health || exit 1

# Run the application
CMD ["./management-api"]
