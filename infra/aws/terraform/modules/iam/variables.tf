variable "environment" {
  description = "Environment name (dev, prod)"
  type        = string
}

variable "region" {
  description = "AWS region"
  type        = string
  default     = "us-east-1"
}

variable "dynamodb_table_arns" {
  description = "List of DynamoDB table ARNs to grant access to"
  type        = list(string)
  default     = []
}

variable "s3_bucket_arns" {
  description = "List of S3 bucket ARNs to grant access to"
  type        = list(string)
  default     = []
}

variable "kms_key_arns" {
  description = "List of KMS key ARNs for decryption"
  type        = list(string)
  default     = ["*"]
}

variable "enable_ecs_exec" {
  description = "Enable ECS Exec for debugging"
  type        = bool
  default     = true
}

variable "efs_file_system_arn" {
  description = "EFS file system ARN for Fluvio"
  type        = string
  default     = null
}

variable "efs_access_point_arns" {
  description = "List of EFS access point ARNs"
  type        = list(string)
  default     = []
}

variable "clickhouse_backup_bucket_arn" {
  description = "S3 bucket ARN for ClickHouse backups"
  type        = string
  default     = ""
}
