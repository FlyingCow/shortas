# Secrets Manager Module for Shortas URL Shortener

# Keycloak Admin Credentials (conditional - only when not using Cognito)
resource "random_password" "keycloak_admin" {
  count = var.enable_keycloak ? 1 : 0

  length           = 32
  special          = true
  override_special = "!#$%&*()-_=+[]{}?"
}

resource "aws_secretsmanager_secret" "keycloak" {
  count = var.enable_keycloak ? 1 : 0

  name                    = "shortas/${var.environment}/keycloak"
  description             = "Keycloak admin credentials for Shortas"
  recovery_window_in_days = var.environment == "prod" ? 30 : 0

  tags = {
    Name        = "shortas-${var.environment}-keycloak-secret"
    Environment = var.environment
  }
}

resource "aws_secretsmanager_secret_version" "keycloak" {
  count = var.enable_keycloak ? 1 : 0

  secret_id = aws_secretsmanager_secret.keycloak[0].id
  secret_string = jsonencode({
    admin_username = "admin"
    admin_password = random_password.keycloak_admin[0].result
  })
}

# ClickHouse Credentials
resource "random_password" "clickhouse" {
  length           = 32
  special          = true
  override_special = "!#$%&*()-_=+[]{}?"
}

resource "aws_secretsmanager_secret" "clickhouse" {
  name                    = "shortas/${var.environment}/clickhouse"
  description             = "ClickHouse credentials for Shortas"
  recovery_window_in_days = var.environment == "prod" ? 30 : 0

  tags = {
    Name        = "shortas-${var.environment}-clickhouse-secret"
    Environment = var.environment
  }
}

resource "aws_secretsmanager_secret_version" "clickhouse" {
  secret_id = aws_secretsmanager_secret.clickhouse.id
  secret_string = jsonencode({
    username = "default"
    password = random_password.clickhouse.result
    host     = var.clickhouse_host
    port     = 8123
    database = "shortas"
  })
}

# API Keys / JWT Secrets
resource "random_password" "jwt_secret" {
  length  = 64
  special = false
}

resource "aws_secretsmanager_secret" "api" {
  name                    = "shortas/${var.environment}/api"
  description             = "API secrets for Shortas"
  recovery_window_in_days = var.environment == "prod" ? 30 : 0

  tags = {
    Name        = "shortas-${var.environment}-api-secret"
    Environment = var.environment
  }
}

resource "aws_secretsmanager_secret_version" "api" {
  secret_id = aws_secretsmanager_secret.api.id
  secret_string = jsonencode({
    jwt_secret           = random_password.jwt_secret.result
    encryption_key       = var.encryption_key != "" ? var.encryption_key : random_password.jwt_secret.result
    api_key              = var.api_key
  })
}

# Application Config Secret (for environment-specific configs)
resource "aws_secretsmanager_secret" "app_config" {
  name                    = "shortas/${var.environment}/app-config"
  description             = "Application configuration for Shortas"
  recovery_window_in_days = var.environment == "prod" ? 30 : 0

  tags = {
    Name        = "shortas-${var.environment}-app-config-secret"
    Environment = var.environment
  }
}

resource "aws_secretsmanager_secret_version" "app_config" {
  secret_id = aws_secretsmanager_secret.app_config.id
  secret_string = jsonencode(merge(
    {
      environment = var.environment
    },
    var.additional_app_config
  ))
}
