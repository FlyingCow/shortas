# Shortas AWS Infrastructure

Terraform modules for deploying Shortas URL Shortener to AWS. This infrastructure replaces the local Docker Compose setup with AWS managed services for production-grade deployments.

## Architecture Overview

```
                    ┌─────────────────────────────────────────────────────────┐
                    │                        VPC                              │
                    │  ┌─────────────────────────────────────────────────────┐│
                    │  │                  Public Subnets                     ││
                    │  │  ┌──────────────┐  ┌──────────────┐                ││
Internet ──────────►│  │  │     ALB      │  │ NAT Gateway  │                ││
                    │  │  └──────┬───────┘  └──────────────┘                ││
                    │  └─────────┼───────────────────────────────────────────┘│
                    │            │                                            │
                    │  ┌─────────▼───────────────────────────────────────────┐│
                    │  │                  Private Subnets                    ││
                    │  │  ┌────────────────────────────────────────────────┐ ││
                    │  │  │              ECS Fargate Cluster               │ ││
                    │  │  │  ┌─────────┐ ┌─────────┐ ┌─────────┐          │ ││
                    │  │  │  │ click-  │ │ shortas │ │dashboard│  ...     │ ││
                    │  │  │  │ router  │ │   api   │ │         │          │ ││
                    │  │  │  └─────────┘ └─────────┘ └─────────┘          │ ││
                    │  │  └────────────────────────────────────────────────┘ ││
                    │  │                                                     ││
                    │  │  ┌──────────────┐  ┌──────────────┐                ││
                    │  │  │  ClickHouse  │  │    Fluvio    │                ││
                    │  │  │    (EC2)     │  │   SC + SPU   │                ││
                    │  │  └──────────────┘  └──────────────┘                ││
                    │  └─────────────────────────────────────────────────────┘│
                    │                                                         │
                    │  ┌─────────────────────────────────────────────────────┐│
                    │  │               Database Subnets                      ││
                    │  │  ┌──────────┐ ┌──────────┐ ┌──────────┐            ││
                    │  │  │   RDS    │ │ElastiCache│ │Amazon MQ │            ││
                    │  │  │ Aurora   │ │  Redis   │ │ RabbitMQ │            ││
                    │  │  └──────────┘ └──────────┘ └──────────┘            ││
                    │  └─────────────────────────────────────────────────────┘│
                    └─────────────────────────────────────────────────────────┘

External Services:
┌──────────────┐  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐
│   DynamoDB   │  │      S3      │  │   Cognito    │  │  CloudWatch  │
│   (routes)   │  │   (images)   │  │    (auth)    │  │ (monitoring) │
└──────────────┘  └──────────────┘  └──────────────┘  └──────────────┘
```

## Service Mapping

| Local Service | AWS Replacement |
|---------------|-----------------|
| MongoDB | DynamoDB |
| PostgreSQL | RDS Aurora PostgreSQL |
| Redis | ElastiCache Redis |
| MinIO | S3 |
| RabbitMQ | Amazon MQ (RabbitMQ) |
| Keycloak | AWS Cognito |
| ClickHouse | EC2 (self-managed) |
| Fluvio | ECS Fargate |
| All services | ECS Fargate |

## Directory Structure

```
terraform/
├── environments/
│   ├── dev/                    Development environment
│   │   ├── main.tf             Main configuration
│   │   ├── variables.tf        Environment variables
│   │   ├── outputs.tf          Output values
│   │   ├── terraform.tfvars    Variable values
│   │   └── backend.tf          S3 backend config
│   └── prod/                   Production environment
│       └── ...
├── modules/
│   ├── vpc/                    VPC, subnets, NAT, endpoints
│   ├── security-groups/        Security group definitions
│   ├── rds/                    Aurora PostgreSQL
│   ├── elasticache/            Redis cluster
│   ├── dynamodb/               DynamoDB tables
│   ├── s3/                     S3 buckets
│   ├── amazon-mq/              RabbitMQ broker
│   ├── ecs/                    ECS cluster and services
│   ├── alb/                    Application Load Balancers
│   ├── cognito/                Cognito User Pool (auth)
│   ├── ecr/                    Container registries
│   ├── iam/                    IAM roles and policies
│   ├── acm/                    SSL certificates
│   ├── route53/                DNS records
│   ├── cloudwatch/             Monitoring and alarms
│   ├── secrets-manager/        Secrets storage
│   └── clickhouse/             ClickHouse EC2 instance
└── scripts/
    ├── deploy-infra.sh         Infrastructure deployment
    ├── build-push-images.sh    Docker image CI/CD
    ├── deploy-services.sh      Service deployment
    └── setup-secrets.sh        Secrets initialization
```

## Prerequisites

- AWS CLI configured with appropriate credentials
- Terraform >= 1.5.0
- Docker (for building images)
- Domain name with Route53 hosted zone (optional)

## Quick Start

### 1. Initialize Terraform

```bash
cd environments/dev
terraform init
```

