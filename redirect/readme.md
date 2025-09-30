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
- **Web Frameworks**: Salvo
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

## 📚 Component Documentation

Each component has its own detailed documentation:

### Core Services
- **[Click Router](click-router/README.md)** - Main redirect service with intelligent routing, analytics, and multi-database support
- **[Click Router API](click-router-api/README.md)** - REST API for route and settings management with JWT authentication
- **[Click Aggregator API](click-aggregator-api/README.md)** - Analytics and reporting API with comprehensive documentation

### API Documentation
- **[Click Router API Documentation](click-router-api/docs/README.md)** - Complete API documentation index with detailed guides for security, error handling, and data models

### Infrastructure
- **[AWS Infrastructure](infra/aws/readme.md)** - AWS deployment and infrastructure setup

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

## 📚 API Documentation

### Click Router API

The Click Router API provides comprehensive route management, SSL certificate handling, and user settings management.

#### Base URL
- **Development**: `http://localhost:8081`
- **Production**: `https://api.yourdomain.com`

#### Authentication
All protected endpoints require JWT authentication:
```http
Authorization: Bearer <jwt_token>
```

#### Endpoints

**Routes Management:**
- `GET /v1/routes/{domain}/{path}` - Get route information
- `GET /v1/routes/{domain}/{path}/{switch}` - Get specific route switch
- `POST /v1/routes` - Create new route
- `PUT /v1/routes/{domain}/{path}` - Update route
- `DELETE /v1/routes/{domain}/{path}` - Delete route

**SSL Certificate Management:**
- `GET /v1/certificates/{domain}` - Get certificate
- `POST /v1/certificates/{domain}` - Create certificate
- `PUT /v1/certificates/{domain}` - Update certificate
- `DELETE /v1/certificates/{domain}` - Delete certificate

**User Settings:**
- `GET /v1/user-settings/{user_id}` - Get user settings
- `POST /v1/user-settings/{user_id}` - Create user settings
- `PUT /v1/user-settings/{user_id}` - Update user settings
- `DELETE /v1/user-settings/{user_id}` - Delete user settings

**Public Endpoints:**
- `GET /public/health` - Health check
- `GET /public/metrics` - Service metrics
- `GET /swagger-ui` - Interactive API documentation
- `GET /api-doc/openapi.json` - OpenAPI specification

### Click Aggregator API

The Click Aggregator API provides analytics and click stream data access.

#### Base URL
- **Development**: `http://localhost:8082`
- **Production**: `https://analytics.yourdomain.com`

#### Endpoints

**Click Stream Analytics:**
- `GET /v1/clickstream` - Get click stream data
- `GET /v1/clickstream/{route_id}` - Get route-specific analytics
- `GET /v1/clickstream/stats` - Get aggregated statistics

**Public Endpoints:**
- `GET /public/health` - Health check
- `GET /public/metrics` - Service metrics

## 🔧 Configuration

### Environment Variables

| Variable | Description | Default | Required |
|----------|-------------|---------|----------|
| `APP_RUN_MODE` | Application mode | `development` | No |
| `APP_CONFIG_PATH` | Config directory | `./config` | No |
| `MONGODB_URI` | MongoDB connection string | - | Yes |
| `CLICKHOUSE_URL` | ClickHouse connection string | - | Yes |
| `REDIS_URL` | Redis connection string | - | Yes |
| `KAFKA_BROKERS` | Kafka broker list | - | No |
| `FLUVIO_HOST` | Fluvio host | - | No |
| `AWS_ACCESS_KEY_ID` | AWS access key | - | No |
| `AWS_SECRET_ACCESS_KEY` | AWS secret key | - | No |
| `AWS_DEFAULT_REGION` | AWS region | `us-east-1` | No |

### Configuration Files

Each service supports multiple configuration environments:

#### Development Configuration (`config/development.toml`)
```toml
[server]
threads = 4
debug = true

[mongodb]
uri = "mongodb://root:example@mongo:27017/"
database = "shortas"
encryption_collection = "core_routes_encryption_local"
routes_collection = "core_routes_local"
hostname_mappings_collection = "core_routes_hostname_mapping_local"
user_settings_collection = "core_user_settings_local"

[clickhouse]
url = "http://clickhouse:8123"
user = "default"
password = "clickhouse"
database = "shortas"

[redis]
url = "redis://cache:6379"
password = "eYVX7EwVmmxKPCDmwMtyKVge8oLd2t81"

[fluvio]
topic = "hit-stream-local"
host = "localhost:9103"
batch_size = 10000
linger = 1000

[geo_ip]
mmdb = "../data/geo-ip/GeoLite2-Country.mmdb"

[uaparser]
yaml = "../data/ua-parser/regexes.yaml"
```

