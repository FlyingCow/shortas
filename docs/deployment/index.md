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
- **Included — `management-api/docker-compose.yml`**: a dedicated PostgreSQL instance, Elasticsearch, the `management-api` container, and the `notification-worker` container.
- **Included — `ui/docker-compose.yml`**: the dashboard and landing page containers.
- **Defined directly**: Keycloak (+ its own Postgres DB), HashiCorp Vault (dev mode), RabbitMQ, and MinIO/RustFS (+ bucket/CORS setup and an nginx reverse proxy in front of it for presigned-URL CORS handling).

Init containers run automatically: MinIO bucket + CORS setup, Fluvio topic creation, ClickHouse migrations, and a `notification-templates-sync` container that mirrors the repo's `notification-templates/` directory into the MinIO bucket (`aws s3 sync --delete`) before `notification-worker` starts — see [Notification Templates](#notification-templates) below.

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
notification-worker    → postgres, rabbitmq, minio (notification-templates-sync), mailhog/SES
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

## Notification Templates

Notification Worker renders templates it fetches from S3/MinIO at `notification-templates/<notification_id>/{metadata.json, email.subject.hbs, email.body.hbs, in_app.hbs}`, but the templates themselves live in the repo under `notification-templates/` — not baked into any Docker image — so they can be edited and reviewed like code.

- **Local (Docker Compose)**: the `notification-templates-sync` init container (`amazon/aws-cli`, defined in `infra/custom/docker-compose.yml`) mounts the repo's `notification-templates/` directory and runs `aws s3 sync --delete` into the local MinIO bucket on every `docker compose up`, after `minio-setup` creates the bucket and before `notification-worker` starts (`depends_on: condition: service_completed_successfully`).
- **AWS**: `infra/aws/scripts/deploy-services.sh` runs the same `aws s3 sync --delete` (resolving the bucket name from that environment's `s3_route_images_bucket` Terraform output) whenever `notification-worker` is one of the services being deployed — so a normal deploy always leaves S3 matching the deployed commit.

Adding a new notification type is a new `notification-templates/<id>/` directory plus a PR — no Rust or frontend change needed, as long as the type only needs the `email`/`in_app` channels and fields already in its action's render context.

## Docker Images

Multi-stage builds minimize image sizes:

- **Rust services** (including `management-api` and `notification-worker`): built from the repo-root Dockerfile context, producing a Debian slim image with a non-root user. `notification-worker` depends on `management-api` as a Cargo library (not a copy of its code), so both Dockerfiles `COPY` both `management-api/` and `notification-worker/` into the build context regardless of which binary they actually build — Cargo needs every workspace member's `Cargo.toml` present to resolve the workspace at all.
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

Three environments are provided (17 Terraform modules power all three) — `dev`, `prod`, and `budget` (a cost-minimized demo/presentation environment meant to be spun up and torn down rather than run continuously; see its own cost notes in `infra/aws/terraform/environments/budget/main.tf`):

| Feature | Dev | Budget | Prod |
|---------|-----|--------|------|
| NAT Gateway | Single | Single | Per-AZ |
| RDS Instances | 1 | 1 (0.5–2 ACU Serverless v2) | 2+ (Multi-AZ) |
| Redis Nodes | 1 | 1 | 3 (Primary + replicas) |
| ECS Spot | Yes | Yes | No |
| Cognito MFA | Optional | Optional | Required |
| Backup Retention | 3 days | 1 day | 14 days |

`notification-worker` is deployed as an ECS service in all three, sized the same as the other lightweight background workers (256 CPU / 512 MB, no ALB target group — it's a pure RabbitMQ consumer).

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
