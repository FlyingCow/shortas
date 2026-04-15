output "routes_table_name" {
  description = "Name of the routes table"
  value       = aws_dynamodb_table.routes.name
}

output "routes_table_arn" {
  description = "ARN of the routes table"
  value       = aws_dynamodb_table.routes.arn
}

output "routes_encryption_table_name" {
  description = "Name of the routes encryption table"
  value       = aws_dynamodb_table.routes_encryption.name
}

output "routes_encryption_table_arn" {
  description = "ARN of the routes encryption table"
  value       = aws_dynamodb_table.routes_encryption.arn
}

output "hostname_mapping_table_name" {
  description = "Name of the hostname mapping table"
  value       = aws_dynamodb_table.hostname_mapping.name
}

output "hostname_mapping_table_arn" {
  description = "ARN of the hostname mapping table"
  value       = aws_dynamodb_table.hostname_mapping.arn
}

output "user_settings_table_name" {
  description = "Name of the user settings table"
  value       = aws_dynamodb_table.user_settings.name
}

output "user_settings_table_arn" {
  description = "ARN of the user settings table"
  value       = aws_dynamodb_table.user_settings.arn
}

output "domains_table_name" {
  description = "Name of the domains table"
  value       = aws_dynamodb_table.domains.name
}

output "domains_table_arn" {
  description = "ARN of the domains table"
  value       = aws_dynamodb_table.domains.arn
}

output "routes_to_verify_table_name" {
  description = "Name of the routes to verify table"
  value       = aws_dynamodb_table.routes_to_verify.name
}

output "routes_to_verify_table_arn" {
  description = "ARN of the routes to verify table"
  value       = aws_dynamodb_table.routes_to_verify.arn
}

output "certificate_orders_table_name" {
  description = "Name of the certificate orders table"
  value       = aws_dynamodb_table.certificate_orders.name
}

output "certificate_orders_table_arn" {
  description = "ARN of the certificate orders table"
  value       = aws_dynamodb_table.certificate_orders.arn
}

output "challenges_table_name" {
  description = "Name of the challenges table"
  value       = aws_dynamodb_table.challenges.name
}

output "challenges_table_arn" {
  description = "ARN of the challenges table"
  value       = aws_dynamodb_table.challenges.arn
}

output "all_table_arns" {
  description = "List of all DynamoDB table ARNs"
  value = [
    aws_dynamodb_table.routes.arn,
    aws_dynamodb_table.routes_encryption.arn,
    aws_dynamodb_table.hostname_mapping.arn,
    aws_dynamodb_table.user_settings.arn,
    aws_dynamodb_table.domains.arn,
    aws_dynamodb_table.routes_to_verify.arn,
    aws_dynamodb_table.certificate_orders.arn,
    aws_dynamodb_table.challenges.arn
  ]
}
