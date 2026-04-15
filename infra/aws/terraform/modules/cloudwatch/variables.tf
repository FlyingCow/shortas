variable "environment" {
  description = "Environment name (dev, prod)"
  type        = string
}

variable "region" {
  description = "AWS region"
  type        = string
  default     = "us-east-1"
}

variable "ecs_cluster_name" {
  description = "ECS cluster name"
  type        = string
}

variable "ecs_service_names" {
  description = "List of ECS service names"
  type        = list(string)
  default     = []
}

variable "alb_arn_suffix" {
  description = "ALB ARN suffix for CloudWatch metrics"
  type        = string
}

variable "alarm_email_endpoints" {
  description = "Email addresses for alarm notifications"
  type        = list(string)
  default     = []
}

variable "alb_5xx_threshold" {
  description = "Threshold for ALB 5xx errors"
  type        = number
  default     = 10
}

variable "target_5xx_threshold" {
  description = "Threshold for target 5xx errors"
  type        = number
  default     = 50
}

variable "response_time_threshold" {
  description = "Threshold for response time (seconds)"
  type        = number
  default     = 2
}

variable "create_composite_alarm" {
  description = "Create composite alarm for service health"
  type        = bool
  default     = true
}
