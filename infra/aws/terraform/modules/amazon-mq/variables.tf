variable "environment" {
  description = "Environment name (dev, prod)"
  type        = string
}

variable "subnet_ids" {
  description = "List of subnet IDs for the broker"
  type        = list(string)
}

variable "security_group_id" {
  description = "Security group ID for the broker"
  type        = string
}

variable "engine_version" {
  description = "RabbitMQ engine version"
  type        = string
  default     = "3.11.20"
}

variable "host_instance_type" {
  description = "Broker instance type"
  type        = string
  default     = "mq.t3.micro"
}

variable "deployment_mode" {
  description = "Deployment mode: SINGLE_INSTANCE or CLUSTER_MULTI_AZ"
  type        = string
  default     = "SINGLE_INSTANCE"
}

variable "admin_username" {
  description = "Admin username for RabbitMQ"
  type        = string
  default     = "shortas_admin"
}

variable "use_aws_owned_key" {
  description = "Use AWS owned key for encryption"
  type        = bool
  default     = true
}

variable "kms_key_id" {
  description = "KMS key ID for encryption (if not using AWS owned key)"
  type        = string
  default     = null
}

variable "log_retention_days" {
  description = "CloudWatch log retention in days"
  type        = number
  default     = 14
}

variable "enable_cloudwatch_alarms" {
  description = "Enable CloudWatch alarms"
  type        = bool
  default     = true
}

variable "alarm_actions" {
  description = "List of ARNs to notify when alarm triggers"
  type        = list(string)
  default     = []
}

variable "message_count_threshold" {
  description = "Threshold for message count alarm"
  type        = number
  default     = 10000
}
