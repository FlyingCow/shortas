output "zone_id" {
  description = "Route53 zone ID"
  value       = local.zone_id
}

output "zone_name_servers" {
  description = "Name servers for the zone"
  value       = var.create_hosted_zone ? aws_route53_zone.main[0].name_servers : null
}

output "health_check_id" {
  description = "Health check ID"
  value       = var.create_health_check ? aws_route53_health_check.main[0].id : null
}
