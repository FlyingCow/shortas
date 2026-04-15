variable "environment" {
  description = "Environment name (dev, prod)"
  type        = string
}

variable "clickhouse_host" {
  description = "ClickHouse host address"
  type        = string
  default     = ""
}

variable "encryption_key" {
  description = "Custom encryption key (leave empty to auto-generate)"
  type        = string
  default     = ""
  sensitive   = true
}

variable "api_key" {
  description = "API key for external services"
  type        = string
  default     = ""
  sensitive   = true
}

variable "additional_app_config" {
  description = "Additional application configuration"
  type        = map(string)
  default     = {}
}

variable "enable_keycloak" {
  description = "Enable Keycloak secrets (set to false when using Cognito)"
  type        = bool
  default     = true
}