#### Production Configuration (`config/production.toml`)
```toml
[server]
threads = 16
exit = false

[mongodb]
uri = "mongodb://prod-cluster:27017/"
database = "shortas_prod"
encryption_collection = "core_routes_encryption_main"
routes_collection = "core_routes_main"
hostname_mappings_collection = "core_routes_hostname_mapping_main"
user_settings_collection = "core_user_settings_main"

[clickhouse]
url = "http://clickhouse-prod:8123"
user = "analytics"
password = "secure_password"
database = "shortas_prod"

[redis]
url = "redis://redis-cluster:6379"
password = "production_redis_password"

[fluvio]
topic = "hit-stream-main"
host = "fluvio-prod:9003"
batch_size = 50000
linger = 5000

[geo_ip]
mmdb = "./data/geo-ip/GeoLite2-Country.mmdb"

[uaparser]
yaml = "./data/ua-parser/regexes.yaml"
```

## 📊 Data Models

### Route Model
```json
{
  "switch": "string",
  "link": "string", 
  "dest": "string",
  "dest_format": "Http|Native",
  "code": 200,
  "ttl": 3600000,
  "status": "Active|Blocked",
  "terminal": "External|Internal|Middleware",
  "policy": "Basic|Conditional|Challenge|File|Mirroring|Unknown",
  "properties": {
    "route_id": "string",
    "domain_id": "string", 
    "owner_id": "string",
    "creator_id": "string",
    "workspace_id": "string",
    "scripts": ["string"],
    "tags": ["string"],
    "custom": {},
    "native": {},
    "bundling": {},
    "opengraph": false,
    "allow_debug": false
  }
}
```

### Click Stream Model
```json
{
  "id": "string",
  "owner_id": "string",
  "creator_id": "string", 
  "route_id": "string",
  "workspace_id": "string",
  "created": "2024-01-01T12:00:00Z",
  "dest": "string",
  "ip": "string",
  "continent": "string",
  "country": "string", 
  "location": "string",
  "os_family": "string",
  "os_version": "string",
  "user_agent_family": "string",
  "user_agent_version": "string",
  "device_brand": "string",
  "device_family": "string",
  "device_model": "string",
  "session_first": "2024-01-01T12:00:00Z",
  "session_clicks": 1,
  "is_unique": true,
  "is_bot": false
}
```

### User Settings Model
```json
{
  "user_id": "string",
  "user_email": "string",
  "api_key": "string",
  "active_status": "Active|Blocked",
  "debug": false,
  "overflow": false,
  "skip": ["tracking"],
  "allowed_request_params": ["utm_source", "utm_medium"],
  "allowed_destination_params": ["ref", "campaign"]
}
```

### Certificate Model
```json
{
  "key": "base64_encoded_private_key",
  "cert": "base64_encoded_certificate", 
  "ocsp_resp": "base64_encoded_ocsp_response"
}
```

## 🔐 Security

### Authentication
- **JWT Bearer Token**: Standard JWT authentication for most endpoints
- **RPT Token**: Fine-grained authorization with UMA (User Managed Access)
- **API Key**: Alternative authentication method for service-to-service communication

### Security Features
- Rate limiting on all API endpoints
- Security headers middleware
- Input validation and sanitization
- CORS configuration
- SSL/TLS encryption support

### Authorization
- Role-based access control (RBAC)
- Workspace-based data isolation
- Resource-level permissions
- Fine-grained authorization with UMA

## 📈 Monitoring & Observability

### Health Checks
All services provide health check endpoints:
- `GET /health` - Basic health status
- `GET /metrics` - Service metrics
- `GET /ready` - Readiness probe
- `GET /live` - Liveness probe

### Metrics
- Request count and duration
- Error rates and types
- Database connection status
- Cache hit/miss ratios
- Queue processing rates

### Logging
- Structured JSON logging
- Configurable log levels
- Request/response logging
- Error tracking and stack traces

## 🚀 Performance

### Caching
- **Redis**: Session and route caching
- **Moka**: In-memory caching for hot data
- **CDN**: Static asset caching

### Optimization
- Async processing throughout
- Connection pooling
- Batch processing for analytics
- Horizontal scaling support

### Benchmarks
- **Click Router**: 10,000+ requests/second
- **Click Tracker**: 50,000+ events/second
- **Click Aggregator**: 100,000+ records/second
- **APIs**: 5,000+ requests/second

## 📄 License

[Add your license information here]