# Click Router Deployment Guide

This guide covers deployment strategies, configuration, and operational considerations for Click Router in various environments.

## 🚀 Quick Start

### Local Development

```bash
# Clone the repository
git clone https://github.com/FlyingCow/shortas.git
cd shortas/redirect/click-router

# Install dependencies
cargo build

# Start MongoDB (using Docker)
docker run -d --name mongodb -p 27017:27017 mongo:latest

# Run the application
cargo run --release
```

### Docker Deployment

```bash
# Build the image
docker build -t click-router .

# Run the container
docker run -d \
  --name click-router \
  -p 5800:5800 \
  -e APP_RUN_MODE=production \
  -e APP_CONFIG_PATH=/app/config \
  click-router
```

## 🐳 Docker Deployment

### Dockerfile Overview

The project includes a multi-stage Dockerfile optimized for production:

```dockerfile
# Build stage
FROM rust:1.75-slim as builder
# ... build configuration

# Runtime stage  
FROM debian:bookworm-slim
# ... runtime configuration
```

### Docker Compose

Create a `docker-compose.yml` for full stack deployment:

```yaml
version: '3.8'

services:
  click-router:
    build: .
    ports:
      - "5800:5800"
    environment:
      - APP_RUN_MODE=production
      - APP_CONFIG_PATH=/app/config
    depends_on:
      - mongodb
      - kafka
    volumes:
      - ./config:/app/config
      - ./data:/app/data
    restart: unless-stopped

  mongodb:
    image: mongo:latest
    ports:
      - "27017:27017"
    volumes:
      - mongodb_data:/data/db
    environment:
      - MONGO_INITDB_ROOT_USERNAME=root
      - MONGO_INITDB_ROOT_PASSWORD=example

  kafka:
    image: confluentinc/cp-kafka:latest
    ports:
      - "9092:9092"
    environment:
      KAFKA_ZOOKEEPER_CONNECT: zookeeper:2181
      KAFKA_ADVERTISED_LISTENERS: PLAINTEXT://localhost:9092
      KAFKA_OFFSETS_TOPIC_REPLICATION_FACTOR: 1

  zookeeper:
    image: confluentinc/cp-zookeeper:latest
    ports:
      - "2181:2181"
    environment:
      ZOOKEEPER_CLIENT_PORT: 2181
      ZOOKEEPER_TICK_TIME: 2000

volumes:
  mongodb_data:
```

### Kubernetes Deployment

#### Namespace

```yaml
apiVersion: v1
kind: Namespace
metadata:
  name: click-router
```

#### ConfigMap

```yaml
apiVersion: v1
kind: ConfigMap
metadata:
  name: click-router-config
  namespace: click-router
data:
  default.toml: |
    [server]
    threads = 8
    listen_os_signals = true
    exit = true
    
    [mongodb]
    uri = "mongodb://mongodb:27017/"
    database = "shortas"
    
    [moka.routes_cache]
    max_capacity = 10000
    time_to_live_minutes = 60
```

#### Deployment

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: click-router
  namespace: click-router
spec:
  replicas: 3
  selector:
    matchLabels:
      app: click-router
  template:
    metadata:
      labels:
        app: click-router
    spec:
      containers:
      - name: click-router
        image: click-router:latest
        ports:
        - containerPort: 5800
        env:
        - name: APP_RUN_MODE
          value: "production"
        - name: APP_CONFIG_PATH
          value: "/app/config"
        volumeMounts:
        - name: config
          mountPath: /app/config
        - name: data
          mountPath: /app/data
        resources:
          requests:
            memory: "256Mi"
            cpu: "250m"
          limits:
            memory: "512Mi"
            cpu: "500m"
        livenessProbe:
          httpGet:
            path: /health
            port: 5800
          initialDelaySeconds: 30
          periodSeconds: 10
        readinessProbe:
          httpGet:
            path: /health
            port: 5800
          initialDelaySeconds: 5
          periodSeconds: 5
      volumes:
      - name: config
        configMap:
          name: click-router-config
      - name: data
        persistentVolumeClaim:
          claimName: click-router-data
```

#### Service

```yaml
apiVersion: v1
kind: Service
metadata:
  name: click-router-service
  namespace: click-router
spec:
  selector:
    app: click-router
  ports:
  - port: 80
    targetPort: 5800
  type: LoadBalancer
```

#### Ingress

```yaml
apiVersion: networking.k8s.io/v1
kind: Ingress
metadata:
  name: click-router-ingress
  namespace: click-router
  annotations:
    nginx.ingress.kubernetes.io/rewrite-target: /
    cert-manager.io/cluster-issuer: "letsencrypt-prod"
spec:
  tls:
  - hosts:
    - short.ly
    - www.short.ly
    secretName: click-router-tls
  rules:
  - host: short.ly
    http:
      paths:
      - path: /
        pathType: Prefix
        backend:
          service:
            name: click-router-service
            port:
              number: 80
