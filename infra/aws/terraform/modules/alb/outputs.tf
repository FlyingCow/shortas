output "public_alb_arn" {
  description = "ARN of the public ALB"
  value       = aws_lb.public.arn
}

output "public_alb_dns_name" {
  description = "DNS name of the public ALB"
  value       = aws_lb.public.dns_name
}

output "public_alb_zone_id" {
  description = "Zone ID of the public ALB"
  value       = aws_lb.public.zone_id
}

output "internal_alb_arn" {
  description = "ARN of the internal ALB"
  value       = aws_lb.internal.arn
}

output "internal_alb_dns_name" {
  description = "DNS name of the internal ALB"
  value       = aws_lb.internal.dns_name
}

output "https_listener_arn" {
  description = "ARN of the HTTPS listener"
  value       = aws_lb_listener.https.arn
}

output "http_listener_arn" {
  description = "ARN of the HTTP listener"
  value       = aws_lb_listener.http.arn
}

output "internal_http_listener_arn" {
  description = "ARN of the internal HTTP listener"
  value       = aws_lb_listener.internal_http.arn
}

# Target Group ARNs
output "click_router_target_group_arn" {
  description = "Target group ARN for click-router"
  value       = aws_lb_target_group.click_router.arn
}

output "click_router_api_target_group_arn" {
  description = "Target group ARN for click-router-api"
  value       = aws_lb_target_group.click_router_api.arn
}

output "aggregator_api_target_group_arn" {
  description = "Target group ARN for aggregator-api"
  value       = aws_lb_target_group.aggregator_api.arn
}

output "shortas_api_target_group_arn" {
  description = "Target group ARN for shortas-api"
  value       = aws_lb_target_group.shortas_api.arn
}

output "dashboard_target_group_arn" {
  description = "Target group ARN for dashboard"
  value       = aws_lb_target_group.dashboard.arn
}

output "landing_target_group_arn" {
  description = "Target group ARN for landing"
  value       = aws_lb_target_group.landing.arn
}

output "pages_target_group_arn" {
  description = "Target group ARN for pages"
  value       = aws_lb_target_group.pages.arn
}

output "keycloak_target_group_arn" {
  description = "Target group ARN for keycloak (null when using Cognito)"
  value       = length(aws_lb_target_group.keycloak) > 0 ? aws_lb_target_group.keycloak[0].arn : null
}

output "domain_verifier_target_group_arn" {
  description = "Target group ARN for domain-verifier"
  value       = aws_lb_target_group.domain_verifier.arn
}

output "target_groups" {
  description = "Map of all target group ARNs"
  value = merge(
    {
      click_router     = aws_lb_target_group.click_router.arn
      click_router_api = aws_lb_target_group.click_router_api.arn
      aggregator_api   = aws_lb_target_group.aggregator_api.arn
      shortas_api      = aws_lb_target_group.shortas_api.arn
      dashboard        = aws_lb_target_group.dashboard.arn
      landing          = aws_lb_target_group.landing.arn
      pages            = aws_lb_target_group.pages.arn
      domain_verifier  = aws_lb_target_group.domain_verifier.arn
    },
    length(aws_lb_target_group.keycloak) > 0 ? {
      keycloak = aws_lb_target_group.keycloak[0].arn
    } : {}
  )
}
