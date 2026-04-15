# Route53 Module for Shortas URL Shortener

# Hosted zone (optional - may already exist)
resource "aws_route53_zone" "main" {
  count = var.create_hosted_zone ? 1 : 0
  name  = var.domain_name

  tags = {
    Name        = "${var.environment}-shortas-zone"
    Environment = var.environment
  }
}

locals {
  zone_id = var.create_hosted_zone ? aws_route53_zone.main[0].zone_id : var.existing_zone_id
}

# A record for root domain -> ALB
resource "aws_route53_record" "root" {
  count   = var.create_root_record ? 1 : 0
  zone_id = local.zone_id
  name    = var.domain_name
  type    = "A"

  alias {
    name                   = var.alb_dns_name
    zone_id                = var.alb_zone_id
    evaluate_target_health = true
  }
}

# A record for www subdomain -> ALB
resource "aws_route53_record" "www" {
  count   = var.create_www_record ? 1 : 0
  zone_id = local.zone_id
  name    = "www.${var.domain_name}"
  type    = "A"

  alias {
    name                   = var.alb_dns_name
    zone_id                = var.alb_zone_id
    evaluate_target_health = true
  }
}

# A records for subdomains
resource "aws_route53_record" "subdomains" {
  for_each = var.subdomain_records

  zone_id = local.zone_id
  name    = "${each.key}.${var.domain_name}"
  type    = "A"

  alias {
    name                   = each.value.target_dns_name
    zone_id                = each.value.target_zone_id
    evaluate_target_health = true
  }
}

# Health check for the main endpoint
resource "aws_route53_health_check" "main" {
  count             = var.create_health_check ? 1 : 0
  fqdn              = var.domain_name
  port              = 443
  type              = "HTTPS"
  resource_path     = var.health_check_path
  failure_threshold = 3
  request_interval  = 30

  tags = {
    Name        = "${var.environment}-shortas-health-check"
    Environment = var.environment
  }
}

# CloudWatch alarm for health check
resource "aws_cloudwatch_metric_alarm" "health_check" {
  count               = var.create_health_check ? 1 : 0
  alarm_name          = "${var.environment}-shortas-health-check"
  comparison_operator = "LessThanThreshold"
  evaluation_periods  = 2
  metric_name         = "HealthCheckStatus"
  namespace           = "AWS/Route53"
  period              = 60
  statistic           = "Minimum"
  threshold           = 1
  alarm_description   = "Health check failed for ${var.domain_name}"

  dimensions = {
    HealthCheckId = aws_route53_health_check.main[0].id
  }

  alarm_actions = var.alarm_actions

  tags = {
    Name        = "${var.environment}-shortas-health-check-alarm"
    Environment = var.environment
  }
}
