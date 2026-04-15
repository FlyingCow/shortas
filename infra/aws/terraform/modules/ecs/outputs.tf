output "cluster_id" {
  description = "ECS cluster ID"
  value       = aws_ecs_cluster.main.id
}

output "cluster_arn" {
  description = "ECS cluster ARN"
  value       = aws_ecs_cluster.main.arn
}

output "cluster_name" {
  description = "ECS cluster name"
  value       = aws_ecs_cluster.main.name
}

output "service_discovery_namespace_id" {
  description = "Service discovery namespace ID"
  value       = aws_service_discovery_private_dns_namespace.main.id
}

output "service_discovery_namespace_name" {
  description = "Service discovery namespace name"
  value       = aws_service_discovery_private_dns_namespace.main.name
}

output "efs_file_system_id" {
  description = "EFS file system ID for Fluvio"
  value       = var.enable_fluvio ? aws_efs_file_system.fluvio[0].id : null
}

output "service_arns" {
  description = "Map of service names to ARNs"
  value       = { for k, v in aws_ecs_service.services : k => v.id }
}

output "task_definition_arns" {
  description = "Map of service names to task definition ARNs"
  value       = { for k, v in aws_ecs_task_definition.services : k => v.arn }
}

output "log_group_names" {
  description = "Map of service names to log group names"
  value       = { for k, v in aws_cloudwatch_log_group.services : k => v.name }
}

output "fluvio_sc_service_arn" {
  description = "Fluvio SC service ARN"
  value       = var.enable_fluvio ? aws_ecs_service.fluvio_sc[0].id : null
}

output "fluvio_spu_service_arn" {
  description = "Fluvio SPU service ARN"
  value       = var.enable_fluvio ? aws_ecs_service.fluvio_spu[0].id : null
}

output "keycloak_service_arn" {
  description = "Keycloak service ARN"
  value       = var.enable_keycloak ? aws_ecs_service.keycloak[0].id : null
}
