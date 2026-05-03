# syntax=docker/dockerfile:1.4

# ==============================================================================
# Click Router - High-performance redirect service
# ==============================================================================

ARG RUST_VERSION=1.95
ARG DEBIAN_VERSION=trixie

# ------------------------------------------------------------------------------
# Stage 1: Builder - Compile application
# ------------------------------------------------------------------------------
FROM rust:${RUST_VERSION}-slim AS builder

# Install build dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config \
    libssl-dev \
    ca-certificates \
    g++ \
    make \
    cmake \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy workspace structure
COPY redirect/Cargo.toml redirect/Cargo.lock ./redirect/
COPY redirect/cert-bot/ ./redirect/cert-bot/
COPY redirect/click-router/ ./redirect/click-router/
COPY redirect/click-router-api/ ./redirect/click-router-api/
COPY redirect/click-tracker/ ./redirect/click-tracker/
COPY redirect/click-aggregator/ ./redirect/click-aggregator/
COPY redirect/click-aggregator-api/ ./redirect/click-aggregator-api/
COPY redirect/domain-verifier/ ./redirect/domain-verifier/
COPY redirect/route-icon-worker/ ./redirect/route-icon-worker/
COPY redirect/route-verifier/ ./redirect/route-verifier/
COPY redirect/shortas-common/ ./redirect/shortas-common/
COPY salvo/ ./salvo/
COPY redirect/data/ ./redirect/data/

# Build application
WORKDIR /app/redirect
RUN cargo build --release -p click-router

# ------------------------------------------------------------------------------
# Stage 2: Runtime - Minimal production image
# ------------------------------------------------------------------------------
FROM debian:${DEBIAN_VERSION}-slim AS runtime

LABEL org.opencontainers.image.title="Click Router" \
      org.opencontainers.image.description="High-performance redirect and routing service" \
      org.opencontainers.image.vendor="Shortas" \
      org.opencontainers.image.source="https://github.com/FlyingCow/shortas"

# Install runtime dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    curl \
    && rm -rf /var/lib/apt/lists/* \
    && useradd --system --no-create-home --shell /usr/sbin/nologin shortas

WORKDIR /app

# Copy artifacts from builder
COPY --from=builder --chown=shortas:shortas /app/redirect/target/release/click-router ./
COPY --from=builder --chown=shortas:shortas /app/redirect/click-router/config/ ./config/
COPY --from=builder --chown=shortas:shortas /app/redirect/data/ ./data/

USER shortas

EXPOSE 5800 4433

HEALTHCHECK --interval=30s --timeout=5s --start-period=10s --retries=3 \
    CMD curl -sf http://localhost:5800/health || exit 1

ENTRYPOINT ["./click-router"]
CMD ["-r", "production"]
