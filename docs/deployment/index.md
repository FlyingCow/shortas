---
layout: professional-theme
title: Deployment
permalink: /deployment/
---

# Deployment

## Docker Compose

The recommended deployment method runs all services as Docker containers, orchestrated from a single top-level compose file.

### Full Stack

```bash
cd infra/custom
docker compose up -d
```

`infra/custom/docker-compose.yml` is the real entry point. It `include`s three other compose files and additionally defines a few shared services directly:

- **Included — `redirect/docker-compose.yml`**: MongoDB, ClickHouse, Redis, MinIO topic setup, Fluvio (SC + SPU), gglsbl-rest (Safe Browsing), and all the redirect application services (click-router, click-router-api, click-tracker, click-aggregator, click-aggregator-api, route-verifier, route-icon-worker, domain-verifier, cert-bot, pages). This file itself includes `redirect/monitoring/docker-compose.yml` (Prometheus, Grafana, Loki).
- **Included — `management-api/docker-compose.yml`**: a dedicated PostgreSQL instance, Elasticsearch, and the `management-api` container.
- **Included — `ui/docker-compose.yml`**: the dashboard and landing page containers.
- **Defined directly**: Keycloak (+ its own Postgres DB), HashiCorp Vault (dev mode), RabbitMQ, and MinIO/RustFS (+ bucket/CORS setup and an nginx reverse proxy in front of it for presigned-URL CORS handling).

Init containers run automatically: MinIO bucket + CORS setup, Fluvio topic creation, ClickHouse migrations.

### Running a Subset

Each compose file can also be run on its own (they share the external `shortas-net` network once it exists):

```bash
# Just the Rust redirect stack + monitoring
cd redirect && docker compose up -d

# Just the Management API + its Postgres/Elasticsearch
cd management-api && docker compose up -d

# Just the dashboard + landing page
cd ui && docker compose up -d
```

## Service Dependencies

```
click-router         → mongo, redis, clickhouse, fluvio (topics), rabbitmq, pages
click-router-api      → mongo, redis, rabbitmq
click-tracker         → redis, fluvio (topics)
click-aggregator      → clickhouse, fluvio (topics)
click-aggregator-api  → clickhouse (after migrations)
route-verifier         → mongo, click-router-api, rabbitmq, safe-browsing
route-icon-worker      → rabbitmq, minio
domain-verifier        → mongo, rabbitmq
cert-bot               → mongo, click-router-api
pages                  → (standalone, static)
management-api         → postgres, elasticsearch, rabbitmq, minio, click-router-api, click-aggregator-api, keycloak
dashboard              → management-api, keycloak, minio-nginx
```

## Environment Configuration

### Rust Services (including Management API)

Configuration is loaded from TOML files in each service's `config/` directory, overridable by `APP__<SECTION>__<KEY>` environment variables. The `APP_RUN_MODE` environment variable selects the profile:

- `development` — local defaults with debug logging
- `production` — production-optimized settings

