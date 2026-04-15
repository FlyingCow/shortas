# Dev Environment Configuration

region      = "us-east-1"
domain_name = "shortas.dev"

subject_alternative_names = [
  "*.shortas.dev"
]

# Set these if you have a Route53 hosted zone
create_route53_records = false
route53_zone_id        = ""

image_tag = "latest"

# Add email addresses for alarm notifications
alarm_email_endpoints = []