```

## ⚙️ Configuration

### Environment Variables

| Variable | Description | Default | Required |
|----------|-------------|---------|----------|
| `APP_RUN_MODE` | Application mode | `development` | No |
| `APP_CONFIG_PATH` | Config directory | `./config` | No |
| `MONGODB_URI` | MongoDB connection string | - | Yes |
| `KAFKA_BROKERS` | Kafka broker list | - | No |
| `FLUVIO_HOST` | Fluvio host | - | No |

### Configuration Files

#### Base Configuration (`config/default.toml`)

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

[fluvio]
[fluvio.hit_stream]
topic = "hit-stream-main"
host = "sc:9003"
batch_size = 10000
linger = 1000

[geo_ip]
mmdb = "../data/geo-ip/GeoLite2-Country.mmdb"

[uaparser]
yaml = "../data/ua-parser/user-agents.yaml"
```

#### Development Configuration (`config/development.toml`)

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
```

#### Production Configuration (`config/production.toml`)

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

[fluvio.hit_stream]
batch_size = 50000
linger = 5000
```

## 🗄️ Database Setup

### MongoDB

#### Installation

```bash
# Using Docker
docker run -d \
  --name mongodb \
  -p 27017:27017 \
  -e MONGO_INITDB_ROOT_USERNAME=root \
  -e MONGO_INITDB_ROOT_PASSWORD=example \
  mongo:latest

# Using package manager (Ubuntu/Debian)
sudo apt-get install mongodb

# Using package manager (CentOS/RHEL)
sudo yum install mongodb-server
```

#### Database Initialization

```javascript
// Connect to MongoDB
use shortas

// Create collections
db.createCollection("routes")
db.createCollection("encryption")
db.createCollection("user_settings")

// Create indexes
db.routes.createIndex({ "switch": 1, "link": 1 })
db.routes.createIndex({ "properties.route_id": 1 })
db.routes.createIndex({ "properties.domain_id": 1 })

// Insert sample route
db.routes.insertOne({
  "switch": "main",
  "link": "example",
  "dest": "https://example.com",
  "dest_format": "Http",
  "code": 302,
  "ttl": 3600,
  "status": "Active",
  "terminal": "External",
  "policy": {
    "type": "Basic"
  },
  "properties": {
    "route_id": "route_123",
    "domain_id": "domain_456",
    "owner_id": "user_789",
    "allow_debug": true
  }
})
```

### DynamoDB

#### AWS CLI Setup

```bash
# Install AWS CLI
pip install awscli

# Configure credentials
aws configure

# Create tables
aws dynamodb create-table \
  --table-name routes-table \
  --attribute-definitions \
    AttributeName=switch,AttributeType=S \
    AttributeName=link,AttributeType=S \
  --key-schema \
    AttributeName=switch,KeyType=HASH \
    AttributeName=link,KeyType=RANGE \
  --billing-mode PAY_PER_REQUEST
```

#### LocalStack (Development)

```bash
# Start LocalStack
docker run -d \
  --name localstack \
  -p 4566:4566 \
  -e SERVICES=dynamodb \
  localstack/localstack

# Create tables using AWS CLI
aws --endpoint-url=http://localhost:4566 dynamodb create-table \
  --table-name routes-table \
  --attribute-definitions \
    AttributeName=switch,AttributeType=S \
    AttributeName=link,AttributeType=S \
  --key-schema \
    AttributeName=switch,KeyType=HASH \
    AttributeName=link,KeyType=RANGE \
  --billing-mode PAY_PER_REQUEST
```

## 📊 Analytics Setup

### Kafka

#### Installation

```bash
# Using Docker Compose
version: '3.8'
services:
  zookeeper:
    image: confluentinc/cp-zookeeper:latest
    environment:
      ZOOKEEPER_CLIENT_PORT: 2181
      ZOOKEEPER_TICK_TIME: 2000

  kafka:
    image: confluentinc/cp-kafka:latest
    depends_on:
      - zookeeper
    ports:
      - "9092:9092"
    environment:
      KAFKA_BROKER_ID: 1
      KAFKA_ZOOKEEPER_CONNECT: zookeeper:2181
      KAFKA_ADVERTISED_LISTENERS: PLAINTEXT://localhost:9092
      KAFKA_OFFSETS_TOPIC_REPLICATION_FACTOR: 1
```

#### Topic Creation

```bash
# Create hit stream topic
kafka-topics --create \
  --topic hit-stream-main \
  --bootstrap-server localhost:9092 \
  --partitions 3 \
  --replication-factor 1
```

### Fluvio

#### Installation

```bash
# Install Fluvio CLI
curl -fsS https://packages.fluvio.io/v1/install.sh | bash

# Start Fluvio cluster
fluvio cluster start

# Create topic
fluvio topic create hit-stream-main --partitions 3
```

## 🔒 Security Configuration

### TLS/SSL Setup

#### Self-Signed Certificates (Development)

```bash
# Generate private key
openssl genrsa -out key.pem 2048

# Generate certificate
openssl req -new -x509 -key key.pem -out cert.pem -days 365
```

#### Let's Encrypt (Production)

