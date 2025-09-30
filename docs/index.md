---
layout: home
title: Shortas Documentation
description: Comprehensive documentation for Shortas - a fast and scalable URL shortener with advanced analytics, multi-tenancy, and real-time click tracking capabilities.
---

# Welcome to Shortas Documentation

**Shortas** is a fast and scalable URL shortener built with Rust, featuring advanced analytics, multi-tenancy, and real-time click tracking capabilities.

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

## 🏗️ Architecture Overview

Shortas is built as a microservices architecture with five main components:

- **[Click Router](redirect/click-router/README.md)** - Main redirect service handling URL routing and redirects
- **[Click Tracker](redirect/click-tracker/)** - Real-time click processing and data enrichment  
- **[Click Aggregator](redirect/click-aggregator/)** - Analytics data processing and storage
- **[Click Router API](redirect/click-router-api/README.md)** - REST API for route and settings management
- **[Click Aggregator API](redirect/click-aggregator-api/README.md)** - Analytics and reporting API

## 📚 Documentation Sections

### Getting Started
- [Quick Start Guide](getting-started/)
- [Installation](getting-started/installation/)
- [Configuration](getting-started/configuration/)
- [First Steps](getting-started/first-steps/)

### Architecture
- [System Overview](architecture/)
- [Microservices](architecture/microservices/)
- [Data Flow](architecture/data-flow/)
- [Security](architecture/security/)

### API Reference
- [Click Router API](api/click-router/)
- [Click Aggregator API](api/click-aggregator/)
- [Authentication](api/authentication/)
- [Data Models](api/data-models/)

### Deployment
- [Local Development](deployment/local/)
- [Docker Deployment](deployment/docker/)
- [Kubernetes](deployment/kubernetes/)
- [AWS Production](deployment/aws/)

### Development
- [Contributing](development/contributing/)
- [Code Style](development/code-style/)
- [Testing](development/testing/)
- [Debugging](development/debugging/)

## 🛠️ Technology Stack

- **Language**: Rust (all components)
- **Web Frameworks**: Salvo
- **Databases**: MongoDB, ClickHouse, Redis, AWS DynamoDB
- **Message Queues**: Apache Kafka, Fluvio
- **Infrastructure**: Docker, Terraform, AWS
- **Analytics**: ClickHouse for OLAP queries
- **Caching**: Redis for session and route caching

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

## 📊 Performance

- **Click Router**: 10,000+ requests/second
- **Click Tracker**: 50,000+ events/second
- **Click Aggregator**: 100,000+ records/second
- **APIs**: 5,000+ requests/second

## 🔗 Quick Links

- [GitHub Repository](https://github.com/FlyingCow/shortas)
- [Issue Tracker](https://github.com/FlyingCow/shortas/issues)
- [Contributing Guide](development/contributing/)
- [API Documentation](api/)

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](https://github.com/FlyingCow/shortas/blob/main/LICENSE) file for details.

---

**Need help?** Check out our [Getting Started Guide](getting-started/) or [open an issue](https://github.com/FlyingCow/shortas/issues) on GitHub.
