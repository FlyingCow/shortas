output "keycloak_secret_arn" {
  description = "ARN of the Keycloak secret (null when using Cognito)"
  value       = length(aws_secretsmanager_secret.keycloak) > 0 ? aws_secretsmanager_secret.keycloak[0].arn : null
}

output "keycloak_secret_name" {
  description = "Name of the Keycloak secret (null when using Cognito)"
  value       = length(aws_secretsmanager_secret.keycloak) > 0 ? aws_secretsmanager_secret.keycloak[0].name : null
}

output "clickhouse_secret_arn" {
  description = "ARN of the ClickHouse secret"
  value       = aws_secretsmanager_secret.clickhouse.arn
}

output "clickhouse_secret_name" {
  description = "Name of the ClickHouse secret"
  value       = aws_secretsmanager_secret.clickhouse.name
}

output "api_secret_arn" {
  description = "ARN of the API secret"
  value       = aws_secretsmanager_secret.api.arn
}

output "api_secret_name" {
  description = "Name of the API secret"
  value       = aws_secretsmanager_secret.api.name
}

output "app_config_secret_arn" {
  description = "ARN of the app config secret"
  value       = aws_secretsmanager_secret.app_config.arn
}

output "app_config_secret_name" {
  description = "Name of the app config secret"
  value       = aws_secretsmanager_secret.app_config.name
}

output "all_secret_arns" {
  description = "List of all secret ARNs"
  value = concat(
    length(aws_secretsmanager_secret.keycloak) > 0 ? [aws_secretsmanager_secret.keycloak[0].arn] : [],
    [
      aws_secretsmanager_secret.clickhouse.arn,
      aws_secretsmanager_secret.api.arn,
      aws_secretsmanager_secret.app_config.arn
    ]
  )
}
