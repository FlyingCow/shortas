variable "environment" {
  description = "Environment name (dev, prod)"
  type        = string
}

variable "project_name" {
  description = "Project name for bucket naming"
  type        = string
  default     = "shortas"
}

variable "cors_allowed_origins" {
  description = "List of allowed origins for CORS"
  type        = list(string)
  default     = ["*"]
}

variable "enable_versioning" {
  description = "Enable versioning for route images bucket"
  type        = bool
  default     = false
}

variable "enable_lifecycle_rules" {
  description = "Enable lifecycle rules"
  type        = bool
  default     = true
}

variable "backup_transition_days" {
  description = "Days before transitioning backups to Glacier"
  type        = number
  default     = 30
}

variable "backup_expiration_days" {
  description = "Days before expiring old backups"
  type        = number
  default     = 365
}

variable "create_terraform_state_bucket" {
  description = "Create S3 bucket for Terraform state"
  type        = bool
  default     = false
}

variable "create_logs_bucket" {
  description = "Create S3 bucket for application logs"
  type        = bool
  default     = false
}
