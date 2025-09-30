---
layout: page
title: Configuration
permalink: /getting-started/configuration/
---

# Configuration Guide

This guide covers how to configure Shortas for different environments and use cases.

## ⚙️ Configuration Overview

Shortas uses environment-based configuration with TOML files. Each service supports multiple configuration environments:

- `config/default.toml` - Base configuration
- `config/development.toml` - Development overrides
- `config/production.toml` - Production settings
- `config/test.toml` - Test configuration

## 🔧 Environment Variables

### Core Application Settings

| Variable | Description | Default | Required |
|----------|-------------|---------|----------|
| `APP_RUN_MODE` | Application mode | `development` | No |
| `APP_CONFIG_PATH` | Config directory | `./config` | No |
| `LOG_LEVEL` | Logging level | `info` | No |

### Database Configuration

| Variable | Description | Default | Required |
|----------|-------------|---------|----------|
| `MONGODB_URI` | MongoDB connection string | - | Yes |
| `CLICKHOUSE_URL` | ClickHouse connection string | - | Yes |
| `REDIS_URL` | Redis connection string | - | Yes |
| `AWS_ACCESS_KEY_ID` | AWS access key | - | No |
| `AWS_SECRET_ACCESS_KEY` | AWS secret key | - | No |
| `AWS_DEFAULT_REGION` | AWS region | `us-east-1` | No |

### Message Queue Configuration

| Variable | Description | Default | Required |
|----------|-------------|---------|----------|
| `KAFKA_BROKERS` | Kafka broker list | - | No |
| `FLUVIO_HOST` | Fluvio host | - | No |

## 📁 Configuration Files

### Base Configuration (`config/default.toml`)

```toml
[server]
threads = 8
listen_os_signals = true
exit = true

[redirect]
not_found_url = "http://localhost:5801/404/{}"
index_url = "http://localhost:5801/index/{}"

[mongodb]
uri = "mongodb://root:example@mongo:27017/"
database = "shortas"
encryption_collection = "core_routes_encryption_main"
routes_collection = "core_routes_main"
hostname_mappings_collection = "core_routes_hostname_mapping_main"
user_settings_collection = "core_user_settings_main"

[clickhouse]
url = "http://clickhouse:8123"
user = "default"
password = "clickhouse"
database = "shortas"

[redis]
url = "redis://cache:6379"
password = "eYVX7EwVmmxKPCDmwMtyKVge8oLd2t81"

[fluvio]
topic = "hit-stream-main"
host = "sc:9003"
batch_size = 10000
linger = 1000

[geo_ip]
mmdb = "../data/geo-ip/GeoLite2-Country.mmdb"

[uaparser]
yaml = "../data/ua-parser/regexes.yaml"

[moka]
[moka.crypto_cache]
max_capacity = 10_000
time_to_live_minutes = 60
time_to_idle_minutes = 20

[moka.routes_cache]
max_capacity = 10_000
time_to_live_minutes = 60
time_to_idle_minutes = 20

[moka.user_settings_cache]
max_capacity = 10_000
time_to_live_minutes = 60
time_to_idle_minutes = 20
```

### Development Configuration (`config/development.toml`)

```toml
[server]
threads = 4

[debug]
enabled = true
verbose = true

[mongodb]
uri = "mongodb://localhost:27017/"
database = "shortas_dev"

[moka.routes_cache]
max_capacity = 1000
time_to_live_minutes = 30

[fluvio]
topic = "hit-stream-local"
host = "localhost:9103"
batch_size = 1000
linger = 100
```

### Production Configuration (`config/production.toml`)

```toml
[server]
threads = 16
exit = false

[debug]
enabled = false
verbose = false

[mongodb]
uri = "mongodb://prod-cluster:27017/"
database = "shortas_prod"

[moka.routes_cache]
max_capacity = 100000
time_to_live_minutes = 60

[fluvio]
topic = "hit-stream-main"
host = "fluvio-prod:9003"
batch_size = 50000
linger = 5000
```

## 🗄️ Database Configuration

### MongoDB Configuration

```toml
[mongodb]
uri = "mongodb://username:password@host:port/"
database = "shortas"
routes_collection = "routes"
encryption_collection = "encryption"
user_settings_collection = "user_settings"
hostname_mappings_collection = "hostname_mappings"
```

### ClickHouse Configuration

```toml
[clickhouse]
url = "http://clickhouse:8123"
user = "default"
password = "clickhouse"
database = "shortas"
```

### Redis Configuration

```toml
[redis]
url = "redis://username:password@host:port/db"
password = "your_redis_password"
```

### DynamoDB Configuration (Alternative)

```toml
[aws.dynamo]
routes_table = "routes-table"
encryption_table = "encryption-table"
user_settings_table = "user-settings-table"
```

## 📊 Analytics Configuration

### Kafka Configuration

```toml
[kafka]
brokers = ["localhost:9092", "localhost:9093"]
topic = "hit-stream-main"
ack_timeout_secs = 60
batch_size = 100
consumers_count = 2
iteration_seconds = 1
```

### Fluvio Configuration

```toml
[fluvio]
topic = "hit-stream-main"
host = "sc:9003"
batch_size = 10000
linger = 1000
```

