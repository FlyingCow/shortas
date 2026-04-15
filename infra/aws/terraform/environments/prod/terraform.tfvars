# Production Environment Configuration

region      = "us-east-1"
domain_name = "shortas.io"

subject_alternative_names = [
  "*.shortas.io"
]

# Set these to your Route53 hosted zone
create_route53_records = true
route53_zone_id        = ""  # UPDATE: Set your zone ID

image_tag = "latest"

# Add email addresses for alarm notifications
alarm_email_endpoints = []  # UPDATE: Add your team email addresses

cors_allowed_origins = [
  "https://shortas.io",
  "https://www.shortas.io",
  "https://app.shortas.io",
  "https://api.shortas.io"
]
