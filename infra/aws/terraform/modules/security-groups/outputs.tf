output "alb_security_group_id" {
  description = "Security group ID for public ALB"
  value       = aws_security_group.alb.id
}

output "internal_alb_security_group_id" {
  description = "Security group ID for internal ALB"
  value       = aws_security_group.internal_alb.id
}

output "ecs_security_group_id" {
  description = "Security group ID for ECS services"
  value       = aws_security_group.ecs.id
}

output "rds_security_group_id" {
  description = "Security group ID for RDS"
  value       = aws_security_group.rds.id
}

output "elasticache_security_group_id" {
  description = "Security group ID for ElastiCache"
  value       = aws_security_group.elasticache.id
}

output "mq_security_group_id" {
  description = "Security group ID for Amazon MQ"
  value       = aws_security_group.mq.id
}

output "clickhouse_security_group_id" {
  description = "Security group ID for ClickHouse"
  value       = aws_security_group.clickhouse.id
}

output "fluvio_security_group_id" {
  description = "Security group ID for Fluvio"
  value       = aws_security_group.fluvio.id
}

output "keycloak_security_group_id" {
  description = "Security group ID for Keycloak (null when using Cognito)"
  value       = length(aws_security_group.keycloak) > 0 ? aws_security_group.keycloak[0].id : null
}
