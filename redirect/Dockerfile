FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    curl \
    unzip \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Download fluvio CLI directly from GitHub releases
RUN mkdir -p /root/.fluvio/bin \
    && curl -L -o /tmp/fluvio.zip https://github.com/infinyon/fluvio/releases/latest/download/fluvio-x86_64-unknown-linux-musl.zip \
    && unzip /tmp/fluvio.zip -d /root/.fluvio/bin \
    && chmod +x /root/.fluvio/bin/fluvio \
    && rm /tmp/fluvio.zip

ENV PATH="/root/.fluvio/bin:${PATH}"

# Verify fluvio is installed
RUN fluvio version