Docker containers default to `APP_RUN_MODE=production` (except Management API's compose file, which sets `development` for its local setup).

Example (Management API, from its `docker-compose.yml`):

```bash
APP__DATABASE__HOST=shortas-api-postgres
APP__JWT__JWKS_URL=http://keycloak:8080/realms/shortas-dev/protocol/openid-connect/certs
APP__CLICK_ROUTER__BASE_URL=http://shortas-click-router-api:5810
APP__CLICK_AGGREGATOR__BASE_URL=http://shortas-click-aggregator-api:5820
APP__MINIO__PUBLIC_ENDPOINT=http://localhost:9003
APP__RABBITMQ__URL=amqp://guest:guest@shortas-rabbitmq:5672
```

## Docker Images

Multi-stage builds minimize image sizes:

- **Rust services** (including `management-api`): built from the repo-root Dockerfile context, producing a Debian slim image with a non-root user
- **UI apps**: Node-based builds outputting static assets served by nginx

The legacy `api/` (ASP.NET Core) project has its own three-stage Dockerfile targeting `mcr.microsoft.com/dotnet/aspnet:9.0`, but it is not part of the active deployment.

## Health Checks

Most services expose health endpoints, and Docker Compose healthchecks monitor them automatically:

```bash
make ps    # Show redirect/ service status and health
make logs  # Tail redirect/ service logs
```

## Networking

All services share the `shortas-net` Docker bridge network (created by `infra/custom/docker-compose.yml`, consumed as `external: true` by the included files). Inter-service communication uses container hostnames (e.g., `mongo`, `clickhouse`, `shortas-click-router-api`, `keycloak`).

## AWS Deployment

For production deployments, Shortas can be deployed to AWS using Terraform. This replaces local services with AWS managed equivalents for scalability and reliability.

### Service Mapping

| Local Service | AWS Replacement |
|---------------|-----------------|
| MongoDB | DynamoDB |
| PostgreSQL | RDS Aurora PostgreSQL Serverless v2 |
| Redis | ElastiCache Redis |
| MinIO | S3 |
| RabbitMQ | Amazon MQ (RabbitMQ) |
| Keycloak | AWS Cognito |
| ClickHouse | EC2 (self-managed) |
| All services | ECS Fargate |

### Prerequisites

- AWS CLI configured with appropriate credentials
- Terraform >= 1.5.0
- Docker (for building images)
- Domain name with Route53 hosted zone (optional)

### Quick Start

```bash
# Initialize Terraform
cd infra/aws/terraform/environments/dev
terraform init

# Configure variables
cp terraform.tfvars.example terraform.tfvars
# Edit terraform.tfvars with your settings

# Deploy infrastructure
terraform apply

# Build and push Docker images
cd ../scripts
./build-push-images.sh dev

# Deploy services to ECS
./deploy-services.sh dev
```

### Environments

Two environments are provided (17 Terraform modules power both):

| Feature | Dev | Prod |
|---------|-----|------|
| NAT Gateway | Single | Per-AZ |
| RDS Instances | 1 | 2+ (Multi-AZ) |
| Redis Nodes | 1 | 3 (Primary + replicas) |
| ECS Spot | Yes | No |
| Cognito MFA | Optional | Required |
| Backup Retention | 3 days | 14 days |

### Authentication with AWS Cognito

AWS deployments use Cognito instead of Keycloak for authentication:

- User Pool with email-based sign-up/sign-in
- Dashboard app client (public SPA with PKCE)
- API app client (confidential with client credentials)
- Configurable MFA and password policy
- Hosted UI for sign-in/sign-up

Configure the dashboard with Cognito endpoints:

```bash
REACT_APP_COGNITO_ISSUER_URL=https://cognito-idp.us-east-1.amazonaws.com/us-east-1_xxxxxxxx
REACT_APP_COGNITO_CLIENT_ID=xxxxxxxxxxxxxxxxxxxxxxxxxxx
```

Configure the Management API via its `APP__JWT__*` environment variables (or `config/production.toml`):

```bash
APP__JWT__ISSUER=https://cognito-idp.us-east-1.amazonaws.com/us-east-1_xxxxxxxx
APP__JWT__AUDIENCE=shortas-api
APP__JWT__JWKS_URL=https://cognito-idp.us-east-1.amazonaws.com/us-east-1_xxxxxxxx/.well-known/jwks.json
```

### Monitoring

CloudWatch provides:
- Log aggregation for all ECS services
- Custom metrics dashboards
- Alarms for error rates, latency, resource usage
- SNS notifications for critical alerts

### Terraform Outputs

After deployment, retrieve important values:

```bash
terraform output

# Outputs include:
# - cognito_user_pool_id
# - cognito_hosted_ui_url
# - alb_dns_name
# - ecr_repository_urls
# - dynamodb_tables
```

See [infra/aws/terraform/README.md](/infra/aws/terraform/README.md) for detailed documentation.
