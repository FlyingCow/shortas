# Shortas

**Shortas** is a fast and scalable URL shortener built with Rust, featuring advanced analytics, multi-tenancy, and real-time click tracking capabilities.

[![Rust](https://img.shields.io/badge/rust-1.75+-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Documentation](https://img.shields.io/badge/docs-latest-green.svg)](https://docs.shortas.com)

## 🚀 Quick Start

Get up and running with Shortas in minutes:

```bash
# Clone the repository
git clone https://github.com/FlyingCow/shortas.git
cd shortas

# Complete development setup
make dev-setup

# Start all services
make dev-start
```

## 🏗️ Architecture

Shortas is built as a microservices architecture with five main components:

- **[Click Router](redirect/click-router/README.md)** - Main redirect service handling URL routing and redirects
- **[Click Tracker](redirect/click-tracker/)** - Real-time click processing and data enrichment  
- **[Click Aggregator](redirect/click-aggregator/)** - Analytics data processing and storage
- **[Click Router API](redirect/click-router-api/README.md)** - REST API for route and settings management
- **[Click Aggregator API](redirect/click-aggregator-api/README.md)** - Analytics and reporting API

## 🚀 Key Features

### Routing & Redirects
- Multiple redirect types (301, 302, proxy, retargeting)
- Domain-based routing with wildcard support
- SSL certificate management
- Deep link support
- A/B testing capabilities

### Analytics & Tracking
- Real-time click tracking
- Geographic analytics (country, continent, location)
- Device and browser analytics
- Session tracking and user behavior
- Bot detection and filtering
- Unique visitor tracking

### Multi-tenancy
- Workspace-based isolation
- User and creator role management
- Owner-based data segregation
- Custom user settings per workspace

## 🛠️ Technology Stack

- **Language**: Rust (all components)
- **Web Frameworks**: Salvo
- **Databases**: MongoDB, ClickHouse, Redis, AWS DynamoDB
- **Message Queues**: Apache Kafka, Fluvio
- **Infrastructure**: Docker, Terraform, AWS
- **Analytics**: ClickHouse for OLAP queries
- **Caching**: Redis for session and route caching

## 📊 Performance

- **Click Router**: 10,000+ requests/second
- **Click Tracker**: 50,000+ events/second
- **Click Aggregator**: 100,000+ records/second
- **APIs**: 5,000+ requests/second

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

## 🚀 Getting Started

### Prerequisites

- Rust 1.75+ (stable)
- Docker & Docker Compose
- Make
- curl (for health checks)

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

```bash
# Start infrastructure
make infra-start-custom

# Build all services
make build

# Run tests
make test
```

## 🐳 Docker Deployment

```bash
# Build all Docker images
make build-docker

# Deploy with Docker Compose
make deploy-docker

# Check health
make health-check
```

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

## 🌐 Service Ports

- **Click Router**: 8080
- **Click Router API**: 8081
- **Click Aggregator API**: 8082
- **MongoDB**: 27017
- **ClickHouse**: 8123
- **Redis**: 6379
- **Kafka**: 9092

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

## 🤝 Contributing

We welcome contributions from the community! Please see our [Contributing Guide](docs/development/) for details.

### Ways to Contribute
- **Code**: Bug fixes, new features, performance improvements
- **Documentation**: Improve guides, add examples, fix typos
- **Testing**: Write tests, report bugs, improve test coverage
- **Community**: Help others, answer questions, share knowledge

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🔗 Links

- **GitHub Repository**: [https://github.com/FlyingCow/shortas](https://github.com/FlyingCow/shortas)
- **Issue Tracker**: [https://github.com/FlyingCow/shortas/issues](https://github.com/FlyingCow/shortas/issues)
- **Documentation**: [https://docs.shortas.com](https://docs.shortas.com)
- **Contributing Guide**: [docs/development/](docs/development/)

## 🆘 Support

For support and questions:

- **Documentation**: Check our comprehensive [documentation](docs/)
- **Issues**: Report bugs and feature requests via [GitHub Issues](https://github.com/FlyingCow/shortas/issues)
- **Security**: Report security issues privately to security@shortas.com
- **Community**: Join our community discussions

---

**Built with ❤️ using Rust and modern web technologies**
