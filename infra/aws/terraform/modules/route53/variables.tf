variable "environment" {
  description = "Environment name (dev, prod)"
  type        = string
}

variable "domain_name" {
  description = "Domain name"
  type        = string
}

variable "create_hosted_zone" {
  description = "Create a new hosted zone"
  type        = bool
  default     = false
}

variable "existing_zone_id" {
  description = "Existing Route53 zone ID (if not creating new)"
  type        = string
  default     = ""
}

variable "alb_dns_name" {
  description = "DNS name of the ALB"
  type        = string
}

variable "alb_zone_id" {
  description = "Zone ID of the ALB"
  type        = string
}

variable "create_root_record" {
  description = "Create A record for root domain"
  type        = bool
  default     = true
}

variable "create_www_record" {
  description = "Create A record for www subdomain"
  type        = bool
  default     = true
}

variable "subdomain_records" {
  description = "Map of subdomain records to create"
  type = map(object({
    target_dns_name = string
    target_zone_id  = string
  }))
  default = {}
}

variable "create_health_check" {
  description = "Create Route53 health check"
  type        = bool
  default     = true
}

variable "health_check_path" {
  description = "Path for health check"
  type        = string
  default     = "/health"
}

variable "alarm_actions" {
  description = "List of ARNs to notify when alarm triggers"
  type        = list(string)
  default     = []
}
