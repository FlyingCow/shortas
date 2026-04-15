# Production Environment - Shortas URL Shortener AWS Infrastructure

terraform {
  required_version = ">= 1.5.0"

  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 5.0"
    }
    random = {
      source  = "hashicorp/random"
      version = "~> 3.5"
    }
  }
}

provider "aws" {
  region = var.region

  default_tags {
    tags = {
      Project     = "shortas"
      Environment = "prod"
      ManagedBy   = "terraform"
    }
  }
}

locals {
  environment = "prod"

  service_names = [
    "click-router",
    "click-router-api",
    "click-tracker",
    "click-aggregator",
    "click-aggregator-api",
    "domain-verifier",
    "route-verifier",
    "route-icon-worker",
    "cert-bot",
    "shortas-api",
    "dashboard",
    "landing",
    "pages",
    "fluvio-sc",
    "fluvio-spu"
  ]
}

# VPC
module "vpc" {
  source = "../../modules/vpc"

  environment          = local.environment
  region               = var.region
  vpc_cidr             = "10.1.0.0/16"
  enable_nat_gateway   = true
  single_nat_gateway   = false  # High availability for prod
  enable_vpc_endpoints = true
}

# Security Groups
module "security_groups" {
  source = "../../modules/security-groups"

  environment     = local.environment
  vpc_id          = module.vpc.vpc_id
  vpc_cidr        = module.vpc.vpc_cidr
  enable_keycloak = false  # Using AWS Cognito instead
}

# ECR Repositories
module "ecr" {
  source = "../../modules/ecr"

  environment         = local.environment
  service_names       = local.service_names
  scan_on_push        = true
  image_count_to_keep = 50
}

# DynamoDB Tables
module "dynamodb" {
  source = "../../modules/dynamodb"

  environment                   = local.environment
  billing_mode                  = "PAY_PER_REQUEST"
  enable_point_in_time_recovery = true
  enable_cloudwatch_alarms      = true
  alarm_actions                 = [module.cloudwatch.sns_topic_arn]
}

# S3 Buckets
module "s3" {
  source = "../../modules/s3"

  environment                   = local.environment
  create_terraform_state_bucket = true
  create_logs_bucket            = true
  enable_versioning             = true
  cors_allowed_origins          = var.cors_allowed_origins
}

# RDS Aurora PostgreSQL
module "rds" {
  source = "../../modules/rds"

  environment                 = local.environment
  db_subnet_group_name        = module.vpc.db_subnet_group_name
  security_group_id           = module.security_groups.rds_security_group_id
  instance_count              = 2  # Multi-AZ for prod
  min_capacity                = 2
  max_capacity                = 32
  backup_retention_period     = 14
  enable_performance_insights = true
  enable_enhanced_monitoring  = true
}

# ElastiCache Redis
module "elasticache" {
  source = "../../modules/elasticache"

  environment              = local.environment
  subnet_group_name        = module.vpc.elasticache_subnet_group_name
  security_group_id        = module.security_groups.elasticache_security_group_id
  node_type                = "cache.r6g.large"
  num_cache_clusters       = 3  # Primary + 2 replicas
  multi_az_enabled         = true
  snapshot_retention_limit = 7
  enable_cloudwatch_alarms = true
  alarm_actions            = [module.cloudwatch.sns_topic_arn]
}

# Amazon MQ (RabbitMQ)
module "amazon_mq" {
  source = "../../modules/amazon-mq"

  environment              = local.environment
  subnet_ids               = module.vpc.database_subnet_ids
  security_group_id        = module.security_groups.mq_security_group_id
  host_instance_type       = "mq.m5.large"
  deployment_mode          = "CLUSTER_MULTI_AZ"
  enable_cloudwatch_alarms = true
  alarm_actions            = [module.cloudwatch.sns_topic_arn]
}

# Secrets Manager
module "secrets" {
  source = "../../modules/secrets-manager"

  environment     = local.environment
  clickhouse_host = module.clickhouse.private_ip
  enable_keycloak = false  # Using AWS Cognito instead
}

# IAM Roles
module "iam" {
  source = "../../modules/iam"

  environment                  = local.environment
  region                       = var.region
  dynamodb_table_arns          = module.dynamodb.all_table_arns
  s3_bucket_arns               = [module.s3.route_images_bucket_arn, module.s3.logs_bucket_arn]
  clickhouse_backup_bucket_arn = module.s3.clickhouse_backups_bucket_arn
  enable_ecs_exec              = true
}

# ACM Certificate
module "acm" {
  source = "../../modules/acm"

