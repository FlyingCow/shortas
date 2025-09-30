# Shortas Redirect System

A comprehensive Rust-based URL shortening and redirect system with advanced analytics, multi-tenancy, and real-time click tracking capabilities.

## 🏗️ Architecture

The system is built as a microservices architecture with five main components:

### Core Services

- **Click Router** - Main redirect service handling URL routing and redirects
- **Click Tracker** - Real-time click processing and data enrichment
- **Click Aggregator** - Analytics data processing and storage
- **Click Router API** - REST API for route and settings management
- **Click Aggregator API** - Analytics and reporting API

### Data Flow

```
Incoming Request → Click Router → Click Tracker → Click Aggregator → Analytics Storage
                      ↓              ↓              ↓
                   Redirect      Enrichment    ClickHouse
```

## 🚀 Features

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
- **Web Frameworks**: Salvo, Actix Web
- **Databases**: MongoDB, ClickHouse, Redis, AWS DynamoDB
- **Message Queues**: Apache Kafka, Fluvio
- **Infrastructure**: Docker, Terraform, AWS
- **Analytics**: ClickHouse for OLAP queries
- **Caching**: Redis for session and route caching

## 📦 Infrastructure Components

### Databases
- **MongoDB** - Routes, user settings, certificates
- **ClickHouse** - Analytics and click stream data
- **Redis** - Caching and session storage

### Message Streaming
- **Apache Kafka** - Click stream processing
- **Fluvio** - Alternative streaming platform

### Cloud Services
- **AWS DynamoDB** - Alternative data storage
- **AWS S3** - File storage

## 🚀 Quick Start

### Prerequisites
- Rust 1.75+
- Docker & Docker Compose
- Make
- curl (for health checks)

### Complete Development Setup

**One-command setup:**
```bash
make dev-setup
```

This will:
1. Install system dependencies
2. Start infrastructure (MongoDB, ClickHouse, Redis, Fluvio)
3. Build all services
4. Run tests
5. Validate the setup

### Manual Setup Steps

1. **Validate System**
   ```bash
   make validate
   ```

2. **Start Infrastructure**
   ```bash
   # Custom infrastructure (MongoDB, ClickHouse, Redis, Fluvio)
   make infra-start-custom
   
   # OR AWS LocalStack infrastructure
   make infra-start-aws
   ```

3. **Build All Services**
   ```bash
   make build
   ```

4. **Run Tests**
   ```bash
   make test
   ```

5. **Check Health**
   ```bash
   make health-check
   ```

### Docker Deployment

**Complete Docker setup:**
```bash
# Build all Docker images
make build-docker

# Deploy with Docker Compose
make deploy-docker

# Check health
make health-check
```

### Service Management

**Start/Stop Development Services:**
```bash
make dev-start    # Start all services
make dev-stop     # Stop all services
make dev-restart  # Restart all services
```

**View Logs:**
```bash
make logs                    # All services
make logs-router            # Click Router only
make logs-tracker           # Click Tracker only
make logs-aggregator        # Click Aggregator only
```

### Manual Setup (if needed)

Add to `/etc/hosts`:
```
127.0.0.1 kafka
127.0.0.1 clickhouse
127.0.0.1 mongo
127.0.0.1 cache
```

## 📊 Monitoring & Debugging

### Kafka Console Consumer
```bash
~/dev/kafka/bin$ ./kafka-console-consumer.sh --bootstrap-server kafka:9092 --topic hit-stream-local --property "print.key=true"
```

### Service Ports
- **Click Router**: 8080
- **Click Router API**: 8080
- **Click Aggregator API**: 8080
- **MongoDB**: 27017
- **ClickHouse**: 8123
- **Redis**: 6379
- **Kafka**: 9092

## 🏗️ Deployment

### Local Development
- Docker Compose setup with all services
- Automatic database initialization
- Test data seeding

### AWS Production
- Terraform-based infrastructure
- AWS services integration
- Scalable deployment

### Custom Infrastructure
- Flexible database backends
- Multiple streaming options
- Configurable adapters

## 📋 TODO

### Authentication
- [ ] JWT authorization for API

### Click Router
- [ ] SSL support (key storage, ACME http verification)
- [ ] Deep links
- [ ] API to create/modify/delete links/settings data

### Tracker
- [ ] Click source detection
- [ ] Data centers detection
- [ ] Enhanced bot detection

### Aggregator
- [ ] Reports API
- [ ] Click stream API
- [ ] S3 support for ClickHouse

### SSL Bot
- [ ] Domains monitoring

## 🔧 Development

### Project Structure
```
├── click-router/          # Main redirect service
├── click-tracker/         # Click processing service
├── click-aggregator/      # Analytics aggregation
├── click-router-api/      # Route management API
├── click-aggregator-api/  # Analytics API
├── infra/                 # Infrastructure setup
├── data/                  # Static data (GeoIP, UA parser)
├── docker-compose.yml     # Complete system deployment
└── makefile              # Enhanced build system
```

### Build System Commands

**Get Help:**
```bash
make help                  # Show all available commands
```

**Development Workflow:**
```bash
make dev-setup            # Complete development setup
make dev-start            # Start development environment
make dev-stop             # Stop development environment
make dev-restart          # Restart development environment
```

**Building:**
```bash
make build                # Build all services (debug)
make build-release        # Build all services (release)
make build-docker         # Build all Docker images
make build-<service>      # Build specific service
```

**Testing:**
```bash
make test                 # Run all tests
make test-watch           # Run tests in watch mode
make test-coverage        # Run tests with coverage
make test-<service>       # Test specific service
```

**Code Quality:**
```bash
make lint                 # Run linters
make format               # Format code
make check                # Run all checks (lint, test, build)
```

**Infrastructure:**
```bash
make infra-start          # Start infrastructure
make infra-stop           # Stop infrastructure
make infra-reset          # Reset infrastructure (clean data)
```

**Health & Monitoring:**
```bash
make health-check         # Check all services
make health-check-infra   # Check infrastructure
make health-check-apps    # Check applications
make validate             # Validate system
```

**Logs:**
```bash
make logs                 # All services
make logs-<service>        # Specific service
```

**Deployment:**
```bash
make deploy-local         # Local deployment
make deploy-docker        # Docker deployment
make deploy-aws           # AWS deployment
```

### Service-Specific Commands

Each service supports individual commands:
- `make build-<service>` - Build specific service
- `make test-<service>` - Test specific service
- `make clean-<service>` - Clean specific service
- `make logs-<service>` - Logs for specific service

Available services: `click-router`, `click-router-api`, `click-tracker`, `click-aggregator`, `click-aggregator-api`

## 📄 License

[Add your license information here]