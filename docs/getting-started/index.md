---
layout: page
title: Getting Started
permalink: /getting-started/
---

# Getting Started with Shortas

Welcome to Shortas! This guide will help you get up and running quickly with our fast and scalable URL shortener.

## 🚀 Quick Start

### Prerequisites

- Rust 1.75+ (stable)
- Docker & Docker Compose
- Make
- curl (for health checks)

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

This will:
1. Install system dependencies
2. Start infrastructure (MongoDB, ClickHouse, Redis, Fluvio)
3. Build all services
4. Run tests
5. Validate the setup

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
