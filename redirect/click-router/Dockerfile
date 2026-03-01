# syntax=docker/dockerfile:1.4

# ==============================================================================
# Click Router - High-performance redirect service
# ==============================================================================

ARG RUST_VERSION=1.88
ARG DEBIAN_VERSION=bookworm

# ------------------------------------------------------------------------------
# Stage 1: Chef - Dependency caching layer
# ------------------------------------------------------------------------------
FROM rust:${RUST_VERSION}-slim AS chef

RUN cargo install cargo-chef --locked
WORKDIR /app

# ------------------------------------------------------------------------------
# Stage 2: Planner - Analyze dependencies
# ------------------------------------------------------------------------------
FROM chef AS planner

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
COPY redirect/infra/domains/ ./redirect/infra/domains/
COPY salvo/ ./salvo/

WORKDIR /app/redirect
RUN cargo chef prepare --recipe-path recipe.json

# ------------------------------------------------------------------------------
# Stage 3: Builder - Compile dependencies and application
# ------------------------------------------------------------------------------
FROM chef AS builder

# Install build dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config \
    libssl-dev \
    ca-certificates \
    g++ \
    make \
    cmake \
    && rm -rf /var/lib/apt/lists/*

# Copy workspace structure for dependency resolution
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
COPY redirect/infra/domains/ ./redirect/infra/domains/
COPY salvo/ ./salvo/

# Build dependencies (cached layer)
WORKDIR /app/redirect
COPY --from=planner /app/redirect/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json -p click-router

# Copy source and build application
COPY redirect/data/ ./data/
RUN cargo build --release -p click-router

# ------------------------------------------------------------------------------
# Stage 4: Runtime - Minimal production image
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

EXPOSE 5800

HEALTHCHECK --interval=30s --timeout=5s --start-period=10s --retries=3 \
    CMD curl -sf http://localhost:5800/health || exit 1

ENTRYPOINT ["./click-router"]
CMD ["-r", "production"]
