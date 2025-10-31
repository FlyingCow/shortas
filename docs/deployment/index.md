---
layout: vector-theme
title: Deployment Guide
permalink: /deployment/
---

<div class="hero-section">
  <h1>Deployment Guide</h1>
  <p class="lead">This guide covers deployment strategies, configuration, and operational considerations for Shortas in various environments. From local development to production-scale deployments on AWS.</p>
</div>

## 🚀 Deployment Options

Shortas can be deployed in multiple environments. Choose the option that best fits your needs:

<div class="feature-grid">
  <div class="feature-card">
    <div class="feature-icon">💻</div>
    <h3>Local Development</h3>
    <p>Development environment setup with all necessary tools and dependencies. Perfect for development and testing.</p>
    <ul>
      <li>Docker Compose</li>
      <li>All services in one stack</li>
      <li>Easy debugging</li>
    </ul>
    <a href="#local-development" class="btn">Learn More</a>
  </div>
  
  <div class="feature-card">
    <div class="feature-icon">🐳</div>
    <h3>Docker Deployment</h3>
    <p>Containerized deployment with Docker and Docker Compose. Ideal for staging and small production deployments.</p>
    <ul>
      <li>Container orchestration</li>
      <li>Easy scaling</li>
      <li>Isolated environments</li>
    </ul>
    <a href="#docker-deployment" class="btn">Learn More</a>
  </div>
  
  <div class="feature-card">
    <div class="feature-icon">☸️</div>
    <h3>Kubernetes</h3>
    <p>Container orchestration for production-scale deployments. Provides auto-scaling, self-healing, and advanced networking.</p>
    <ul>
      <li>Auto-scaling</li>
      <li>Self-healing</li>
      <li>Service mesh</li>
    </ul>
    <a href="#kubernetes-deployment" class="btn">Learn More</a>
  </div>
  
  <div class="feature-card">
    <div class="feature-icon">☁️</div>
    <h3>AWS Production</h3>
    <p>Cloud deployment with AWS services and infrastructure. Enterprise-grade scalability and reliability.</p>
    <ul>
      <li>Auto-scaling groups</li>
      <li>Load balancers</li>
      <li>Managed services</li>
    </ul>
    <a href="#aws-production-deployment" class="btn">Learn More</a>
  </div>
</div>

## 📋 Prerequisites

<div class="card">
  <div class="card-header">System Requirements</div>
  <ul>
    <li><strong>Rust:</strong> Version 1.75+ (stable)</li>
    <li><strong>Docker & Docker Compose:</strong> For containerized deployments</li>
    <li><strong>Make:</strong> Build automation tool</li>
    <li><strong>curl:</strong> For health checks and API testing</li>
    <li><strong>Kubernetes CLI (kubectl):</strong> For Kubernetes deployments</li>
    <li><strong>Terraform:</strong> For infrastructure as code (AWS)</li>
  </ul>
</div>

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

<div class="card">
  <div class="card-header">Deployment Steps</div>
  <ol>
    <li><strong>Navigate to the redirect directory:</strong>
      <pre><code>cd redirect</code></pre>
    </li>
    <li><strong>Start the services:</strong>
      <pre><code>docker-compose up -d</code></pre>
      <p>This command will pull necessary images, create containers, and start all services in detached mode.</p>
    </li>
    <li><strong>Verify services:</strong>
      <pre><code>docker-compose ps
# Or use the project's health check
cd .. # Go back to root
make health-check</code></pre>
    </li>
    <li><strong>Stop services:</strong>
      <pre><code>cd redirect
docker-compose down</code></pre>
    </li>
  </ol>
</div>

## ☸️ Kubernetes Deployment

For production environments, Kubernetes provides robust orchestration, scaling, and self-healing capabilities. Example manifests are provided in `infra/aws/terraform`.

### Prerequisites

<div class="card">
  <div class="card-header">Kubernetes Prerequisites</div>
  <ul>
    <li>A running Kubernetes cluster</li>
    <li><code>kubectl</code> configured to connect to your cluster</li>
    <li>Docker images pushed to a registry accessible by your cluster</li>
  </ul>
</div>

### Deployment Steps

<div class="card">
  <div class="card-header">Kubernetes Deployment Process</div>
  <ol>
    <li><strong>Create Namespace:</strong>
      <pre><code>kubectl apply -f infra/aws/terraform/namespace.yaml</code></pre>
    </li>
    <li><strong>Create ConfigMaps:</strong>
      <p>Define application configurations as ConfigMaps.</p>
      <pre><code>kubectl apply -f infra/aws/terraform/click-router-configmap.yaml
