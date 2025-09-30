---
layout: page
title: Getting Started
permalink: /getting-started/
---

<div class="hero-section">
  <h1>Getting Started with Shortas</h1>
  <p class="lead">Welcome to Shortas! This guide will help you get up and running quickly with our fast and scalable URL shortener.</p>
</div>

## 🚀 Quick Start

<div class="alert alert-info">
  <strong>One-Command Setup</strong> - Get up and running with Shortas in minutes using our automated setup process.
</div>

### Prerequisites

<div class="card">
  <div class="card-header">System Requirements</div>
  <ul>
    <li><strong>Rust</strong> 1.75+ (stable)</li>
    <li><strong>Docker</strong> & Docker Compose</li>
    <li><strong>Make</strong> - GNU Make 4.0+</li>
    <li><strong>curl</strong> - for health checks</li>
  </ul>
</div>

### One-Command Setup

```bash
# Clone the repository
git clone https://github.com/FlyingCow/shortas.git
cd shortas

# Complete development setup (installs dependencies, starts infrastructure, builds services, runs tests)
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

If you prefer to set up manually or need to troubleshoot:

### 1. Validate System

```bash
make validate
```

### 2. Start Infrastructure

```bash
# Custom infrastructure (MongoDB, ClickHouse, Redis, Fluvio)
make infra-start-custom

# OR AWS LocalStack infrastructure
make infra-start-aws
```

### 3. Build All Services

```bash
make build
```

### 4. Run Tests

```bash
make test
```

### 5. Check Health

```bash
make health-check
```

## 🐳 Docker Deployment

### Complete Docker Setup

```bash
# Build all Docker images
make build-docker

# Deploy with Docker Compose
make deploy-docker

# Check health
make health-check
```

## 🔧 Service Management

### Development Services

```bash
make dev-start    # Start all services
make dev-stop     # Stop all services
make dev-restart  # Restart all services
```

### View Logs

```bash
make logs                    # All services
make logs-router            # Click Router only
make logs-tracker           # Click Tracker only
make logs-aggregator        # Click Aggregator only
```

## 🌐 Service Ports

- **Click Router**: 8080
- **Click Router API**: 8080
- **Click Aggregator API**: 8080
- **MongoDB**: 27017
- **ClickHouse**: 8123
- **Redis**: 6379
- **Kafka**: 9092

## 📊 Health Checks

All services provide health check endpoints:

```bash
# Check all services
curl http://localhost:8080/health

# Check specific service
curl http://localhost:8080/health
curl http://localhost:8081/health  # Router API
curl http://localhost:8082/health  # Aggregator API
```

## 🔍 Next Steps

Now that you have Shortas running, explore these guides:

- [Installation Guide](installation/) - Detailed installation instructions
- [Configuration](configuration/) - Configure your deployment
- [First Steps](first-steps/) - Create your first shortened URL
- [Architecture Overview](../architecture/) - Understand the system architecture
- [API Reference](../api/) - Learn about the APIs

## 🆘 Troubleshooting

### Common Issues

**Port conflicts:**
```bash
# Check what's using a port
sudo netstat -tlnp | grep 8080

# Kill process using port
sudo kill -9 $(sudo lsof -t -i:8080)
```

**Database connection issues:**
```bash
# Test MongoDB connection
mongosh "mongodb://localhost:27017/shortas"

# Test ClickHouse connection
curl http://localhost:8123/ping
```

**Service not starting:**
```bash
# Check logs
make logs

# Restart services
make dev-restart
```

### Getting Help

- Check the [troubleshooting section](../deployment/troubleshooting/)
- Review service logs: `make logs`
- Open an issue on [GitHub](https://github.com/FlyingCow/shortas/issues)

---

**Ready to dive deeper?** Check out our [Architecture Overview](../architecture/) or [API Documentation](../api/).
