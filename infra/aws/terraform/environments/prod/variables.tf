variable "region" {
  description = "AWS region"
  type        = string
  default     = "us-east-1"
}

variable "domain_name" {
  description = "Domain name for the application"
  type        = string
  default     = "shortas.io"
}

variable "subject_alternative_names" {
  description = "Subject alternative names for SSL certificate"
  type        = list(string)
  default     = ["*.shortas.io"]
}

variable "route53_zone_id" {
  description = "Route53 zone ID for DNS records"
  type        = string
  default     = ""
}

variable "create_route53_records" {
  description = "Create Route53 DNS records"
  type        = bool
  default     = true
}

variable "image_tag" {
  description = "Docker image tag to deploy"
  type        = string
  default     = "latest"
}

variable "alarm_email_endpoints" {
  description = "Email addresses for alarm notifications"
  type        = list(string)
  default     = []
}

variable "cors_allowed_origins" {
  description = "Allowed origins for CORS"
  type        = list(string)
  default     = ["https://shortas.io", "https://app.shortas.io", "https://www.shortas.io"]
}
