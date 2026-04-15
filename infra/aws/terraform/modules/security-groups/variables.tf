variable "environment" {
  description = "Environment name (dev, prod)"
  type        = string
}

variable "vpc_id" {
  description = "VPC ID where security groups will be created"
  type        = string
}

variable "vpc_cidr" {
  description = "VPC CIDR block"
  type        = string
}

variable "enable_clickhouse_ssh" {
  description = "Enable SSH access to ClickHouse instances"
  type        = bool
  default     = false
}

variable "ssh_cidr_blocks" {
  description = "CIDR blocks allowed for SSH access"
  type        = list(string)
  default     = []
}

variable "enable_keycloak" {
  description = "Enable Keycloak security group (set to false when using Cognito)"
  type        = bool
  default     = true
}
