# Shortas

<div align="center">

**A high-performance, enterprise-grade URL shortener built with Rust**

[![Rust](https://img.shields.io/badge/rust-1.75+-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Documentation](https://img.shields.io/badge/docs-latest-2563eb.svg)](https://docs.shortas.com)

[Getting Started](#-quick-start) • [Documentation](https://docs.shortas.com) • [Architecture](#-architecture) • [Contributing](#-contributing)

</div>

---

## 🚀 Quick Start

Get Shortas up and running in minutes with our one-command setup:

```bash
# Clone the repository
git clone https://github.com/FlyingCow/shortas.git
cd shortas

# Complete development setup (installs dependencies, starts infrastructure, builds services)
make dev-setup

# Start all services
make dev-start
```

**What you get:**
- ✅ Full microservices stack running locally
- ✅ MongoDB, ClickHouse, Redis, and Kafka configured
- ✅ All services built and tested
- ✅ Health checks validated

---

## 🏗️ Architecture

Shortas is built as a **microservices architecture** with five core components, each optimized for specific responsibilities:

<table>
<tr>
<td width="50%">

### **Click Router** 🚀
High-performance URL redirection service with:
- Conditional routing (device, geo, time-based)
- Multiple redirect types (301, 302, proxy, retargeting)
- SSL certificate management
- Multi-database support (MongoDB/DynamoDB)
- Advanced caching (Redis/Moka)

**Performance:** 360,000+ req/s (CPU), 2.6-2.8 µs latency

</td>
<td width="50%">

### **Click Tracker** 📊
Real-time click processing with:
- Bot detection
- Geographic analytics
- Device & browser tracking
- Unique visitor tracking
- Session analysis

**Performance:** 1.07M events/s (CPU), 927 ns latency, 7,800/s with I/O

</td>
</tr>
<tr>
<td width="50%">

### **Click Aggregator** ⚡
Analytics data processing with:
- High-throughput batch processing
- OLAP-optimized storage
- Scalable data ingestion
- Real-time aggregations

**Performance:** 1.05M records/s (CPU), ~950 ns latency

</td>
<td width="50%">

### **APIs** 🔧
Management and analytics APIs:
- **Click Router API** - Route & settings management
- **Click Aggregator API** - Analytics & reporting
- JWT authentication (Keycloak)
- OpenAPI documentation
- Role-based access control

**Performance:** 5,000+ req/s

</td>
</tr>
</table>

---

## ✨ Key Features

### 🔄 Advanced Routing & Redirects

- **Multiple Redirect Types**: 301, 302, proxy, retargeting
- **Conditional Logic**: Route based on device, location, time, or custom expressions
- **Domain-Based Routing**: Wildcard support for multi-tenant setups
- **SSL Management**: Automated certificate handling
- **A/B Testing**: Built-in traffic splitting capabilities

### 📊 Comprehensive Analytics

- **Real-Time Tracking**: Sub-millisecond click tracking
- **Geographic Analytics**: Country, continent, and location-based insights
- **Device Analytics**: Browser, OS, and device type tracking
- **Session Tracking**: User behavior and flow analysis
- **Bot Detection**: Automatic filtering of bot traffic
- **Unique Visitors**: Accurate visitor counting

### 🏢 Enterprise Multi-Tenancy

- **Workspace Isolation**: Complete data segregation per workspace
- **Role Management**: User and creator role hierarchies
- **Owner-Based Access**: Fine-grained permission control
- **Custom Settings**: Per-workspace configuration

---

## 🛠️ Technology Stack

<div align="center">

| Category | Technology |
|----------|-----------|
| **Language** | Rust 1.75+ |
| **Web Framework** | Salvo |
| **Databases** | MongoDB, ClickHouse, Redis, AWS DynamoDB |
| **Message Queues** | Apache Kafka, Fluvio |
| **Infrastructure** | Docker, Terraform, AWS |
| **Analytics** | ClickHouse (OLAP) |
| **Caching** | Redis, Moka |

</div>

---

## 📊 Performance Metrics

<div align="center">

| Service | Throughput | Latency | Notes |
|---------|-----------|---------|-------|
| **Click Router** | 360,000+ req/s | 2.6-2.8 µs | CPU-only (warm cache) |
| **Click Tracker** | 1.07M events/s | 927 ns | CPU-only, 7,800/s with I/O (8 workers) |
| **Click Aggregator** | 1.05M records/s | ~950 ns | CPU-only (conversions) |
| **APIs** | 5,000+ req/s | <5ms p95 | Production estimates |

</div>

> **Note:** CPU-only benchmarks measure pure processing speed. Real-world throughput with I/O (Redis, Kafka, ClickHouse) varies by workload but typically achieves 4,000-10,000 events/sec per worker.

---

## 📚 Documentation

### Quick Links

- **[Getting Started](docs/getting-started/)** - Installation and setup guide
- **[Architecture](docs/architecture/)** - System architecture overview
- **[API Reference](docs/api/)** - Complete API documentation
- **[Deployment](docs/deployment/)** - Deployment strategies
- **[Development](docs/development/)** - Contributing guidelines

### Component Documentation

- **[Click Router](redirect/click-router/README.md)** - Main redirect service
- **[Click Router API](redirect/click-router-api/README.md)** - Route management API
- **[Click Aggregator API](redirect/click-aggregator-api/README.md)** - Analytics API

---

## 🚀 Getting Started

### Prerequisites

- **Rust** 1.75+ (stable channel)
- **Docker** & Docker Compose
- **Make** (GNU Make 4.0+)
- **curl** (for health checks)

### One-Command Setup

```bash
# Complete development setup
make dev-setup

# Start all services
make dev-start

# Check health
make health-check
```

### Manual Setup

If you prefer step-by-step setup or need to troubleshoot:

```bash
# Start infrastructure (MongoDB, ClickHouse, Redis, Kafka)
make infra-start-custom

# Build all services
make build

# Run tests
make test

# Start services
make dev-start
```

---

## 🐳 Docker Deployment

```bash
# Build all Docker images
make build-docker

# Deploy with Docker Compose
make deploy-docker

# Check health
make health-check
```

---

## 🔧 Service Management

```bash
# Development services
make dev-start    # Start all services
make dev-stop     # Stop all services
make dev-restart  # Restart all services

# View logs
make logs                    # All services
make logs-router            # Click Router only
make logs-tracker           # Click Tracker only
make logs-aggregator        # Click Aggregator only
```

---

## 🌐 Service Ports

| Service | Port | Description |
|---------|------|-------------|
| **Click Router** | 8080 | Main redirect service |
| **Click Router API** | 8081 | Route management API |
| **Click Aggregator API** | 8082 | Analytics API |
| **MongoDB** | 27017 | Primary database |
| **ClickHouse** | 8123 | Analytics database |
| **Redis** | 6379 | Cache |
| **Kafka** | 9092 | Message queue |

---

## 🧪 Testing

```bash
# Run all tests
make test

# Run tests for specific service
make test-click-router
make test-click-tracker
make test-click-aggregator

# Run tests with coverage
make test-coverage
```

---

## 🔍 Health Checks

All services provide health check endpoints:

```bash
# Check all services
make health-check

# Check specific service
curl http://localhost:8080/health
curl http://localhost:8081/health  # Router API
curl http://localhost:8082/health  # Aggregator API
```

---

## 🤝 Contributing

We welcome contributions! Here's how you can help:

### Ways to Contribute

- **Code**: Bug fixes, new features, performance improvements
- **Documentation**: Improve guides, add examples, fix typos
- **Testing**: Write tests, report bugs, improve test coverage
- **Community**: Help others, answer questions, share knowledge

### Getting Started

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Run tests (`make test`)
5. Commit your changes (`git commit -m 'feat: Add amazing feature'`)
6. Push to the branch (`git push origin feature/amazing-feature`)
7. Open a Pull Request

See our [Contributing Guide](docs/development/) for detailed guidelines.

---

## 📄 License

This project is licensed under the **MIT License** - see the [LICENSE](LICENSE) file for details.

---

## 🔗 Links

- **GitHub Repository**: [https://github.com/FlyingCow/shortas](https://github.com/FlyingCow/shortas)
- **Issue Tracker**: [https://github.com/FlyingCow/shortas/issues](https://github.com/FlyingCow/shortas/issues)
- **Documentation**: [https://docs.shortas.com](https://docs.shortas.com)
- **Contributing Guide**: [docs/development/](docs/development/)

---

## 🆘 Support

For support and questions:

- **Documentation**: Check our comprehensive [documentation](docs/)
- **Issues**: Report bugs and feature requests via [GitHub Issues](https://github.com/FlyingCow/shortas/issues)
- **Security**: Report security issues privately to security@shortas.com
- **Community**: Join our community discussions

---

<div align="center">

**Built with ❤️ using Rust and modern web technologies**

[⭐ Star us on GitHub](https://github.com/FlyingCow/shortas) • [📖 Read the Docs](https://docs.shortas.com) • [🐛 Report a Bug](https://github.com/FlyingCow/shortas/issues)

</div>
