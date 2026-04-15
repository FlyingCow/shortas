variable "environment" {
  description = "Environment name (dev, prod)"
  type        = string
}

variable "vpc_id" {
  description = "VPC ID"
  type        = string
}

variable "public_subnet_ids" {
  description = "List of public subnet IDs"
  type        = list(string)
}

variable "private_subnet_ids" {
  description = "List of private subnet IDs"
  type        = list(string)
}

variable "public_alb_security_group_id" {
  description = "Security group ID for public ALB"
  type        = string
}

variable "internal_alb_security_group_id" {
  description = "Security group ID for internal ALB"
  type        = string
}

variable "certificate_arn" {
  description = "ACM certificate ARN for HTTPS"
  type        = string
}

variable "access_logs_bucket" {
  description = "S3 bucket for ALB access logs"
  type        = string
  default     = ""
}

variable "enable_access_logs" {
  description = "Enable ALB access logs"
  type        = bool
  default     = false
}

# Host headers for routing
variable "api_host_headers" {
  description = "Host headers for API routing"
  type        = list(string)
  default     = ["api.shortas.io", "api.shortas.dev"]
}

variable "dashboard_host_headers" {
  description = "Host headers for dashboard routing"
  type        = list(string)
  default     = ["app.shortas.io", "app.shortas.dev"]
}

variable "landing_host_headers" {
  description = "Host headers for landing page routing"
  type        = list(string)
  default     = ["www.shortas.io", "shortas.io", "www.shortas.dev", "shortas.dev"]
}

variable "keycloak_host_headers" {
  description = "Host headers for Keycloak routing (optional - not used when using Cognito)"
  type        = list(string)
  default     = []
}
