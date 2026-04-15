variable "environment" {
  description = "Environment name (dev, prod)"
  type        = string
}

variable "region" {
  description = "AWS region"
  type        = string
  default     = "us-east-1"
}

variable "subnet_id" {
  description = "Subnet ID for the instance"
  type        = string
}

variable "availability_zone" {
  description = "Availability zone for the instance"
  type        = string
}

variable "security_group_id" {
  description = "Security group ID for the instance"
  type        = string
}

variable "instance_profile_name" {
  description = "IAM instance profile name"
  type        = string
}

variable "instance_type" {
  description = "EC2 instance type"
  type        = string
  default     = "r6i.large"
}

variable "ami_id" {
  description = "Custom AMI ID (leave empty for latest Amazon Linux 2)"
  type        = string
  default     = ""
}

variable "key_name" {
  description = "EC2 key pair name"
  type        = string
  default     = null
}

variable "clickhouse_version" {
  description = "ClickHouse version to install"
  type        = string
  default     = "latest"
}

variable "clickhouse_password" {
  description = "ClickHouse default user password"
  type        = string
  sensitive   = true
}

variable "data_volume_size" {
  description = "Size of the data volume in GB"
  type        = number
  default     = 100
}

variable "data_volume_type" {
  description = "Type of the data volume"
  type        = string
  default     = "gp3"
}

variable "data_volume_iops" {
  description = "IOPS for the data volume (for io1/io2)"
  type        = number
  default     = 3000
}

variable "s3_backup_bucket" {
  description = "S3 bucket for backups"
  type        = string
}

variable "assign_elastic_ip" {
  description = "Assign an Elastic IP to the instance"
  type        = bool
  default     = false
}

variable "enable_detailed_monitoring" {
  description = "Enable detailed monitoring"
  type        = bool
  default     = true
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
