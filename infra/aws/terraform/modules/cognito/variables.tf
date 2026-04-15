variable "environment" {
  description = "Environment name (dev, prod)"
  type        = string
}

# Password Policy
variable "password_minimum_length" {
  description = "Minimum password length"
  type        = number
  default     = 8
}

variable "password_require_lowercase" {
  description = "Require lowercase letters in password"
  type        = bool
  default     = true
}

variable "password_require_uppercase" {
  description = "Require uppercase letters in password"
  type        = bool
  default     = true
}

variable "password_require_numbers" {
  description = "Require numbers in password"
  type        = bool
  default     = true
}

variable "password_require_symbols" {
  description = "Require symbols in password"
  type        = bool
  default     = false
}

# MFA Configuration
variable "mfa_configuration" {
  description = "MFA configuration (OFF, ON, OPTIONAL)"
  type        = string
  default     = "OPTIONAL"
}

# Advanced Security
variable "advanced_security_mode" {
  description = "Advanced security mode (OFF, AUDIT, ENFORCED)"
  type        = string
  default     = "AUDIT"
}

# Email Configuration
variable "ses_email_identity" {
  description = "SES email identity ARN for sending emails"
  type        = string
  default     = null
}

variable "from_email_address" {
  description = "From email address for Cognito emails"
  type        = string
  default     = null
}

# Domain Configuration
variable "custom_domain" {
  description = "Custom domain for Cognito hosted UI (requires ACM certificate)"
  type        = string
  default     = null
}

variable "certificate_arn" {
  description = "ACM certificate ARN for custom domain"
  type        = string
  default     = null
}

# Callback URLs
variable "dashboard_callback_urls" {
  description = "Allowed callback URLs for dashboard app"
  type        = list(string)
  default     = ["http://localhost:3000/callback"]
}

variable "dashboard_logout_urls" {
  description = "Allowed logout URLs for dashboard app"
  type        = list(string)
  default     = ["http://localhost:3000"]
}

# Token Validity
variable "access_token_validity_hours" {
  description = "Access token validity in hours"
  type        = number
  default     = 1
}

variable "id_token_validity_hours" {
  description = "ID token validity in hours"
  type        = number
  default     = 1
}

variable "refresh_token_validity_days" {
  description = "Refresh token validity in days"
  type        = number
  default     = 30
}

# Identity Pool
variable "enable_identity_pool" {
  description = "Enable Cognito Identity Pool for federated AWS access"
  type        = bool
  default     = false
}
