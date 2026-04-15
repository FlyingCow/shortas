output "vpc_id" {
  description = "VPC ID"
  value       = module.vpc.vpc_id
}

output "alb_dns_name" {
  description = "Public ALB DNS name"
  value       = module.alb.public_alb_dns_name
}

output "ecr_repository_urls" {
  description = "ECR repository URLs"
  value       = module.ecr.repository_urls
}

output "rds_endpoint" {
  description = "RDS cluster endpoint"
  value       = module.rds.cluster_endpoint
}

output "redis_endpoint" {
  description = "ElastiCache Redis endpoint"
  value       = module.elasticache.primary_endpoint
}

output "rabbitmq_endpoint" {
  description = "Amazon MQ RabbitMQ endpoint"
  value       = module.amazon_mq.primary_amqp_endpoint
}

output "clickhouse_endpoint" {
  description = "ClickHouse HTTP endpoint"
  value       = module.clickhouse.http_endpoint
}

output "s3_route_images_bucket" {
  description = "S3 bucket for route images"
  value       = module.s3.route_images_bucket_name
}

output "s3_route_images_url" {
  description = "S3 URL for route images"
  value       = module.s3.route_images_bucket_url
}

output "ecs_cluster_name" {
  description = "ECS cluster name"
  value       = module.ecs.cluster_name
}

output "service_discovery_namespace" {
  description = "Service discovery namespace"
  value       = module.ecs.service_discovery_namespace_name
}

output "dynamodb_tables" {
  description = "DynamoDB table names"
  value = {
    routes           = module.dynamodb.routes_table_name
    encryption       = module.dynamodb.routes_encryption_table_name
    hostname_mapping = module.dynamodb.hostname_mapping_table_name
    user_settings    = module.dynamodb.user_settings_table_name
  }
}

# Cognito outputs
output "cognito_user_pool_id" {
  description = "Cognito User Pool ID"
  value       = module.cognito.user_pool_id
}

output "cognito_user_pool_endpoint" {
  description = "Cognito User Pool endpoint"
  value       = module.cognito.user_pool_endpoint
}

output "cognito_hosted_ui_url" {
  description = "Cognito Hosted UI URL"
  value       = module.cognito.hosted_ui_url
}

output "cognito_dashboard_client_id" {
  description = "Cognito dashboard app client ID"
  value       = module.cognito.dashboard_client_id
}

output "cognito_issuer_url" {
  description = "Cognito OIDC issuer URL"
  value       = module.cognito.issuer_url
}

output "cognito_jwks_url" {
  description = "Cognito JWKS URL for token validation"
  value       = module.cognito.jwks_url
}