# Repeat for other services (click-tracker, click-aggregator, etc.)</code></pre>
    </li>
    <li><strong>Deploy Databases:</strong>
      <p>Deploy MongoDB, ClickHouse, and Redis using their respective Kubernetes manifests or Helm charts. Ensure persistent storage is configured.</p>
    </li>
    <li><strong>Deploy Message Queues:</strong>
      <p>Deploy Kafka/Fluvio using their Kubernetes manifests or Helm charts.</p>
    </li>
    <li><strong>Deploy Shortas Services:</strong>
      <p>Apply the deployment manifests for each Shortas microservice.</p>
      <pre><code>kubectl apply -f infra/aws/terraform/click-router-deployment.yaml
kubectl apply -f infra/aws/terraform/click-router-service.yaml
kubectl apply -f infra/aws/terraform/click-router-ingress.yaml
# Repeat for other services</code></pre>
    </li>
    <li><strong>Monitor Deployment:</strong>
      <pre><code>kubectl get pods -n shortas
kubectl get services -n shortas
kubectl get ingress -n shortas</code></pre>
    </li>
  </ol>
</div>

## ☁️ AWS Production Deployment

Shortas can be deployed on AWS using Terraform for infrastructure as code. The `infra/aws/terraform` directory contains example Terraform configurations.

### Prerequisites

<div class="card">
  <div class="card-header">AWS Prerequisites</div>
  <ul>
    <li>AWS account configured with necessary permissions</li>
    <li>Terraform CLI installed</li>
    <li>AWS CLI configured with credentials</li>
  </ul>
</div>

### Deployment Steps

<div class="card">
  <div class="card-header">AWS Deployment Process</div>
  <ol>
    <li><strong>Initialize Terraform:</strong>
      <pre><code>cd infra/aws/terraform
terraform init</code></pre>
    </li>
    <li><strong>Review and Plan:</strong>
      <pre><code>terraform plan</code></pre>
    </li>
    <li><strong>Apply Terraform:</strong>
      <pre><code>terraform apply</code></pre>
      <p>This will provision AWS resources including:</p>
      <ul>
        <li>EKS Cluster (Kubernetes)</li>
        <li>EC2 instances for services</li>
        <li>RDS for MongoDB (or DynamoDB tables)</li>
        <li>MSK for Kafka (or Fluvio on EC2)</li>
        <li>ElastiCache for Redis</li>
        <li>Load Balancers, Security Groups, etc.</li>
      </ul>
    </li>
  </ol>
</div>

## ⚙️ Configuration Management

Refer to the [Configuration Guide](../getting-started/) for detailed information on environment variables and TOML configuration files.

<div class="card">
  <div class="card-header">Configuration Files</div>
  <ul>
    <li><code>config/default.toml</code> - Base configuration</li>
    <li><code>config/development.toml</code> - Development overrides</li>
    <li><code>config/production.toml</code> - Production settings</li>
    <li><code>config/test.toml</code> - Test configuration</li>
  </ul>
</div>

## 🗄️ Database Setup

Ensure your databases are properly set up and accessible by Shortas services.

<div class="feature-grid">
  <div class="card">
    <div class="card-header">MongoDB</div>
    <ul>
      <li><strong>Local:</strong> Use <code>docker-compose</code> or manual installation</li>
      <li><strong>Production:</strong> AWS DocumentDB, MongoDB Atlas, or self-hosted cluster</li>
    </ul>
  </div>
  
  <div class="card">
    <div class="card-header">ClickHouse</div>
    <ul>
      <li><strong>Local:</strong> Use <code>docker-compose</code> or manual installation</li>
      <li><strong>Production:</strong> AWS EC2, ClickHouse Cloud, or self-hosted cluster</li>
    </ul>
  </div>
  
  <div class="card">
    <div class="card-header">Redis</div>
    <ul>
      <li><strong>Local:</strong> Use <code>docker-compose</code> or manual installation</li>
      <li><strong>Production:</strong> AWS ElastiCache or self-hosted Redis cluster</li>
    </ul>
  </div>
</div>

## 📊 Analytics Setup

Configure your message queues for click stream processing.