### 2. Configure Variables

Copy and edit the example variables:

```bash
cp terraform.tfvars.example terraform.tfvars
```

Edit `terraform.tfvars`:

```hcl
region                    = "us-east-1"
domain_name               = "shortas.dev"
route53_zone_id           = "Z1234567890ABC"
create_route53_records    = true
alarm_email_endpoints     = ["alerts@example.com"]
image_tag                 = "latest"
```

### 3. Deploy Infrastructure

```bash
terraform plan
terraform apply
```

### 4. Build and Push Docker Images

```bash
cd ../scripts
./build-push-images.sh dev
```

### 5. Deploy Services

```bash
./deploy-services.sh dev
```

## Modules

### VPC (`modules/vpc`)

Creates the network infrastructure:
- VPC with configurable CIDR
- Public subnets (3 AZs) for ALB and NAT
- Private subnets (3 AZs) for ECS services
- Database subnets (3 AZs) for RDS/ElastiCache
- NAT Gateway(s) for private subnet internet access
- VPC Endpoints for AWS services (S3, DynamoDB, ECR, etc.)

### Cognito (`modules/cognito`)

AWS Cognito User Pool for authentication (replaces Keycloak):
- User Pool with email-based authentication
- Dashboard app client (public SPA with PKCE)
- API app client (confidential with client credentials)
- Configurable password policy and MFA
- User groups (admin, user)
- Optional Identity Pool for federated AWS access

See [modules/cognito/README.md](modules/cognito/README.md) for details.

### ECS (`modules/ecs`)

ECS Fargate cluster with all application services:
- Auto-scaling based on CPU/memory
- Service discovery via Cloud Map
- Spot instance support for cost savings
- ECS Exec enabled for debugging

### DynamoDB (`modules/dynamodb`)

DynamoDB tables for route storage:
- `core-routes-{env}` - URL routes
- `core-routes-encryption-{env}` - Encryption keys
- `core-routes-hostname-mapping-{env}` - Custom domain mappings
- `core-user-settings-{env}` - User preferences
- `core-domains-{env}` - Domain verification
- `core-routes-to-verify-{env}` - Route verification queue
- `core-certificate-orders-{env}` - SSL certificate orders
- `core-challenges-{env}` - ACME challenges

### Other Modules

- **RDS**: Aurora PostgreSQL Serverless v2
- **ElastiCache**: Redis cluster with replication
- **Amazon MQ**: RabbitMQ broker
- **S3**: Route images, ClickHouse backups, ALB logs
- **ALB**: Public and internal load balancers
- **CloudWatch**: Log groups, metrics, alarms, dashboards

## Environment Differences

| Feature | Dev | Prod |
|---------|-----|------|
| NAT Gateway | Single | Per-AZ |
| RDS Instances | 1 | 2+ (Multi-AZ) |
| Redis Nodes | 1 | 3 (Primary + replicas) |
| ECS Spot | Yes | No |
| Container Insights | No | Yes |
| Cognito MFA | Optional | Required |
| Password Policy | 8 chars | 12 chars + symbols |
| Backup Retention | 3 days | 14 days |

## Outputs

After deployment, Terraform outputs important values:

```bash
terraform output

# Key outputs:
cognito_user_pool_id      = "us-east-1_xxxxxxxx"
cognito_hosted_ui_url     = "https://dev-shortas-auth.auth.us-east-1.amazoncognito.com"
cognito_dashboard_client_id = "xxxxxxxxxxxxxxxxxxxxxxxxxx"
alb_dns_name              = "dev-shortas-public-alb-123456789.us-east-1.elb.amazonaws.com"
ecr_repository_urls       = { ... }
```

## Cost Optimization

The dev environment is optimized for cost:
- Single NAT Gateway instead of per-AZ
- Fargate Spot instances
- Smaller instance types
- Reduced backup retention
- Container Insights disabled

For production, these are enabled for reliability.

## Security

- All services run in private subnets
- Database subnets have no internet access
- Security groups follow least-privilege
- Secrets stored in AWS Secrets Manager
- SSL/TLS everywhere (ACM certificates)
- Cognito handles authentication with MFA support

## Monitoring

CloudWatch provides:
- Log aggregation for all services
- Custom metrics dashboards
- Alarms for error rates, latency, resource usage
- SNS notifications for critical alerts

## Troubleshooting

### ECS Exec for Debugging

```bash
aws ecs execute-command \
  --cluster dev-shortas-cluster \
  --task <task-id> \
  --container click-router \
  --interactive \
  --command "/bin/sh"
```

### View Service Logs

```bash
aws logs tail /ecs/dev-shortas-click-router --follow
```

### Check Service Health

```bash
aws ecs describe-services \
  --cluster dev-shortas-cluster \
  --services click-router
```

## Related Documentation

- [Cognito Module](modules/cognito/README.md)
- [Main Project README](../../../README.md)
- [Deployment Guide](../../../docs/deployment/index.md)
