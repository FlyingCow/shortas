---
layout: page
title: Deployment Guide
permalink: /deployment/
---

<div class="hero-section">
  <h1>Deployment Guide</h1>
  <p class="lead">This guide covers deployment strategies, configuration, and operational considerations for Shortas in various environments.</p>
</div>

<div class="feature-grid">
  <div class="feature-card">
    <div class="feature-icon">💻</div>
    <h3>Local Development</h3>
    <p>Development environment setup with all necessary tools and dependencies.</p>
    <a href="local/" class="btn">Learn More</a>
  </div>
  
  <div class="feature-card">
    <div class="feature-icon">🐳</div>
    <h3>Docker Deployment</h3>
    <p>Containerized deployment with Docker and Docker Compose.</p>
    <a href="docker/" class="btn">Learn More</a>
  </div>
  
  <div class="feature-card">
    <div class="feature-icon">☸️</div>
    <h3>Kubernetes</h3>
    <p>Container orchestration for production-scale deployments.</p>
    <a href="kubernetes/" class="btn">Learn More</a>
  </div>
  
  <div class="feature-card">
    <div class="feature-icon">☁️</div>
    <h3>AWS Production</h3>
    <p>Cloud deployment with AWS services and infrastructure.</p>
    <a href="aws/" class="btn">Learn More</a>
  </div>
</div>

## 📋 Prerequisites

### System Requirements

- **Operating System**: Linux, macOS, or Windows (with WSL2)
- **Memory**: Minimum 4GB RAM (8GB recommended for production)
- **Storage**: 10GB free space (50GB+ for production)
- **Network**: Internet connection for downloading dependencies

### Required Software

- **Rust**: 1.75+ (stable channel)
- **Docker**: 20.10+ with Docker Compose
- **Make**: GNU Make 4.0+
- **Git**: 2.30+
- **curl**: 7.68+

## 🐳 Docker Deployment

### Quick Start

```bash
# Clone the repository
git clone https://github.com/FlyingCow/shortas.git
cd shortas

# Build all Docker images
make build-docker

# Deploy with Docker Compose
make deploy-docker

# Check health
make health-check
```

### Docker Compose Configuration

```yaml
version: '3.8'

services:
  click-router:
    build: .
    ports:
      - "8080:8080"
    environment:
      - APP_RUN_MODE=production
      - MONGODB_URI=mongodb://mongodb:27017/shortas
      - REDIS_URL=redis://redis:6379
      - CLICKHOUSE_URL=http://clickhouse:8123
    depends_on:
      - mongodb
      - redis
      - clickhouse
    restart: unless-stopped

  click-router-api:
    build: ./redirect/click-router-api
    ports:
      - "8081:8080"
    environment:
      - MONGODB_URI=mongodb://mongodb:27017/shortas
      - KEYCLOAK_URL=http://keycloak:8080
    depends_on:
      - mongodb
      - keycloak
    restart: unless-stopped

  click-aggregator-api:
    build: ./redirect/click-aggregator-api
    ports:
      - "8082:8080"
    environment:
      - CLICKHOUSE_URL=http://clickhouse:8123
      - KEYCLOAK_URL=http://keycloak:8080
    depends_on:
      - clickhouse
      - keycloak
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
    restart: unless-stopped

  clickhouse:
    image: clickhouse/clickhouse-server:latest
    ports:
      - "8123:8123"
    volumes:
      - clickhouse_data:/var/lib/clickhouse
    environment:
      - CLICKHOUSE_DB=shortas
      - CLICKHOUSE_USER=default
      - CLICKHOUSE_PASSWORD=clickhouse
    restart: unless-stopped

  redis:
    image: redis:latest
    ports:
      - "6379:6379"
    volumes:
      - redis_data:/data
    command: redis-server --requirepass eYVX7EwVmmxKPCDmwMtyKVge8oLd2t81
    restart: unless-stopped

  keycloak:
    image: quay.io/keycloak/keycloak:latest
    ports:
      - "8080:8080"
    environment:
      - KEYCLOAK_ADMIN=admin
      - KEYCLOAK_ADMIN_PASSWORD=admin
    command: start-dev
    restart: unless-stopped

volumes:
  mongodb_data:
  clickhouse_data:
  redis_data:
```

## ☸️ Kubernetes Deployment

### Namespace

```yaml
apiVersion: v1
kind: Namespace
metadata:
  name: shortas
```

### ConfigMap