<div class="feature-grid">
  <div class="card">
    <div class="card-header">Kafka</div>
    <ul>
      <li><strong>Local:</strong> Use <code>docker-compose</code></li>
      <li><strong>Production:</strong> AWS MSK or self-hosted Kafka cluster</li>
    </ul>
  </div>
  
  <div class="card">
    <div class="card-header">Fluvio</div>
    <ul>
      <li><strong>Local:</strong> Use <code>docker-compose</code> or <code>fluvio cluster start</code></li>
      <li><strong>Production:</strong> Fluvio on EC2 or other cloud instances</li>
    </ul>
  </div>
</div>

## 🔒 Security Configuration

Implement robust security measures for your deployments.

<div class="card">
  <div class="card-header">Security Checklist</div>
  <ul>
    <li><strong>TLS/SSL:</strong> Configure SSL certificates for all public-facing services. Use Certbot for Let's Encrypt certificates in production.</li>
    <li><strong>Firewall Configuration:</strong> Restrict access to necessary ports only. Use security groups (AWS) or network policies (Kubernetes).</li>
    <li><strong>Authentication & Authorization:</strong> Integrate JWT with Keycloak or other identity providers. Implement role-based access control (RBAC).</li>
    <li><strong>Secrets Management:</strong> Use Kubernetes secrets, AWS Secrets Manager, or HashiCorp Vault for sensitive data.</li>
    <li><strong>Network Policies:</strong> Implement network segmentation and access controls.</li>
  </ul>
</div>

## 📈 Monitoring & Logging

Set up comprehensive monitoring and logging for observability.

<div class="feature-grid">
  <div class="card">
    <div class="card-header">Health Checks</div>
    <ul>
      <li>All services expose <code>/health</code> and <code>/metrics</code> endpoints</li>
      <li>Integrate with Prometheus and Grafana</li>
      <li>Set up alerting for service downtime</li>
    </ul>
  </div>
  
  <div class="card">
    <div class="card-header">Logging</div>
    <ul>
      <li>Structured JSON logging</li>
      <li>Centralized logging with ELK stack or AWS CloudWatch</li>
      <li>Log aggregation and analysis</li>
    </ul>
  </div>
</div>

## 🚀 Performance Optimization

Tune your environment and application for maximum performance.

<div class="card">
  <div class="card-header">Optimization Tips</div>
  <ul>
    <li><strong>System Tuning:</strong> Adjust Linux kernel parameters (<code>sysctl.conf</code>). Increase file descriptor limits (<code>limits.conf</code>).</li>
    <li><strong>Application Tuning:</strong> Configure thread pools, cache sizes, and database connection pools.</li>
    <li><strong>Database Optimization:</strong> Index optimization, connection pooling, query optimization.</li>
    <li><strong>Caching Strategy:</strong> Multi-level caching with Redis and Moka for optimal performance.</li>
  </ul>
</div>

## 🔄 Backup & Recovery

Implement backup strategies for critical data.

<div class="card">
  <div class="card-header">Backup Strategy</div>
  <ul>
    <li><strong>Database Backup:</strong> Regular backups for MongoDB (mongodump) and ClickHouse. AWS DynamoDB backups.</li>
    <li><strong>Application Backup:</strong> Backup configuration files and static data.</li>
    <li><strong>Disaster Recovery:</strong> Document recovery procedures and test regularly.</li>
    <li><strong>Backup Storage:</strong> Use durable, off-site storage for backups.</li>
  </ul>
</div>

## 🚨 Troubleshooting

Common issues and debugging tips.

<div class="card">
  <div class="card-header">Common Issues</div>
  <ul>
    <li><strong>Service Won't Start:</strong> Check Docker is running, verify ports are not in use, check service logs.</li>
    <li><strong>Database Connection Issues:</strong> Verify database is running, check connection strings, test connectivity.</li>
    <li><strong>High Latency:</strong> Check network latency, database query performance, cache hit rates.</li>
    <li><strong>Memory Issues:</strong> Monitor memory usage, adjust cache sizes, optimize data structures.</li>
  </ul>
</div>

<div class="alert alert-info">
  <strong>Debug Logging:</strong> Enable debug logging (<code>RUST_LOG=debug</code>) for detailed troubleshooting information.
</div>

---

**Need help with deployment?** Check the [Issue Tracker](https://github.com/FlyingCow/shortas/issues) or our [Support](#support) section.
