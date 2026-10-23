# Budget Environment - Shortas URL Shortener AWS Infrastructure (demo/presentation, spin up/tear down)

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
      Environment = "budget"
      ManagedBy   = "terraform"
    }
  }
}

locals {
  environment = "budget"

  # Cloud Map DNS name for the Fluvio SC service (see modules/ecs/main.tf's
  # service-discovery namespace "${environment}.shortas.local").
  fluvio_sc_host = "fluvio-sc.budget.shortas.local:9003"

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
    "management-api",
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
  single_nat_gateway   = true # Cost savings for budget (demo/presentation only)
  enable_vpc_endpoints = true
}

# Security Groups
module "security_groups" {
  source = "../../modules/security-groups"

  environment     = local.environment
  vpc_id          = module.vpc.vpc_id
  vpc_cidr        = module.vpc.vpc_cidr
  enable_keycloak = false # Using AWS Cognito instead
}

# ECR Repositories
module "ecr" {
  source = "../../modules/ecr"

  environment   = local.environment
  service_names = local.service_names
  scan_on_push  = true
}

# DynamoDB Tables
module "dynamodb" {
  source = "../../modules/dynamodb"

  environment                   = local.environment
  billing_mode                  = "PAY_PER_REQUEST"
  enable_point_in_time_recovery = true
}

# S3 Buckets
module "s3" {
  source = "../../modules/s3"

  environment                   = local.environment
  create_terraform_state_bucket = true
  cors_allowed_origins          = ["*"]
}

# RDS Aurora PostgreSQL
module "rds" {
  source = "../../modules/rds"

  environment                 = local.environment
  db_subnet_group_name        = module.vpc.db_subnet_group_name
  security_group_id           = module.security_groups.rds_security_group_id
  instance_count              = 1 # Single instance for budget
  min_capacity                = 0.5
  max_capacity                = 2 # Lower ceiling than dev - demo traffic only
  backup_retention_period     = 1 # Minimum allowed - short-lived environment
  enable_performance_insights = false
  enable_enhanced_monitoring  = false
}

# ElastiCache Redis
module "elasticache" {
  source = "../../modules/elasticache"

  environment        = local.environment
  subnet_group_name  = module.vpc.elasticache_subnet_group_name
  security_group_id  = module.security_groups.elasticache_security_group_id
  node_type          = "cache.t4g.micro"
  num_cache_clusters = 1 # Single node for budget
  multi_az_enabled   = false
}

# OpenSearch (route search index for management-api)
module "opensearch" {
  source = "../../modules/opensearch"

  environment              = local.environment
  subnet_ids               = [module.vpc.private_subnet_ids[0]]
  security_group_id        = module.security_groups.opensearch_security_group_id
  instance_type            = "t3.small.search" # Smallest for budget
  instance_count           = 1
  zone_awareness_enabled   = false
  ebs_volume_size          = 10
  enable_cloudwatch_alarms = false # Cost savings for budget (demo/presentation only)
}

# Amazon MQ (RabbitMQ)
module "amazon_mq" {
  source = "../../modules/amazon-mq"

  environment        = local.environment
  subnet_ids         = module.vpc.database_subnet_ids
  security_group_id  = module.security_groups.mq_security_group_id
  host_instance_type = "mq.t3.micro"
  deployment_mode    = "SINGLE_INSTANCE"
}

# Secrets Manager
module "secrets" {
  source = "../../modules/secrets-manager"

  environment     = local.environment
  clickhouse_host = module.clickhouse.private_ip
  enable_keycloak = false # Using AWS Cognito instead
}

# IAM Roles
module "iam" {
  source = "../../modules/iam"

  environment                  = local.environment
  region                       = var.region
  dynamodb_table_arns          = module.dynamodb.all_table_arns
  s3_bucket_arns               = [module.s3.route_images_bucket_arn]
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

  # Password policy
  password_minimum_length    = 8
  password_require_lowercase = true
  password_require_uppercase = true
  password_require_numbers   = true
  password_require_symbols   = false

  # MFA - optional for budget
  mfa_configuration = "OPTIONAL"

  # Advanced security in audit mode for budget
  advanced_security_mode = "AUDIT"

  # Callback URLs for dashboard
  dashboard_callback_urls = [
    "https://app.${var.domain_name}/callback",
    "http://localhost:3000/callback"
  ]
  dashboard_logout_urls = [
    "https://app.${var.domain_name}",
    "http://localhost:3000"
  ]