```yaml
apiVersion: v1
kind: ConfigMap
metadata:
  name: shortas-config
  namespace: shortas
data:
  default.toml: |
    [server]
    threads = 8
    listen_os_signals = true
    exit = true
    
    [mongodb]
    uri = "mongodb://mongodb:27017/shortas"
    
    [clickhouse]
    url = "http://clickhouse:8123"
    user = "default"
    password = "clickhouse"
    database = "shortas"
    
    [redis]
    url = "redis://redis:6379"
    password = "eYVX7EwVmmxKPCDmwMtyKVge8oLd2t81"
```

### Click Router Deployment

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: click-router
  namespace: shortas
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
        image: shortas/click-router:latest
        ports:
        - containerPort: 8080
        env:
        - name: APP_RUN_MODE
          value: "production"
        - name: APP_CONFIG_PATH
          value: "/app/config"
        volumeMounts:
        - name: config
          mountPath: /app/config
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
            port: 8080
          initialDelaySeconds: 30
          periodSeconds: 10
        readinessProbe:
          httpGet:
            path: /health
            port: 8080
          initialDelaySeconds: 5
          periodSeconds: 5
      volumes:
      - name: config
        configMap:
          name: shortas-config
```

### Service

```yaml
apiVersion: v1
kind: Service
metadata:
  name: click-router-service
  namespace: shortas
spec:
  selector:
    app: click-router
  ports:
  - port: 80
    targetPort: 8080
  type: LoadBalancer
```

### Ingress

```yaml
apiVersion: networking.k8s.io/v1
kind: Ingress
metadata:
  name: shortas-ingress
  namespace: shortas
  annotations:
    nginx.ingress.kubernetes.io/rewrite-target: /
    cert-manager.io/cluster-issuer: "letsencrypt-prod"
spec:
  tls:
  - hosts:
    - short.ly
    - www.short.ly
    secretName: shortas-tls
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

## ☁️ AWS Production Deployment

### Infrastructure Setup

```bash
# Initialize Terraform
cd infra/aws/terraform
terraform init

# Plan deployment
terraform plan

# Apply configuration
terraform apply
```

### Terraform Configuration

```hcl
# main.tf
provider "aws" {
  region = var.aws_region
}

# VPC
resource "aws_vpc" "shortas_vpc" {
  cidr_block           = "10.0.0.0/16"
  enable_dns_hostnames = true
  enable_dns_support   = true

  tags = {
    Name = "shortas-vpc"
  }
}

# EKS Cluster
resource "aws_eks_cluster" "shortas_cluster" {
  name     = "shortas-cluster"
  role_arn = aws_iam_role.eks_cluster.arn
  version  = "1.28"

  vpc_config {
    subnet_ids = aws_subnet.shortas_subnets[*].id
  }

  depends_on = [
    aws_iam_role_policy_attachment.eks_cluster_policy,
    aws_iam_role_policy_attachment.eks_vpc_resource_controller,
  ]
}

# RDS for MongoDB (DocumentDB)
resource "aws_docdb_cluster" "shortas_mongodb" {
  cluster_identifier      = "shortas-mongodb"
  engine                 = "docdb"
  master_username        = "shortas"
  master_password        = var.mongodb_password
  backup_retention_period = 7
  preferred_backup_window = "07:00-09:00"
  skip_final_snapshot    = true
}

# ElastiCache for Redis
resource "aws_elasticache_replication_group" "shortas_redis" {
  replication_group_id         = "shortas-redis"
  description                  = "Redis cluster for Shortas"
  node_type                   = "cache.t3.micro"
  port                        = 6379
  parameter_group_name        = "default.redis7"
  num_cache_clusters          = 2
  automatic_failover_enabled  = true
  multi_az_enabled           = true
}
```

## 🔧 Configuration Management

### Environment Variables

```bash
# Production environment
export APP_RUN_MODE=production
export MONGODB_URI=mongodb://prod-cluster:27017/shortas_prod
export CLICKHOUSE_URL=http://clickhouse-prod:8123
export REDIS_URL=redis://redis-cluster:6379
export KEYCLOAK_URL=https://keycloak.example.com
```

### Secrets Management

#### Kubernetes Secrets

```yaml
apiVersion: v1
kind: Secret
metadata:
  name: shortas-secrets
  namespace: shortas
type: Opaque
data:
  mongodb-password: <base64-encoded-password>
  redis-password: <base64-encoded-password>
  keycloak-secret: <base64-encoded-secret>
```

#### AWS Secrets Manager