  environment               = local.environment
  domain_name               = var.domain_name
  subject_alternative_names = var.subject_alternative_names
  create_route53_records    = var.create_route53_records
  route53_zone_id           = var.route53_zone_id
}

# Cognito (replaces Keycloak for AWS deployments)
module "cognito" {
  source = "../../modules/cognito"

  environment = local.environment

  # Password policy (stricter for prod)
  password_minimum_length    = 12
  password_require_lowercase = true
  password_require_uppercase = true
  password_require_numbers   = true
  password_require_symbols   = true

  # MFA required for prod
  mfa_configuration = "ON"

  # Advanced security enforced for prod
  advanced_security_mode = "ENFORCED"

  # Callback URLs for dashboard
  dashboard_callback_urls = [
    "https://app.${var.domain_name}/callback"
  ]
  dashboard_logout_urls = [
    "https://app.${var.domain_name}"
  ]

  # Token validity
  access_token_validity_hours  = 1
  id_token_validity_hours      = 1
  refresh_token_validity_days  = 7
}

# ALB
module "alb" {
  source = "../../modules/alb"

  environment                    = local.environment
  vpc_id                         = module.vpc.vpc_id
  public_subnet_ids              = module.vpc.public_subnet_ids
  private_subnet_ids             = module.vpc.private_subnet_ids
  public_alb_security_group_id   = module.security_groups.alb_security_group_id
  internal_alb_security_group_id = module.security_groups.internal_alb_security_group_id
  certificate_arn                = module.acm.certificate_arn
  access_logs_bucket             = module.s3.logs_bucket_name
  enable_access_logs             = true

  api_host_headers       = ["api.${var.domain_name}"]
  dashboard_host_headers = ["app.${var.domain_name}"]
  landing_host_headers   = ["www.${var.domain_name}", var.domain_name]
  # Keycloak removed - using AWS Cognito instead
}

# ClickHouse
module "clickhouse" {
  source = "../../modules/clickhouse"

  environment                = local.environment
  region                     = var.region
  subnet_id                  = module.vpc.private_subnet_ids[0]
  availability_zone          = module.vpc.availability_zones[0]
  security_group_id          = module.security_groups.clickhouse_security_group_id
  instance_profile_name      = module.iam.clickhouse_instance_profile_name
  instance_type              = "r6i.xlarge"  # Larger for prod
  data_volume_size           = 500
  data_volume_type           = "gp3"
  clickhouse_password        = random_password.clickhouse.result
  s3_backup_bucket           = module.s3.clickhouse_backups_bucket_name
  enable_detailed_monitoring = true
  enable_cloudwatch_alarms   = true
  alarm_actions              = [module.cloudwatch.sns_topic_arn]
}

resource "random_password" "clickhouse" {
  length  = 32
  special = false
}

# ECS Cluster and Services
module "ecs" {
  source = "../../modules/ecs"

  environment              = local.environment
  region                   = var.region
  vpc_id                   = module.vpc.vpc_id
  private_subnet_ids       = module.vpc.private_subnet_ids
  ecs_security_group_id    = module.security_groups.ecs_security_group_id
  fluvio_security_group_id = module.security_groups.fluvio_security_group_id
  # keycloak_security_group_id removed - using AWS Cognito instead
  task_execution_role_arn  = module.iam.ecs_task_execution_role_arn
  task_role_arn            = module.iam.ecs_task_role_arn
  ecr_repository_url       = module.ecr.base_repository_url
  image_tag                = var.image_tag

  enable_container_insights = true
  use_spot_instances        = false  # Reliable capacity for prod
  enable_ecs_exec           = true

  service_names = local.service_names

