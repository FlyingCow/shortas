---
layout: professional-theme
title: Deployment
permalink: /deployment/
---

# Deployment

## Docker Compose

The recommended deployment method runs all services as Docker containers.

### Full Stack

```bash
cd redirect
docker compose up -d
```

This starts:
- **Infrastructure**: MongoDB 7, ClickHouse, Redis 7, MinIO, Fluvio (SC + SPU), RabbitMQ, gglsbl-rest (Safe Browsing)
- **Init containers**: MinIO bucket setup, Fluvio topic creation, ClickHouse migrations
- **Application services**: click-router, click-router-api, click-tracker, click-aggregator, click-aggregator-api, route-verifier, route-icon-worker, domain-verifier, domains

### Management API

```bash
cd api
docker compose up -d
```

Starts the .NET API and its PostgreSQL database.

### UI

```bash
cd ui
docker compose up -d
```

Starts the dashboard and landing page. This compose file includes the redirect compose file via the `include` directive.

## Service Dependencies

```
click-router         → mongo, redis, clickhouse, fluvio (topics), rabbitmq, domains
click-router-api     → mongo, redis, rabbitmq
click-tracker        → redis, fluvio (topics)
click-aggregator     → clickhouse, fluvio (topics)
click-aggregator-api → clickhouse (after migrations)
route-verifier       → mongo, rabbitmq, gglsbl-rest
route-icon-worker    → rabbitmq, minio
domain-verifier      → mongo, rabbitmq
domains              → (standalone)
management-api       → postgresql, elasticsearch, click-router-api, click-aggregator-api
dashboard            → management-api
```

## Environment Configuration

### Rust Services

Configuration is loaded from TOML files in each service's `config/` directory. The `APP_RUN_MODE` environment variable selects the profile:

- `development` — local defaults with debug logging
- `production` — production-optimized settings

Docker containers default to `APP_RUN_MODE=production`.

### Management API

Configured via `appsettings.json` and environment variables:

| Variable | Description |
|----------|-------------|
| `ConnectionStrings__DefaultConnection` | PostgreSQL connection string |
| `Keycloak__Authority` | Keycloak realm URL |
| `Keycloak__Audience` | JWT audience |
| `ApiSettings__ClickRouterApi__BaseUrl` | Click Router API URL |
| `ApiSettings__ClickAggregatorApi__BaseUrl` | Click Aggregator API URL |

## Docker Images

Multi-stage builds minimize image sizes:

- **Rust services**: Built from the workspace Dockerfile, producing a Debian slim image with a non-root user
- **.NET API**: Three-stage build (base, build/publish, runtime) targeting `mcr.microsoft.com/dotnet/aspnet:9.0`
- **UI apps**: Node-based builds outputting static assets

## Health Checks

All services expose health endpoints. Docker Compose healthchecks monitor service status automatically:

```bash
make ps    # Show service status and health
make logs  # Tail service logs
```

## Networking

All services share the `shortas-net` Docker bridge network. Inter-service communication uses container hostnames (e.g., `mongo`, `clickhouse`, `click-router-api`).

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

Two environments are provided:

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
REACT_APP_COGNITO_CLIENT_ID=xxxxxxxxxxxxxxxxxxxxxxxxxx
```

Configure the .NET API with `appsettings.Aws.json`:

```json
{
  "Authentication": {
    "Provider": "Cognito",
    "Cognito": {
      "UserPoolId": "${COGNITO_USER_POOL_ID}",
      "Authority": "${COGNITO_ISSUER_URL}"
    }
  }
}
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
