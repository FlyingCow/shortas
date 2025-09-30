---
layout: vector-theme
title: Deployment Guide
permalink: /deployment/
---

<div class="hero-section">
  <h1>Deployment Guide</h1>
  <p class="lead">This guide covers deployment strategies, configuration, and operational considerations for Shortas in various environments.</p>
</div>

## 🚀 Deployment Options

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

-   **Rust**: Version 1.75+ (stable)
-   **Docker & Docker Compose**: For containerized deployments
-   **Make**: Build automation tool
-   **curl**: For health checks and API testing

## 🚀 Quick Start Deployment

For a rapid local deployment, use the `make` commands from the project root:

```bash
# Complete development setup (installs deps, starts infra, builds, tests)
make dev-setup

# Start all services in development mode
make dev-start

# Check if all services are healthy
make health-check
```

## 🐳 Docker Deployment

Docker is the recommended way to run Shortas in development and staging environments.

### Building Docker Images

From the project root, build all service images:
```bash
make build-docker
```
This will create images for `click-router`, `click-tracker`, `click-aggregator`, `click-router-api`, and `click-aggregator-api`.

### Deploying with Docker Compose

The `docker-compose.yml` file in the `redirect/` directory orchestrates all Shortas services and their dependencies (MongoDB, ClickHouse, Redis, Kafka/Fluvio).

1.  **Navigate to the `redirect` directory**:
    ```bash
    cd redirect
    ```

2.  **Start the services**:
    ```bash
    docker-compose up -d
    ```
    This command will pull necessary images, create containers, and start all services in detached mode.

3.  **Verify services**:
    ```bash
    docker-compose ps
    # Or use the project's health check
    cd .. # Go back to root
    make health-check
    ```

4.  **Stop services**:
    ```bash
    cd redirect
    docker-compose down
    ```

## ☸️ Kubernetes Deployment

For production environments, Kubernetes provides robust orchestration, scaling, and self-healing capabilities. Example manifests are provided in `infra/aws/terraform`.

### Prerequisites

-   A running Kubernetes cluster.
-   `kubectl` configured to connect to your cluster.
-   Docker images pushed to a registry accessible by your cluster.

### Deployment Steps

1.  **Create Namespace**:
    ```bash
    kubectl apply -f infra/aws/terraform/namespace.yaml
    ```

2.  **Create ConfigMaps**:
    Define application configurations as ConfigMaps.
    ```bash
    kubectl apply -f infra/aws/terraform/click-router-configmap.yaml
    # Repeat for other services (click-tracker, click-aggregator, etc.)
    ```

3.  **Deploy Databases**:
    Deploy MongoDB, ClickHouse, and Redis using their respective Kubernetes manifests or Helm charts. Ensure persistent storage is configured.

4.  **Deploy Message Queues**:
    Deploy Kafka/Fluvio using their Kubernetes manifests or Helm charts.

5.  **Deploy Shortas Services**:
    Apply the deployment manifests for each Shortas microservice.
    ```bash
    kubectl apply -f infra/aws/terraform/click-router-deployment.yaml
    kubectl apply -f infra/aws/terraform/click-router-service.yaml
    kubectl apply -f infra/aws/terraform/click-router-ingress.yaml
    # Repeat for other services
    ```

6.  **Monitor Deployment**:
    ```bash
    kubectl get pods -n shortas
    kubectl get services -n shortas
    kubectl get ingress -n shortas
    ```

## ☁️ AWS Production Deployment

Shortas can be deployed on AWS using Terraform for infrastructure as code. The `infra/aws/terraform` directory contains example Terraform configurations.

### Prerequisites

-   AWS account configured with necessary permissions.
-   Terraform CLI installed.

### Deployment Steps

1.  **Initialize Terraform**:
    ```bash
    cd infra/aws/terraform
    terraform init
    ```

2.  **Review and Plan**:
    ```bash
    terraform plan
    ```

3.  **Apply Terraform**:
    ```bash
    terraform apply
    ```
    This will provision AWS resources including:
    -   EKS Cluster (Kubernetes)
    -   EC2 instances for services
    -   RDS for MongoDB (or DynamoDB tables)
    -   MSK for Kafka (or Fluvio on EC2)
    -   ElastiCache for Redis
    -   Load Balancers, Security Groups, etc.

## ⚙️ Configuration Management

Refer to the [Configuration Guide](../getting-started/configuration.md) for detailed information on environment variables and TOML configuration files.

## 🗄️ Database Setup

Ensure your databases are properly set up and accessible by Shortas services.

### MongoDB
-   **Local**: Use `docker-compose` or manual installation.
-   **Production**: AWS DocumentDB, MongoDB Atlas, or self-hosted cluster.
-   [MongoDB Setup Details](deployment.md#mongodb-setup)

### ClickHouse
-   **Local**: Use `docker-compose` or manual installation.
-   **Production**: AWS EC2, ClickHouse Cloud, or self-hosted cluster.
-   [ClickHouse Setup Details](deployment.md#clickhouse-setup)

### Redis
-   **Local**: Use `docker-compose` or manual installation.
-   **Production**: AWS ElastiCache or self-hosted Redis cluster.

## 📊 Analytics Setup

Configure your message queues for click stream processing.

### Kafka
-   **Local**: Use `docker-compose`.
-   **Production**: AWS MSK or self-hosted Kafka cluster.
-   [Kafka Setup Details](deployment.md#kafka-setup)

### Fluvio
-   **Local**: Use `docker-compose` or `fluvio cluster start`.
-   **Production**: Fluvio on EC2 or other cloud instances.
-   [Fluvio Setup Details](deployment.md#fluvio-setup)

## 🔒 Security Configuration

Implement robust security measures for your deployments.

### TLS/SSL
-   Configure SSL certificates for all public-facing services.
-   Use Certbot for Let's Encrypt certificates in production.
-   [TLS/SSL Setup Details](deployment.md#tls-ssl-setup)

### Firewall Configuration
-   Restrict access to necessary ports only.
-   Use security groups (AWS) or network policies (Kubernetes).

### Authentication & Authorization
-   Integrate JWT with Keycloak or other identity providers.
-   Implement role-based access control (RBAC).

## 📈 Monitoring & Logging

Set up comprehensive monitoring and logging for observability.

### Health Checks
-   All services expose `/health` and `/metrics` endpoints.
-   Integrate with Prometheus and Grafana.
-   [Health Checks Details](deployment.md#health-checks)

### Logging
-   Structured JSON logging.
-   Centralized logging with ELK stack or AWS CloudWatch.
-   [Logging Configuration Details](deployment.md#logging-configuration)

## 🚀 Performance Optimization

Tune your environment and application for maximum performance.

### System Tuning
-   Adjust Linux kernel parameters (`sysctl.conf`).
-   Increase file descriptor limits (`limits.conf`).

### Application Tuning
-   Configure thread pools, cache sizes, and database connection pools.
-   [Performance Optimization Details](deployment.md#performance-optimization)

## 🔄 Backup & Recovery

Implement backup strategies for critical data.

### Database Backup
-   Regular backups for MongoDB (mongodump) and ClickHouse.
-   AWS DynamoDB backups.
-   [Database Backup Details](deployment.md#database-backup)

### Application Backup
-   Backup configuration files and static data.

## 🚨 Troubleshooting

Common issues and debugging tips.

-   [Troubleshooting Guide](deployment.md#troubleshooting)
-   Enable debug logging (`RUST_LOG=debug`).

---

**Need help with deployment?** Check the [Issue Tracker](https://github.com/FlyingCow/shortas/issues) or our [Support](#support) section.