  # Token validity
  access_token_validity_hours = 1
  id_token_validity_hours     = 1
  refresh_token_validity_days = 30
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

  api_host_headers       = ["api.${var.domain_name}"]
  dashboard_host_headers = ["app.${var.domain_name}"]
  landing_host_headers   = ["www.${var.domain_name}", var.domain_name]
  # Keycloak removed - using AWS Cognito instead
}

# ClickHouse
module "clickhouse" {
  source = "../../modules/clickhouse"

  environment           = local.environment
  region                = var.region
  subnet_id             = module.vpc.private_subnet_ids[0]
  availability_zone     = module.vpc.availability_zones[0]
  security_group_id     = module.security_groups.clickhouse_security_group_id
  instance_profile_name = module.iam.clickhouse_instance_profile_name
  instance_type         = "t3.small" # Smaller than dev - demo data volumes only
  data_volume_size      = 20
  clickhouse_password   = random_password.clickhouse.result
  s3_backup_bucket      = module.s3.clickhouse_backups_bucket_name
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
  task_execution_role_arn = module.iam.ecs_task_execution_role_arn
  task_role_arn           = module.iam.ecs_task_role_arn
  ecr_repository_url      = module.ecr.base_repository_url
  image_tag               = var.image_tag

  enable_container_insights = false # Cost savings for budget (demo/presentation only)
  use_spot_instances        = true  # Cost savings for budget (demo/presentation only)
  enable_ecs_exec           = true
  app_run_mode              = "aws-dev" # selects config/aws-dev.toml in each service

  service_names = local.service_names

