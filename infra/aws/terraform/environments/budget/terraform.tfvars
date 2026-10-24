# Budget Environment Configuration
#
# This environment is meant to be spun up right before a live demo/
# presentation and torn down right after (terraform apply / terraform
# destroy) - it runs the full feature set, just on the cheapest possible
# sizing (see main.tf). Nothing here is sized for sustained real traffic.

region      = "us-east-1"
domain_name = "shortas-budget.dev"

subject_alternative_names = [
  "*.shortas-budget.dev"
]

# Set these if you have a Route53 hosted zone
create_route53_records = false
route53_zone_id        = ""

image_tag = "latest"

# Add email addresses for alarm notifications
alarm_email_endpoints = []
