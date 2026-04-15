variable "environment" {
  description = "Environment name (dev, prod)"
  type        = string
}

variable "billing_mode" {
  description = "DynamoDB billing mode (PAY_PER_REQUEST or PROVISIONED)"
  type        = string
  default     = "PAY_PER_REQUEST"
}

variable "enable_point_in_time_recovery" {
  description = "Enable point-in-time recovery"
  type        = bool
  default     = true
}

variable "enable_ttl" {
  description = "Enable TTL on tables"
  type        = bool
  default     = false
}

variable "enable_autoscaling" {
  description = "Enable auto-scaling (only for PROVISIONED billing mode)"
  type        = bool
  default     = false
}

# Routes table capacity
variable "routes_read_capacity" {
  description = "Read capacity units for routes table"
  type        = number
  default     = 5
}

variable "routes_write_capacity" {
  description = "Write capacity units for routes table"
  type        = number
  default     = 5
}

variable "routes_read_max_capacity" {
  description = "Max read capacity units for routes table autoscaling"
  type        = number
  default     = 100
}

variable "routes_write_max_capacity" {
  description = "Max write capacity units for routes table autoscaling"
  type        = number
  default     = 50
}

# Encryption table capacity
variable "encryption_read_capacity" {
  description = "Read capacity units for encryption table"
  type        = number
  default     = 5
}

variable "encryption_write_capacity" {
  description = "Write capacity units for encryption table"
  type        = number
  default     = 2
}

# Hostname mapping table capacity
variable "hostname_read_capacity" {
  description = "Read capacity units for hostname mapping table"
  type        = number
  default     = 5
}

variable "hostname_write_capacity" {
  description = "Write capacity units for hostname mapping table"
  type        = number
  default     = 2
}

# User settings table capacity
variable "user_settings_read_capacity" {
  description = "Read capacity units for user settings table"
  type        = number
  default     = 5
}

variable "user_settings_write_capacity" {
  description = "Write capacity units for user settings table"
  type        = number
  default     = 2
}

# Domains table capacity
variable "domains_read_capacity" {
  description = "Read capacity units for domains table"
  type        = number
  default     = 5
}

variable "domains_write_capacity" {
  description = "Write capacity units for domains table"
  type        = number
  default     = 2
}

# Routes to verify table capacity
variable "routes_verify_read_capacity" {
  description = "Read capacity units for routes to verify table"
  type        = number
  default     = 5
}

variable "routes_verify_write_capacity" {
  description = "Write capacity units for routes to verify table"
  type        = number
  default     = 2
}

# Certificate orders table capacity
variable "orders_read_capacity" {
  description = "Read capacity units for certificate orders table"
  type        = number
  default     = 5
}

variable "orders_write_capacity" {
  description = "Write capacity units for certificate orders table"
  type        = number
  default     = 2
}

# Challenges table capacity
variable "challenges_read_capacity" {
  description = "Read capacity units for challenges table"
  type        = number
  default     = 10
}

variable "challenges_write_capacity" {
  description = "Write capacity units for challenges table"
  type        = number
  default     = 5
}

# Alarms
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