  services = [
    {
      name             = "click-router"
      cpu              = 256
      memory           = 512
      desired_count    = 1
      container_port   = 5800
      target_group_arn = module.alb.click_router_target_group_arn
      health_check = {
        command      = ["CMD-SHELL", "curl -f http://localhost:5800/health || exit 1"]
        interval     = 30
        timeout      = 5
        retries      = 3
        start_period = 60
      }
      environment_variables = [
        { name = "APP__AWS__DYNAMO__ROUTES_TABLE", value = module.dynamodb.routes_table_name },
        { name = "APP__AWS__DYNAMO__ENCRYPTION_TABLE", value = module.dynamodb.routes_encryption_table_name },
        { name = "APP__AWS__DYNAMO__HOSTNAME_MAPPINGS_TABLE", value = module.dynamodb.hostname_mapping_table_name },
        { name = "APP__AWS__DYNAMO__USER_SETTINGS_TABLE", value = module.dynamodb.user_settings_table_name },
        { name = "APP__FLUVIO__HIT_STREAM__HOST", value = local.fluvio_sc_host },
      ]
      secrets = [
        { name = "APP__RABBITMQ__URI", valueFrom = "${module.amazon_mq.secret_arn}:amqp_uri::" },
      ]
    },
    {
      name             = "click-router-api"
      cpu              = 256
      memory           = 512
      desired_count    = 1
      container_port   = 5810
      target_group_arn = module.alb.click_router_api_target_group_arn
      environment_variables = [
        { name = "APP__AWS__DYNAMO__ROUTES_TABLE", value = module.dynamodb.routes_table_name },
        { name = "APP__AWS__DYNAMO__ENCRYPTION_TABLE", value = module.dynamodb.routes_encryption_table_name },
        { name = "APP__AWS__DYNAMO__USER_SETTINGS_TABLE", value = module.dynamodb.user_settings_table_name },
      ]
      secrets = [
        { name = "APP__RABBITMQ__URI", valueFrom = "${module.amazon_mq.secret_arn}:amqp_uri::" },
      ]
    },
    {
      name          = "click-tracker"
      cpu           = 256
      memory        = 512
      desired_count = 1
      environment_variables = [
        { name = "APP__AWS__DYNAMO__USER_SETTINGS_TABLE", value = module.dynamodb.user_settings_table_name },
        { name = "APP__FLUVIO__HIT_STREAM__HOST", value = local.fluvio_sc_host },
        { name = "APP__FLUVIO__CLICK_AGGS__HOST", value = local.fluvio_sc_host },
      ]
      secrets = [
        { name = "APP__RABBITMQ__URI", valueFrom = "${module.amazon_mq.secret_arn}:amqp_uri::" },
        { name = "APP__REDIS__HOST", valueFrom = "${module.elasticache.secret_arn}:connection_string::" },
      ]
    },
    {
      name          = "click-aggregator"
      cpu           = 256
      memory        = 512
      desired_count = 1
      environment_variables = [
        { name = "APP__FLUVIO__CLICK_STREAM__HOST", value = local.fluvio_sc_host },
        { name = "APP__CLICKHOUSE__CLICK_STREAM_STORE__URL", value = module.clickhouse.http_endpoint },
        { name = "APP__CLICKHOUSE__CLICK_STREAM_STORE__USER", value = "default" },
        { name = "APP__CLICKHOUSE__CLICK_STREAM_STORE__DATABASE", value = "shortas" },
        # Same plaintext-env approach already used for the ClickHouse instance
        # itself (no secrets-manager entry exists for this password).
        { name = "APP__CLICKHOUSE__CLICK_STREAM_STORE__PASSWORD", value = random_password.clickhouse.result },
      ]
    },
    {
      name             = "click-aggregator-api"
      cpu              = 256
      memory           = 512
      desired_count    = 1
      container_port   = 5820
      target_group_arn = module.alb.aggregator_api_target_group_arn
      environment_variables = [
        { name = "APP__CLICKHOUSE__URL", value = module.clickhouse.http_endpoint },
        { name = "APP__CLICKHOUSE__USER", value = "default" },
        { name = "APP__CLICKHOUSE__DATABASE", value = "shortas" },
        { name = "APP__CLICKHOUSE__PASSWORD", value = random_password.clickhouse.result },
      ]
    },
    {
      name             = "domain-verifier"
      cpu              = 256
      memory           = 512
      desired_count    = 1
      container_port   = 5830
      target_group_arn = module.alb.domain_verifier_target_group_arn
      environment_variables = [
        { name = "APP__AWS__DYNAMO__DOMAINS_TABLE", value = module.dynamodb.domains_table_name },
      ]
      secrets = [
        { name = "APP__RABBITMQ__URI", valueFrom = "${module.amazon_mq.secret_arn}:amqp_uri::" },
      ]
    },
    {
      name           = "route-verifier"
      cpu            = 256
      memory         = 512
      desired_count  = 1
      container_port = 5831
      environment_variables = [
        { name = "APP__AWS__DYNAMO__ROUTES_TABLE", value = module.dynamodb.routes_to_verify_table_name },
      ]
      secrets = [
        { name = "APP__RABBITMQ__URI", valueFrom = "${module.amazon_mq.secret_arn}:amqp_uri::" },
      ]
    },
    {
      name          = "route-icon-worker"
      cpu           = 256
      memory        = 512
      desired_count = 1
      environment_variables = [
        { name = "APP__S3__BUCKET", value = module.s3.route_images_bucket_name },
        { name = "APP__S3__REGION", value = var.region },
      ]
      secrets = [
        { name = "APP__RABBITMQ__URI", valueFrom = "${module.amazon_mq.secret_arn}:amqp_uri::" },
      ]
    },
    {
      name          = "cert-bot"
      cpu           = 256
      memory        = 512
      desired_count = 1
      environment_variables = [
        { name = "APP__AWS__DYNAMO__ORDERS_TABLE", value = module.dynamodb.certificate_orders_table_name },
        { name = "APP__CLICK_ROUTER_API__BASE_URL", value = "http://click-router-api.${local.environment}.shortas.local:5810" },
      ]
      secrets = [
        { name = "APP__RABBITMQ__URI", valueFrom = "${module.amazon_mq.secret_arn}:amqp_uri::" },
      ]
    },
    {
      name             = "management-api"
      cpu              = 512
      memory           = 1024
      desired_count    = 1
      container_port   = 8080
      target_group_arn = module.alb.management_api_target_group_arn
      health_check = {
        command      = ["CMD-SHELL", "curl -f http://localhost:8080/health || exit 1"]
        interval     = 30
        timeout      = 5
        retries      = 3
        start_period = 60
      }
      environment_variables = [
        { name = "APP__DATABASE__MAX_CONNECTIONS", value = "10" },
        { name = "APP__JWT__ISSUER", value = module.cognito.issuer_url },
        { name = "APP__JWT__AUDIENCE", value = "management-api" },
        { name = "APP__JWT__JWKS_URL", value = module.cognito.jwks_url },
        { name = "APP__CLICK_ROUTER__BASE_URL", value = "http://click-router-api.${local.environment}.shortas.local:5810" },
        { name = "APP__CLICK_AGGREGATOR__BASE_URL", value = "http://click-aggregator-api.${local.environment}.shortas.local:5820" },
        { name = "APP__ELASTICSEARCH__URL", value = module.opensearch.url },
        { name = "APP__MINIO__BUCKET", value = module.s3.route_images_bucket_name },
        { name = "APP__MINIO__REGION", value = var.region },
      ]
      secrets = [
        { name = "APP__DATABASE__HOST", valueFrom = "${module.rds.secret_arn}:host::" },
        { name = "APP__DATABASE__PORT", valueFrom = "${module.rds.secret_arn}:port::" },
        { name = "APP__DATABASE__USERNAME", valueFrom = "${module.rds.secret_arn}:username::" },
        { name = "APP__DATABASE__PASSWORD", valueFrom = "${module.rds.secret_arn}:password::" },
        { name = "APP__DATABASE__DATABASE", valueFrom = "${module.rds.secret_arn}:dbname::" },
        { name = "APP__RABBITMQ__URL", valueFrom = "${module.amazon_mq.secret_arn}:amqp_uri::" },
      ]
    },
    {
      name             = "dashboard"
      cpu              = 256
      memory           = 512
      desired_count    = 1
      container_port   = 3000
      target_group_arn = module.alb.dashboard_target_group_arn
      environment_variables = [
        { name = "REACT_APP_COGNITO_ISSUER_URL", value = module.cognito.issuer_url },
        { name = "REACT_APP_COGNITO_CLIENT_ID", value = module.cognito.dashboard_client_id },
        { name = "REACT_APP_PROXY_API_URL", value = "https://api.${var.domain_name}" },
        { name = "REACT_APP_ROUTE_IMAGES_BASE_URL", value = module.s3.route_images_bucket_url },
      ]
    },
    {
      name             = "landing"
      cpu              = 256
      memory           = 512
      desired_count    = 1
      container_port   = 3000
      target_group_arn = module.alb.landing_target_group_arn
    },
    {
      name             = "pages"
      cpu              = 256
      memory           = 512
      desired_count    = 1
      container_port   = 5801
      target_group_arn = module.alb.pages_target_group_arn
    }
  ]

