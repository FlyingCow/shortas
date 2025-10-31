---
layout: vector-theme
title: Getting Started
permalink: /getting-started/
---

<div class="hero-section">
  <h1>Getting Started with Shortas</h1>
  <p class="lead">Welcome to Shortas! This guide will help you get up and running quickly with our fast and scalable URL shortener. Follow these steps to have Shortas running locally in minutes.</p>
</div>

## 🚀 Quick Start

<div class="alert alert-info">
  <strong>One-Command Setup</strong> - Get up and running with Shortas in minutes using our automated setup process. This will install dependencies, start infrastructure, build services, and run tests.
</div>

### Prerequisites

<div class="card">
  <div class="card-header">System Requirements</div>
  <ul>
    <li><strong>Rust</strong> 1.75+ (stable channel)</li>
    <li><strong>Docker</strong> & Docker Compose</li>
    <li><strong>Make</strong> - GNU Make 4.0+</li>
    <li><strong>curl</strong> - for health checks</li>
    <li><strong>Git</strong> - for cloning the repository</li>
  </ul>
</div>

### One-Command Setup

```bash
# Clone the repository
git clone https://github.com/FlyingCow/shortas.git
cd shortas

# Complete development setup (installs deps, starts infra, builds, tests)
make dev-setup

# Start all services
make dev-start
```

<div class="alert alert-success">
  <strong>What this does:</strong>
  <ol>
    <li>Install system dependencies</li>
    <li>Start infrastructure (MongoDB, ClickHouse, Redis, Fluvio)</li>
    <li>Build all services</li>
    <li>Run tests</li>
    <li>Validate the setup</li>
  </ol>
</div>

## 📋 Manual Setup Steps

If you prefer to set up manually or need to troubleshoot, follow these steps:

### 1. Clone the Repository

```bash
git clone https://github.com/FlyingCow/shortas.git
cd shortas
```

### 2. Install Rust

Ensure you have Rust 1.75+ installed:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
rustup default stable
```

Verify installation:

```bash
rustc --version  # Should be 1.75 or higher
```

### 3. Install Docker & Docker Compose

For containerized infrastructure and services:

```bash
# Install Docker (Ubuntu/Debian)
sudo apt-get update
sudo apt-get install docker-ce docker-ce-cli containerd.io docker-buildx-plugin docker-compose-plugin

# Verify installation
docker --version
docker compose version
```

### 4. Start Infrastructure

Start MongoDB, ClickHouse, Redis, and Fluvio using Docker Compose:

```bash
make infra-start-custom
```

This will start all required infrastructure services.

### 5. Build Services

Build all Shortas microservices:

```bash
make build
```

This compiles all Rust services in debug mode. For production builds:

```bash
make build-release
```

### 6. Run Tests

Ensure everything is working correctly:

```bash
make test
```

This runs tests for all services.

### 7. Start Services

Start all Shortas services:

```bash
make dev-start
```

## 🏗️ Click Router Setup

### Prerequisites

- Rust 1.75+ (stable)
- MongoDB or DynamoDB
- Optional: Kafka/Fluvio for analytics

### Quick Start

```bash
# Clone the repository
git clone https://github.com/FlyingCow/shortas.git
cd shortas/redirect/click-router

# Build the project
make build-click-router

# Run tests
make test-click-router

# Start the server
cargo run --release
```

### Docker Deployment

```bash
# Build Docker image
docker build -t click-router .

# Run with Docker Compose
docker-compose up -d
```

## ⚙️ Configuration

Click Router uses environment-based configuration with TOML files:

### Environment Files

- `config/default.toml` - Base configuration
- `config/development.toml` - Development overrides
- `config/production.toml` - Production settings
- `config/test.toml` - Test configuration

### Key Configuration Sections

```toml
# Database Configuration
[mongodb]
uri = "mongodb://root:example@mongo:27017/"
database = "shortas"

# Caching Configuration
[moka]
[moka.routes_cache]
max_capacity = 10_000
time_to_live_minutes = 60

# Analytics Configuration
[fluvio]
[fluvio.hit_stream]
topic = "hit-stream-main"
host = "sc:9003"

# GeoIP Configuration
[geo_ip]
mmdb = "../data/geo-ip/GeoLite2-Country.mmdb"
```

## 🔧 Usage

### Basic Redirection

Click Router automatically handles URL redirection based on configured routes. Simply make HTTP requests to your shortened URLs:

```bash
curl -L https://your-domain.com/abc123
# Redirects to the configured destination
```

### Conditional Routing

Configure routes with conditional logic:

```json
{
  "switch": "main",
  "link": "example",
  "dest": "https://example.com",
  "policy": {
    "type": "conditional",
    "conditions": [
      {
        "key": "mobile",
        "condition": {
          "device": {"type": "mobile"}
        }
      }
    ]
  }
}
```

### Analytics Integration

Hits are automatically tracked and can be consumed via Kafka or Fluvio:

```rust
// Hit data structure
{
  "id": "01HZ...",
  "timestamp": "2024-01-01T00:00:00Z",
  "user_agent": "Mozilla/5.0...",
  "ip_address": "192.168.1.1",
  "route": {
    "switch": "main",
    "link": "example"
  }
}
```

## ✅ Verification

After setup, verify everything is working:

```bash
# Check health of all services
make health-check

# Check specific service
curl http://localhost:8080/health  # Click Router
curl http://localhost:8081/health  # Router API
curl http://localhost:8082/health  # Aggregator API
```

## 🐛 Troubleshooting

### Common Issues

<div class="card">
  <div class="card-header">Service Won't Start</div>
  <ul>
    <li>Check Docker is running: <code>docker ps</code></li>
    <li>Verify ports are not in use: <code>lsof -i :8080</code></li>
    <li>Check service logs: <code>make logs-router</code></li>
  </ul>
</div>

<div class="card">
  <div class="card-header">Build Errors</div>
  <ul>
    <li>Ensure Rust is up to date: <code>rustup update</code></li>
    <li>Clear build cache: <code>cargo clean</code></li>
    <li>Check dependencies: <code>cargo check</code></li>
  </ul>
</div>

<div class="card">
  <div class="card-header">Database Connection Issues</div>
  <ul>
    <li>Verify MongoDB is running: <code>docker ps | grep mongo</code></li>
    <li>Check connection string in config files</li>
    <li>Test connection: <code>mongosh mongodb://localhost:27017</code></li>
  </ul>
</div>

---

**Next Steps**: Once installed, proceed to the [Configuration Guide](/getting-started/) or explore the [Architecture Overview](/architecture/) to understand how Shortas works.
