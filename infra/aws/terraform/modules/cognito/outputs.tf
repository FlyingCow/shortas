output "user_pool_id" {
  description = "Cognito User Pool ID"
  value       = aws_cognito_user_pool.main.id
}

output "user_pool_arn" {
  description = "Cognito User Pool ARN"
  value       = aws_cognito_user_pool.main.arn
}

output "user_pool_endpoint" {
  description = "Cognito User Pool endpoint"
  value       = aws_cognito_user_pool.main.endpoint
}

output "user_pool_domain" {
  description = "Cognito User Pool domain"
  value       = aws_cognito_user_pool_domain.main.domain
}

output "hosted_ui_url" {
  description = "Cognito Hosted UI URL"
  value       = "https://${aws_cognito_user_pool_domain.main.domain}.auth.${data.aws_region.current.name}.amazoncognito.com"
}

output "dashboard_client_id" {
  description = "Dashboard app client ID"
  value       = aws_cognito_user_pool_client.dashboard.id
}

output "api_client_id" {
  description = "API app client ID"
  value       = aws_cognito_user_pool_client.api.id
}

output "api_client_secret" {
  description = "API app client secret"
  value       = aws_cognito_user_pool_client.api.client_secret
  sensitive   = true
}

output "issuer_url" {
  description = "OIDC issuer URL for token validation"
  value       = "https://cognito-idp.${data.aws_region.current.name}.amazonaws.com/${aws_cognito_user_pool.main.id}"
}

output "jwks_url" {
  description = "JWKS URL for token validation"
  value       = "https://cognito-idp.${data.aws_region.current.name}.amazonaws.com/${aws_cognito_user_pool.main.id}/.well-known/jwks.json"
}

output "identity_pool_id" {
  description = "Cognito Identity Pool ID (if enabled)"
  value       = var.enable_identity_pool ? aws_cognito_identity_pool.main[0].id : null
}

output "authenticated_role_arn" {
  description = "IAM role ARN for authenticated users (if identity pool enabled)"
  value       = var.enable_identity_pool ? aws_iam_role.authenticated[0].arn : null
}

# Data source for current region
data "aws_region" "current" {}