  enable_fluvio     = true
  fluvio_sc_cpu     = 256
  fluvio_sc_memory  = 512
  fluvio_spu_count  = 1
  fluvio_spu_cpu    = 256
  fluvio_spu_memory = 512

  # Keycloak disabled - using AWS Cognito instead
  enable_keycloak = false

  # Cognito configuration passed to services
  cognito_user_pool_id        = module.cognito.user_pool_id
  cognito_issuer_url          = module.cognito.issuer_url
  cognito_jwks_url            = module.cognito.jwks_url
  cognito_dashboard_client_id = module.cognito.dashboard_client_id
}

# Route53 DNS (optional)
module "route53" {
  source = "../../modules/route53"
  count  = var.create_route53_records ? 1 : 0

  environment         = local.environment
  domain_name         = var.domain_name
  existing_zone_id    = var.route53_zone_id
  alb_dns_name        = module.alb.public_alb_dns_name
  alb_zone_id         = module.alb.public_alb_zone_id
  create_health_check = false # Disable for budget

  subdomain_records = {
    api = { target_dns_name = module.alb.public_alb_dns_name, target_zone_id = module.alb.public_alb_zone_id }
    app = { target_dns_name = module.alb.public_alb_dns_name, target_zone_id = module.alb.public_alb_zone_id }
    # auth subdomain removed - using AWS Cognito hosted UI
  }
}

# CloudWatch Monitoring
module "cloudwatch" {
  source = "../../modules/cloudwatch"

  environment       = local.environment
  region            = var.region
  ecs_cluster_name  = module.ecs.cluster_name
  ecs_service_names = local.service_names
  alb_arn_suffix    = replace(module.alb.public_alb_arn, "/.*:loadbalancer\\//", "")

  alarm_email_endpoints  = var.alarm_email_endpoints
  create_composite_alarm = false # Simpler for budget
}