```bash
# Install certbot
sudo apt-get install certbot

# Generate certificate
sudo certbot certonly --standalone -d short.ly

# Copy certificates
sudo cp /etc/letsencrypt/live/short.ly/fullchain.pem certs/cert.pem
sudo cp /etc/letsencrypt/live/short.ly/privkey.pem certs/key.pem
```

### Firewall Configuration

```bash
# UFW (Ubuntu)
sudo ufw allow 5800/tcp
sudo ufw allow 22/tcp
sudo ufw enable

# iptables
sudo iptables -A INPUT -p tcp --dport 5800 -j ACCEPT
sudo iptables -A INPUT -p tcp --dport 22 -j ACCEPT
```

## 📈 Monitoring & Logging

### Health Checks

```bash
# Basic health check
curl -f http://localhost:5800/health || exit 1

# Detailed health check
curl -s http://localhost:5800/health | jq '.'
```

### Logging Configuration

#### Structured Logging

```toml
[logging]
level = "info"
format = "json"
output = "stdout"

[logging.fields]
service = "click-router"
version = "0.1.0"
environment = "production"
```

#### Log Rotation

```bash
# Install logrotate
sudo apt-get install logrotate

# Configure log rotation
sudo tee /etc/logrotate.d/click-router << EOF
/var/log/click-router/*.log {
    daily
    missingok
    rotate 30
    compress
    delaycompress
    notifempty
    create 644 click-router click-router
    postrotate
        systemctl reload click-router
    endscript
}
EOF
```

### Metrics Collection

#### Prometheus Configuration

```yaml
# prometheus.yml
global:
  scrape_interval: 15s

scrape_configs:
  - job_name: 'click-router'
    static_configs:
      - targets: ['localhost:5800']
    metrics_path: /metrics
    scrape_interval: 5s
```

#### Grafana Dashboard

```json
{
  "dashboard": {
    "title": "Click Router Metrics",
    "panels": [
      {
        "title": "Request Rate",
        "type": "graph",
        "targets": [
          {
            "expr": "rate(http_requests_total[5m])"
          }
        ]
      },
      {
        "title": "Response Time",
        "type": "graph", 
        "targets": [
          {
            "expr": "histogram_quantile(0.95, rate(http_request_duration_seconds_bucket[5m]))"
          }
        ]
      }
    ]
  }
}
```

## 🚀 Performance Optimization

### System Tuning

#### Linux Kernel Parameters

```bash
# /etc/sysctl.conf
net.core.somaxconn = 65535
net.core.netdev_max_backlog = 5000
net.ipv4.tcp_max_syn_backlog = 65535
net.ipv4.tcp_keepalive_time = 600
net.ipv4.tcp_keepalive_intvl = 60
net.ipv4.tcp_keepalive_probes = 3
```

#### File Descriptor Limits

```bash
# /etc/security/limits.conf
click-router soft nofile 65535
click-router hard nofile 65535
```

### Application Tuning

#### Thread Configuration

```toml
[server]
threads = 16  # CPU cores * 2
```

#### Cache Configuration

```toml
[moka.routes_cache]
max_capacity = 100000
time_to_live_minutes = 60
time_to_idle_minutes = 20
```

#### Database Connection Pooling

```toml
[mongodb]
max_pool_size = 100
min_pool_size = 10
max_idle_time_ms = 30000
```

## 🔄 Backup & Recovery

### Database Backup

#### MongoDB Backup

```bash
# Full backup
mongodump --host localhost:27017 --db shortas --out /backup/mongodb/

# Incremental backup
mongodump --host localhost:27017 --db shortas --query '{"timestamp": {"$gte": "2024-01-01"}}' --out /backup/mongodb/
```

#### DynamoDB Backup

```bash
# Create backup
aws dynamodb create-backup \
  --table-name routes-table \
  --backup-name routes-backup-$(date +%Y%m%d)
```

### Application Backup

```bash
# Backup configuration
tar -czf click-router-config-$(date +%Y%m%d).tar.gz config/

# Backup data files
tar -czf click-router-data-$(date +%Y%m%d).tar.gz data/
```

## 🚨 Troubleshooting

### Common Issues

#### Port Already in Use

```bash
# Check port usage
sudo netstat -tlnp | grep 5800

# Kill process
sudo kill -9 $(sudo lsof -t -i:5800)
```

#### Database Connection Issues

```bash
# Test MongoDB connection
mongosh "mongodb://localhost:27017/shortas"

# Test DynamoDB connection
aws dynamodb list-tables --endpoint-url http://localhost:4566
```

#### Cache Issues

```bash
# Clear cache (if supported)
curl -X POST http://localhost:5800/admin/cache/clear

# Restart application
sudo systemctl restart click-router
```

### Debug Mode

```bash
# Enable debug logging
export RUST_LOG=debug
cargo run

# Debug specific module
export RUST_LOG=click_router::core::flow_router=debug
```

This deployment guide provides comprehensive instructions for deploying Click Router in various environments with proper configuration and monitoring.

