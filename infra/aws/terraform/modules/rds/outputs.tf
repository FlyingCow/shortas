output "cluster_endpoint" {
  description = "The cluster endpoint"
  value       = aws_rds_cluster.main.endpoint
}

output "cluster_reader_endpoint" {
  description = "The cluster reader endpoint"
  value       = aws_rds_cluster.main.reader_endpoint
}

output "cluster_port" {
  description = "The cluster port"
  value       = aws_rds_cluster.main.port
}

output "cluster_id" {
  description = "The cluster identifier"
  value       = aws_rds_cluster.main.id
}

output "cluster_arn" {
  description = "The cluster ARN"
  value       = aws_rds_cluster.main.arn
}

output "database_name" {
  description = "Name of the database"
  value       = aws_rds_cluster.main.database_name
}

output "master_username" {
  description = "Master username"
  value       = aws_rds_cluster.main.master_username
  sensitive   = true
}

output "secret_arn" {
  description = "ARN of the Secrets Manager secret containing credentials"
  value       = aws_secretsmanager_secret.rds.arn
}

output "secret_name" {
  description = "Name of the Secrets Manager secret"
  value       = aws_secretsmanager_secret.rds.name
}
