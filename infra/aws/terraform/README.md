# AWS deployment

Terraform for running Shortas on AWS. All services run on ECS Fargate. The self-hosted stores are replaced by managed ones where AWS has an equivalent.

| Local | AWS |
|---|---|
| MongoDB | DynamoDB |
| PostgreSQL | RDS Aurora PostgreSQL (Serverless v2) |
| Redis | ElastiCache |
| RabbitMQ | Amazon MQ |
| MinIO | S3 |
| Keycloak | Cognito ([module notes](modules/cognito/README.md)) |
| ClickHouse | EC2 (self-managed) |
| Fluvio, all services | ECS Fargate |

```
 Internet -> ALB (public subnets)
               -> ECS Fargate services, ClickHouse EC2, Fluvio (private subnets)
                    -> RDS, ElastiCache, Amazon MQ (database subnets, no internet)
            DynamoDB, S3, Cognito, CloudWatch, Secrets Manager (managed)
```

## Layout

```
terraform/
  environments/dev       development (Spot, single NAT)
  environments/prod      production (Multi-AZ, replicas)
  environments/budget    minimal demo environment, spin up and tear down
  modules/               vpc, ecs, alb, rds, elasticache, dynamodb, s3,
                         amazon-mq, cognito, ecr, iam, acm, route53,
                         cloudwatch, secrets-manager, clickhouse, ...
infra/aws/scripts/       deploy-infra, setup-secrets, build-push-images,
                         deploy-services
```

## Prerequisites

- AWS CLI with credentials
- Terraform ≥ 1.5, and `jq`
- Docker, for building images
- Optional: a Route 53 hosted zone for your domain

## Deploy

1. Set your values in `environments/<env>/terraform.tfvars`: `region`, `domain_name`, `route53_zone_id` / `create_route53_records`, `alarm_email_endpoints`, `image_tag`. To keep state remotely, uncomment `backend.tf` after creating its S3 bucket.
2. Run the scripts from `infra/aws/scripts`. They accept `dev` or `prod`:

   ```bash
   ./deploy-infra.sh dev plan
   ./deploy-infra.sh dev apply
   ./build-push-images.sh dev              # optionally: list services
   ./deploy-services.sh dev                # optionally: [image_tag] [services…]
   ```

   `./setup-secrets.sh dev` creates any missing Secrets Manager entries (RDS, Redis, RabbitMQ, ClickHouse, Keycloak) and keeps existing values. `deploy-services.sh` also syncs `notification-templates/` to S3 whenever it deploys `notification-worker`.

The `budget` environment isn't handled by the scripts. Apply it directly:

```bash
cd environments/budget
terraform init
terraform apply
```

After applying, `terraform output` prints the Cognito pool and client IDs, the hosted UI URL, the ALB DNS name and the ECR repository URLs.

## Environments

| | dev | prod |
|---|---|---|
| NAT gateway | single | per AZ |
| RDS instances | 1 | 2+ (Multi-AZ) |
| Redis nodes | 1 | 3 |
| Fargate Spot | yes | no |
| Container Insights | no | yes |
| Cognito MFA | optional | required |
| Password policy | 8 chars | 12 chars + symbols |
| Backup retention | 3 days | 14 days |

## Operate

CloudWatch collects all service logs, and alarms notify through SNS.

```bash
aws logs tail /ecs/dev-shortas-click-router --follow

aws ecs describe-services --cluster dev-shortas-cluster --services click-router

aws ecs execute-command --cluster dev-shortas-cluster --task <task-id> \
  --container click-router --interactive --command "/bin/sh"
```
