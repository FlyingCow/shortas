# Click Router

A high-performance, intelligent URL redirection service built in Rust. Click Router provides advanced routing capabilities with conditional logic, analytics, and multi-database support for enterprise-grade URL shortening and redirection services.

## 🚀 Features

### Core Functionality
- **High-Performance Routing**: Async/await architecture for maximum throughput
- **Intelligent Redirection**: Conditional routing based on user characteristics
- **Analytics & Tracking**: Comprehensive hit tracking and user behavior analysis
- **Multi-Database Support**: MongoDB and DynamoDB integration
- **Advanced Caching**: Multi-level caching with TTL and invalidation
- **TLS Support**: Custom certificate management for HTTPS

### Advanced Routing
- **Conditional Routing**: Route users based on:
  - User Agent (Browser, OS, Device)
  - Geographic Location (Country-based routing)
  - Time-based conditions
  - Custom expressions
- **Multiple Routing Policies**:
  - Basic routing
  - Conditional routing with complex expressions
  - Challenge-based routing
  - File serving
  - Mirroring
- **A/B Testing**: Built-in support for traffic splitting

### Analytics & Monitoring
- **Hit Tracking**: Every click is logged with detailed metadata
- **Real-time Analytics**: Kafka and Fluvio integration
- **Geographic Analytics**: Country and region-based insights
- **Device Analytics**: Browser, OS, and device tracking
- **Debug Mode**: Conditional debug information for development

## 🏗️ Architecture

Click Router uses a modular, pipeline-based architecture:

```
Request → Flow Router → Modules → Adapters → Response
```

### Core Components

- **Flow Router**: Central request processing engine
- **Modules**: Pluggable processing steps (Root, Conditional, NotFound, etc.)
- **Adapters**: Service integrations (databases, caches, analytics)
- **Models**: Data structures for routes, hits, and settings

### Request Processing Pipeline

1. **Start**: Initial request processing and validation
2. **UrlExtract**: URL analysis and route matching
3. **Register**: Hit logging and analytics
4. **BuildResult**: Response generation
5. **End**: Final response processing

## 📦 Installation

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

## 🛠️ Development

### Project Structure

```
src/
├── adapters/          # Service integrations
│   ├── aws/          # DynamoDB integration
│   ├── mongodb/      # MongoDB integration
│   ├── moka/         # Caching layer
│   └── fluvio/       # Analytics streaming
├── core/             # Core routing logic
│   ├── flow_router.rs # Main router
│   └── modules/      # Processing modules
├── model/            # Data models
└── settings.rs       # Configuration
```

### Building

```bash
# Development build
cargo build

# Release build
cargo build --release

# Run benchmarks
make bench-click-router
```

### Testing

```bash
# Run all tests
cargo test

# Run specific test
cargo test conditional_routing

# Run with coverage
cargo test --features coverage
```

## 📊 Performance

Click Router is designed for high performance:

- **Throughput**: 100,000+ requests/second
- **Latency**: Sub-millisecond response times
- **Memory**: Efficient memory usage with caching
- **Scalability**: Horizontal scaling support

### Benchmarking

```bash
# Run performance benchmarks
make bench-click-router

# Custom benchmark
cargo bench --bench flow_router
```

## 🔒 Security

### TLS Support

Click Router supports custom TLS certificates:

```rust
// Custom certificate resolver
struct ServerConfigResolver {
    // Certificate management
}
```

### Security Features

- **Input Validation**: Comprehensive URL and parameter validation
- **Rate Limiting**: Built-in abuse protection
- **Secure Headers**: Proper HTTP security headers
- **TLS Termination**: Custom certificate management

## 📈 Monitoring

### Health Checks

```bash
# Health check endpoint
curl http://localhost:5800/health
```

### Metrics

- Request count and latency
- Cache hit/miss ratios
- Database connection metrics
- Analytics throughput

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests
5. Submit a pull request

### Development Guidelines

- Follow Rust conventions
- Add comprehensive tests
- Update documentation
- Ensure performance benchmarks pass

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🔗 Links

- **Homepage**: https://shortas.com
- **Documentation**: https://shortas.tech/
- **Repository**: https://github.com/FlyingCow/shortas
- **Issues**: https://github.com/FlyingCow/shortas/issues

## 🆘 Support

For support and questions:

- Create an issue on GitHub
- Check the documentation
- Join our community discussions

---

**Click Router** - Intelligent URL redirection for the modern web.