  services = [
    {
      name              = "click-router"
      cpu               = 1024
      memory            = 2048
      desired_count     = 3
      min_count         = 2
      max_count         = 20
      container_port    = 5800
      target_group_arn  = module.alb.click_router_target_group_arn
      enable_autoscaling = true
      cpu_target_value  = 60
      health_check = {
        command      = ["CMD-SHELL", "curl -f http://localhost:5800/health || exit 1"]
        interval     = 30
        timeout      = 5
        retries      = 3
        start_period = 60
      }
    },
    {
      name              = "click-router-api"
      cpu               = 512
      memory            = 1024
      desired_count     = 2
      min_count         = 2
      max_count         = 10
      container_port    = 5810
      target_group_arn  = module.alb.click_router_api_target_group_arn
      enable_autoscaling = true
    },
    {
      name              = "click-tracker"
      cpu               = 512
      memory            = 1024
      desired_count     = 2
      min_count         = 2
      max_count         = 10
      enable_autoscaling = true
    },
    {
      name              = "click-aggregator"
      cpu               = 512
      memory            = 1024
      desired_count     = 2
      min_count         = 1
      max_count         = 5
      enable_autoscaling = true
    },
    {
      name              = "click-aggregator-api"
      cpu               = 512
      memory            = 1024
      desired_count     = 2
      min_count         = 2
      max_count         = 10
      container_port    = 5820
      target_group_arn  = module.alb.aggregator_api_target_group_arn
      enable_autoscaling = true
    },
    {
      name              = "domain-verifier"
      cpu               = 512
      memory            = 1024
      desired_count     = 2
      container_port    = 5830
      target_group_arn  = module.alb.domain_verifier_target_group_arn
    },
    {
      name              = "route-verifier"
      cpu               = 512
      memory            = 1024
      desired_count     = 2
      container_port    = 5831
    },
    {
      name          = "route-icon-worker"
      cpu           = 512
      memory        = 1024
      desired_count = 2
    },
    {
      name          = "cert-bot"
      cpu           = 256
      memory        = 512
      desired_count = 1
    },
    {
      name              = "shortas-api"
      cpu               = 1024
      memory            = 2048
      desired_count     = 3
      min_count         = 2
      max_count         = 10
      container_port    = 80
      target_group_arn  = module.alb.shortas_api_target_group_arn
      enable_autoscaling = true
    },
    {
      name              = "dashboard"
      cpu               = 512
      memory            = 1024
      desired_count     = 2
      min_count         = 2
      max_count         = 5
      container_port    = 3000
      target_group_arn  = module.alb.dashboard_target_group_arn
      enable_autoscaling = true
    },
    {
      name              = "landing"
      cpu               = 256
      memory            = 512
      desired_count     = 2
      min_count         = 2
      max_count         = 5
      container_port    = 3000
      target_group_arn  = module.alb.landing_target_group_arn
      enable_autoscaling = true
    },
    {
      name              = "pages"
      cpu               = 512
      memory            = 1024
      desired_count     = 2
      min_count         = 2
      max_count         = 5
      container_port    = 5801
      target_group_arn  = module.alb.pages_target_group_arn
      enable_autoscaling = true
    }
  ]

  common_secrets = [
    {
      name      = "APP_RABBITMQ_URI"
      valueFrom = "${module.amazon_mq.secret_arn}:amqp_uri::"
    },
    {
      name      = "APP_REDIS_URL"
      valueFrom = "${module.elasticache.secret_arn}:connection_string::"
    }
  ]

  enable_fluvio     = true
  fluvio_sc_cpu     = 512
  fluvio_sc_memory  = 1024
  fluvio_spu_count  = 3
  fluvio_spu_cpu    = 512
  fluvio_spu_memory = 1024

  # Keycloak disabled - using AWS Cognito instead
  enable_keycloak = false

  # Cognito configuration passed to services
  cognito_user_pool_id     = module.cognito.user_pool_id
  cognito_issuer_url       = module.cognito.issuer_url
  cognito_jwks_url         = module.cognito.jwks_url
  cognito_dashboard_client_id = module.cognito.dashboard_client_id
}

# Route53 DNS
module "route53" {
  source = "../../modules/route53"
  count  = var.create_route53_records ? 1 : 0

  environment         = local.environment
  domain_name         = var.domain_name
  existing_zone_id    = var.route53_zone_id
  alb_dns_name        = module.alb.public_alb_dns_name
  alb_zone_id         = module.alb.public_alb_zone_id
  create_health_check = true
  alarm_actions       = [module.cloudwatch.sns_topic_arn]

  subdomain_records = {
    api = { target_dns_name = module.alb.public_alb_dns_name, target_zone_id = module.alb.public_alb_zone_id }
    app = { target_dns_name = module.alb.public_alb_dns_name, target_zone_id = module.alb.public_alb_zone_id }
    # auth subdomain removed - using AWS Cognito hosted UI
  }
}

# CloudWatch Monitoring
module "cloudwatch" {
  source = "../../modules/cloudwatch"

  environment           = local.environment
  region                = var.region
  ecs_cluster_name      = module.ecs.cluster_name
  ecs_service_names     = local.service_names
  alb_arn_suffix        = replace(module.alb.public_alb_arn, "/.*:loadbalancer\\//", "")
  alarm_email_endpoints = var.alarm_email_endpoints
  alb_5xx_threshold     = 5
  target_5xx_threshold  = 20
  response_time_threshold = 1
  create_composite_alarm = true
}
