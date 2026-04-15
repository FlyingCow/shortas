variable "environment" {
  description = "Environment name (dev, prod)"
  type        = string
}

variable "domain_name" {
  description = "Primary domain name for the certificate"
  type        = string
}

variable "subject_alternative_names" {
  description = "List of subject alternative names"
  type        = list(string)
  default     = []
}

variable "create_route53_records" {
  description = "Create Route53 records for DNS validation"
  type        = bool
  default     = true
}

variable "route53_zone_id" {
  description = "Route53 zone ID for DNS validation"
  type        = string
  default     = ""
}
