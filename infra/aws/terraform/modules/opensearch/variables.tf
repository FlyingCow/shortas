variable "environment" {
  description = "Environment name (dev, prod, budget)"
  type        = string
}

variable "subnet_ids" {
  description = "Subnet IDs for the domain. One per AZ when zone_awareness_enabled is true."
  type        = list(string)
}

variable "security_group_id" {
  description = "Security group ID for the OpenSearch domain"
  type        = string
}

variable "engine_version" {
  description = "OpenSearch engine version"
  type        = string
  default     = "OpenSearch_2.17"
}

variable "instance_type" {
  description = "Instance type for data nodes"
  type        = string
  default     = "t3.small.search"
}

variable "instance_count" {
  description = "Number of data nodes"
  type        = number
  default     = 1
}

variable "zone_awareness_enabled" {
  description = "Spread data nodes across multiple AZs (requires instance_count >= 2 and len(subnet_ids) >= 2)"
  type        = bool
  default     = false
}

variable "ebs_volume_size" {
  description = "EBS volume size per node, in GB"
  type        = number
  default     = 20
}

variable "enable_cloudwatch_alarms" {
  description = "Enable CloudWatch alarms for the domain"
  type        = bool
  default     = true
}

variable "alarm_actions" {
  description = "List of ARNs to notify when an alarm triggers"
  type        = list(string)
  default     = []
}