```bash
# Store secrets
aws secretsmanager create-secret \
  --name "shortas/mongodb" \
  --description "MongoDB connection string" \
  --secret-string "mongodb://user:password@cluster:27017/shortas"

# Retrieve secrets
aws secretsmanager get-secret-value \
  --secret-id "shortas/mongodb" \
  --query SecretString --output text
```

## 📊 Monitoring & Observability

### Prometheus Configuration

```yaml
# prometheus.yml
global:
  scrape_interval: 15s

scrape_configs:
  - job_name: 'shortas'
    static_configs:
      - targets: ['click-router:8080', 'click-router-api:8080', 'click-aggregator-api:8080']
    metrics_path: /metrics
    scrape_interval: 5s
```

### Grafana Dashboard

```json
{
  "dashboard": {
    "title": "Shortas Metrics",
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

### Logging Configuration

```yaml
# fluentd-config.yaml
apiVersion: v1
kind: ConfigMap
metadata:
  name: fluentd-config
data:
  fluent.conf: |
    <source>
      @type tail
      path /var/log/containers/*shortas*.log
      pos_file /var/log/fluentd-containers.log.pos
      tag kubernetes.*
      format json
    </source>
    
    <match kubernetes.**>
      @type elasticsearch
      host elasticsearch.logging.svc.cluster.local
      port 9200
      index_name shortas
    </match>
```

## 🔒 Security Configuration

### TLS/SSL Setup

```bash
# Generate certificates
openssl req -x509 -newkey rsa:4096 -keyout key.pem -out cert.pem -days 365 -nodes

# Configure TLS in Kubernetes
apiVersion: v1
kind: Secret
metadata:
  name: shortas-tls
type: kubernetes.io/tls
data:
  tls.crt: <base64-encoded-cert>
  tls.key: <base64-encoded-key>
```

### Network Policies

```yaml
apiVersion: networking.k8s.io/v1
kind: NetworkPolicy
metadata:
  name: shortas-network-policy
  namespace: shortas
spec:
  podSelector:
    matchLabels:
      app: shortas
  policyTypes:
  - Ingress
  - Egress
  ingress:
  - from:
    - namespaceSelector:
        matchLabels:
          name: ingress-nginx
  egress:
  - to:
    - namespaceSelector:
        matchLabels:
          name: database
```

## 🚀 Performance Optimization

### Resource Limits

```yaml
resources:
  requests:
    memory: "256Mi"
    cpu: "250m"
  limits:
    memory: "512Mi"
    cpu: "500m"
```

### Horizontal Pod Autoscaler

```yaml
apiVersion: autoscaling/v2
kind: HorizontalPodAutoscaler
metadata:
  name: click-router-hpa
  namespace: shortas
spec:
  scaleTargetRef:
    apiVersion: apps/v1
    kind: Deployment
    name: click-router
  minReplicas: 3
  maxReplicas: 10
  metrics:
  - type: Resource
    resource:
      name: cpu
      target:
        type: Utilization
        averageUtilization: 70
```

## 🔄 Backup & Recovery

### Database Backup

```bash
# MongoDB backup
mongodump --host mongodb:27017 --db shortas --out /backup/mongodb/

# ClickHouse backup
clickhouse-backup create --config /etc/clickhouse-backup/config.yml
```

### Application Backup

```bash
# Backup configuration
kubectl get configmap shortas-config -o yaml > shortas-config-backup.yaml

# Backup secrets
kubectl get secret shortas-secrets -o yaml > shortas-secrets-backup.yaml
```

## 🚨 Troubleshooting

### Common Issues

**Service not starting:**
```bash
# Check pod status
kubectl get pods -n shortas

# Check logs
kubectl logs -n shortas deployment/click-router

# Check events
kubectl get events -n shortas
```

**Database connection issues:**
```bash
# Test MongoDB connection
kubectl exec -it deployment/mongodb -- mongosh "mongodb://localhost:27017/shortas"

# Test ClickHouse connection
kubectl exec -it deployment/clickhouse -- curl http://localhost:8123/ping
```

**Performance issues:**
```bash
# Check resource usage
kubectl top pods -n shortas

# Check metrics
kubectl port-forward svc/prometheus 9090:9090
```

## 📚 Additional Resources

- [Local Development](local/) - Development environment setup
- [Docker Deployment](docker/) - Containerized deployment
- [Kubernetes](kubernetes/) - Container orchestration
- [AWS Production](aws/) - Cloud deployment
- [Troubleshooting](troubleshooting/) - Common issues and solutions

---

**Need help with deployment?** Check our [troubleshooting guide](troubleshooting/) or [open an issue](https://github.com/FlyingCow/shortas/issues) on GitHub.
