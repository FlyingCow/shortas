variable "environment" {
  description = "Environment name (dev, prod)"
  type        = string
}

variable "region" {
  description = "AWS region"
  type        = string
  default     = "us-east-1"
}

variable "vpc_id" {
  description = "VPC ID"
  type        = string
}

variable "private_subnet_ids" {
  description = "List of private subnet IDs"
  type        = list(string)
}

variable "ecs_security_group_id" {
  description = "Security group ID for ECS services"
  type        = string
}

variable "fluvio_security_group_id" {
  description = "Security group ID for Fluvio services"
  type        = string
}

variable "keycloak_security_group_id" {
  description = "Security group ID for Keycloak (only needed if enable_keycloak = true)"
  type        = string
  default     = null
}

variable "task_execution_role_arn" {
  description = "ARN of the ECS task execution role"
  type        = string
}

variable "task_role_arn" {
  description = "ARN of the ECS task role"
  type        = string
}

variable "ecr_repository_url" {
  description = "Base URL for ECR repositories"
  type        = string
}

variable "image_tag" {
  description = "Docker image tag to deploy"
  type        = string
  default     = "latest"
}

variable "log_retention_days" {
  description = "CloudWatch log retention in days"
  type        = number
  default     = 14
}

variable "enable_container_insights" {
  description = "Enable Container Insights"
  type        = bool
  default     = true
}

variable "enable_ecs_exec" {
  description = "Enable ECS Exec for debugging"
  type        = bool
  default     = true
}

variable "use_spot_instances" {
  description = "Use Fargate Spot for cost savings"
  type        = bool
  default     = false
}

# Service names for log groups and service discovery
variable "service_names" {
  description = "List of all service names"
  type        = list(string)
  default = [
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

# Service configurations
variable "services" {
  description = "List of service configurations"
  type = list(object({
    name                  = string
    cpu                   = number
    memory                = number
    desired_count         = number
    min_count             = optional(number, 1)
    max_count             = optional(number, 10)
    container_port        = optional(number)
    target_group_arn      = optional(string)
    enable_autoscaling    = optional(bool, false)
    cpu_target_value      = optional(number, 70)
    environment_variables = optional(list(object({
      name  = string
      value = string
    })), [])
    secrets = optional(list(object({
      name      = string
      valueFrom = string
    })), [])
    health_check = optional(object({
      command      = list(string)
      interval     = number
      timeout      = number
      retries      = number
      start_period = number
    }))
  }))
  default = []
}

# Common secrets for all services
variable "common_secrets" {
  description = "Secrets to inject into all services"
  type = list(object({
    name      = string
    valueFrom = string
  }))
  default = []
}

# Fluvio configuration
variable "enable_fluvio" {
  description = "Enable Fluvio streaming platform"
  type        = bool
  default     = true
}

variable "fluvio_image" {
  description = "Fluvio Docker image"
  type        = string
  default     = "infinyon/fluvio:stable"
}

variable "fluvio_sc_cpu" {
  description = "CPU units for Fluvio SC"
  type        = number
  default     = 512
}

variable "fluvio_sc_memory" {
  description = "Memory for Fluvio SC"
  type        = number
  default     = 1024
}

variable "fluvio_spu_count" {
  description = "Number of Fluvio SPU instances"
  type        = number
  default     = 2
}

variable "fluvio_spu_cpu" {
  description = "CPU units for Fluvio SPU"
  type        = number
  default     = 512
}

variable "fluvio_spu_memory" {
  description = "Memory for Fluvio SPU"
  type        = number
  default     = 1024
}

# Keycloak configuration
variable "enable_keycloak" {
  description = "Enable Keycloak authentication service"
  type        = bool
  default     = true
}

variable "keycloak_image" {
  description = "Keycloak Docker image"
  type        = string
  default     = "quay.io/keycloak/keycloak:22.0"
}

variable "keycloak_cpu" {
  description = "CPU units for Keycloak"
  type        = number
  default     = 1024
}

variable "keycloak_memory" {
  description = "Memory for Keycloak"
  type        = number
  default     = 2048
}

variable "keycloak_desired_count" {
  description = "Desired count for Keycloak"
  type        = number
  default     = 1
}

variable "keycloak_target_group_arn" {
  description = "Target group ARN for Keycloak"
  type        = string
  default     = null
}

variable "rds_secret_arn" {
  description = "ARN of RDS secret in Secrets Manager"
  type        = string
  default     = ""
}

variable "keycloak_secret_arn" {
  description = "ARN of Keycloak admin credentials secret"
  type        = string
  default     = ""
}

# Cognito configuration (alternative to Keycloak)
variable "cognito_user_pool_id" {
  description = "Cognito User Pool ID (when using Cognito instead of Keycloak)"
  type        = string
  default     = null
}

variable "cognito_issuer_url" {
  description = "Cognito OIDC issuer URL"
  type        = string
  default     = null
}

variable "cognito_jwks_url" {
  description = "Cognito JWKS URL for token validation"
  type        = string
  default     = null
}

variable "cognito_dashboard_client_id" {
  description = "Cognito app client ID for dashboard"
  type        = string
  default     = null
}
