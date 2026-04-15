output "ecs_task_execution_role_arn" {
  description = "ARN of the ECS task execution role"
  value       = aws_iam_role.ecs_task_execution.arn
}

output "ecs_task_execution_role_name" {
  description = "Name of the ECS task execution role"
  value       = aws_iam_role.ecs_task_execution.name
}

output "ecs_task_role_arn" {
  description = "ARN of the ECS task role"
  value       = aws_iam_role.ecs_task.arn
}

output "ecs_task_role_name" {
  description = "Name of the ECS task role"
  value       = aws_iam_role.ecs_task.name
}

output "clickhouse_role_arn" {
  description = "ARN of the ClickHouse EC2 role"
  value       = aws_iam_role.clickhouse.arn
}

output "clickhouse_instance_profile_arn" {
  description = "ARN of the ClickHouse instance profile"
  value       = aws_iam_instance_profile.clickhouse.arn
}

output "clickhouse_instance_profile_name" {
  description = "Name of the ClickHouse instance profile"
  value       = aws_iam_instance_profile.clickhouse.name
}