## 🗂️ Cache Configuration

### Moka Cache Settings

```toml
[moka.routes_cache]
max_capacity = 10000
time_to_live_minutes = 60
time_to_idle_minutes = 20

[moka.crypto_cache]
max_capacity = 1000
time_to_live_minutes = 1440
time_to_idle_minutes = 60

[moka.user_settings_cache]
max_capacity = 5000
time_to_live_minutes = 30
time_to_idle_minutes = 10
```

## 🔒 Security Configuration

### JWT Authentication

```toml
[jwt]
keycloak_url = "http://keycloak:8080"
realm = "shortas"
client_id = "shortas-api"
jwks_url = "http://keycloak:8080/realms/shortas/protocol/openid-connect/certs"
```

### SSL/TLS Configuration

```toml
[tls]
cert_file = "certs/cert.pem"
key_file = "certs/key.pem"
ca_file = "certs/ca.pem"
```

## 🌍 Geographic Configuration

### GeoIP Database

```toml
[geo_ip]
mmdb = "./data/geo-ip/GeoLite2-Country.mmdb"
```

### User Agent Parser

```toml
[uaparser]
yaml = "./data/ua-parser/regexes.yaml"
```

## 📈 Performance Configuration

### Server Settings

```toml
[server]
threads = 16  # CPU cores * 2
listen_os_signals = true
exit = false  # Keep running in production
```

### Database Connection Pooling

```toml
[mongodb]
max_pool_size = 100
min_pool_size = 10
max_idle_time_ms = 30000
```

## 🔧 Service-Specific Configuration

### Click Router Configuration

```toml
[redirect]
not_found_url = "http://localhost:5801/404/{}"
index_url = "http://localhost:5801/index/{}"
```

### Click Tracker Configuration

```toml
[tracker]
batch_size = 1000
flush_interval = 5
max_retries = 3
```

### Click Aggregator Configuration

```toml
[aggregator]
batch_size = 10000
flush_interval = 30
compression = "gzip"
```

## 🐳 Docker Configuration

### Environment Variables in Docker

```yaml
# docker-compose.yml
services:
  click-router:
    environment:
      - APP_RUN_MODE=production
      - MONGODB_URI=mongodb://mongodb:27017/shortas
      - REDIS_URL=redis://redis:6379
      - CLICKHOUSE_URL=http://clickhouse:8123
```

### Docker Secrets

```yaml
# docker-compose.yml
services:
  click-router:
    secrets:
      - mongodb_password
      - redis_password
    environment:
      - MONGODB_URI=mongodb://user:${MONGODB_PASSWORD}@mongodb:27017/shortas

secrets:
  mongodb_password:
    file: ./secrets/mongodb_password.txt
  redis_password:
    file: ./secrets/redis_password.txt
```

## ☁️ Cloud Configuration

### AWS Configuration

```toml
[aws]
region = "us-east-1"
access_key_id = "your_access_key"
secret_access_key = "your_secret_key"

[aws.dynamo]
routes_table = "shortas-routes"
encryption_table = "shortas-encryption"
user_settings_table = "shortas-user-settings"
```

### Kubernetes Configuration

```yaml
# configmap.yaml
apiVersion: v1
kind: ConfigMap
metadata:
  name: shortas-config
data:
  default.toml: |
    [server]
    threads = 8
    [mongodb]
    uri = "mongodb://mongodb:27017/shortas"
```

## 🔍 Configuration Validation

### Validate Configuration

```bash
# Validate all configurations
make validate-config

# Validate specific service
make validate-config-click-router
```

### Test Configuration

```bash
# Test database connections
make test-db-connections

# Test message queue connections
make test-queue-connections
```

## 📚 Configuration Examples

### Minimal Configuration

```toml
[server]
threads = 4

[mongodb]
uri = "mongodb://localhost:27017/"
database = "shortas"

[redis]
url = "redis://localhost:6379"
```

### High-Performance Configuration

```toml
[server]
threads = 32

[mongodb]
uri = "mongodb://cluster:27017/"
max_pool_size = 200

[redis]
url = "redis://cluster:6379"

[moka.routes_cache]
max_capacity = 1000000
time_to_live_minutes = 120
```

## 🚨 Troubleshooting Configuration

### Common Issues

**Configuration not loading:**
```bash
# Check config path
echo $APP_CONFIG_PATH

# Verify config files exist
ls -la config/
```

**Database connection fails:**
```bash
# Test MongoDB connection
mongosh "mongodb://localhost:27017/shortas"

# Test ClickHouse connection
curl http://localhost:8123/ping
```

**Environment variables not set:**
```bash
# Check environment variables
env | grep -E "(MONGODB|REDIS|CLICKHOUSE)"
```

## 📚 Next Steps

Now that you have Shortas configured:

1. [Start your first service](first-steps/)
2. [Learn about the architecture](../architecture/)
3. [Explore the APIs](../api/)
4. [Deploy to production](../deployment/)

---

**Need help with configuration?** Check our [troubleshooting guide](../deployment/troubleshooting/) or [open an issue](https://github.com/FlyingCow/shortas/issues) on GitHub.
