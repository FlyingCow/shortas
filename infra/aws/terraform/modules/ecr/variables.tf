variable "environment" {
  description = "Environment name (dev, prod)"
  type        = string
}

variable "service_names" {
  description = "List of service names to create repositories for"
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
    "pages"
  ]
}

variable "scan_on_push" {
  description = "Enable image scanning on push"
  type        = bool
  default     = true
}

variable "image_count_to_keep" {
  description = "Number of tagged images to keep"
  type        = number
  default     = 30
}

variable "untagged_image_expiry_days" {
  description = "Days after which untagged images expire"
  type        = number
  default     = 7
}

variable "enable_pull_through_cache" {
  description = "Enable pull through cache for Docker Hub"
  type        = bool
  default     = false
}
