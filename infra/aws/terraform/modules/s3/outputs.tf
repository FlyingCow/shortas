output "route_images_bucket_name" {
  description = "Name of the route images bucket"
  value       = aws_s3_bucket.route_images.bucket
}

output "route_images_bucket_arn" {
  description = "ARN of the route images bucket"
  value       = aws_s3_bucket.route_images.arn
}

output "route_images_bucket_domain_name" {
  description = "Domain name of the route images bucket"
  value       = aws_s3_bucket.route_images.bucket_regional_domain_name
}

output "route_images_bucket_url" {
  description = "URL for accessing route images"
  value       = "https://${aws_s3_bucket.route_images.bucket_regional_domain_name}"
}

output "clickhouse_backups_bucket_name" {
  description = "Name of the ClickHouse backups bucket"
  value       = aws_s3_bucket.clickhouse_backups.bucket
}

output "clickhouse_backups_bucket_arn" {
  description = "ARN of the ClickHouse backups bucket"
  value       = aws_s3_bucket.clickhouse_backups.arn
}

output "terraform_state_bucket_name" {
  description = "Name of the Terraform state bucket"
  value       = var.create_terraform_state_bucket ? aws_s3_bucket.terraform_state[0].bucket : null
}

output "terraform_state_bucket_arn" {
  description = "ARN of the Terraform state bucket"
  value       = var.create_terraform_state_bucket ? aws_s3_bucket.terraform_state[0].arn : null
}

output "terraform_locks_table_name" {
  description = "Name of the Terraform locks DynamoDB table"
  value       = var.create_terraform_state_bucket ? aws_dynamodb_table.terraform_locks[0].name : null
}

output "logs_bucket_name" {
  description = "Name of the logs bucket"
  value       = var.create_logs_bucket ? aws_s3_bucket.logs[0].bucket : null
}

output "logs_bucket_arn" {
  description = "ARN of the logs bucket"
  value       = var.create_logs_bucket ? aws_s3_bucket.logs[0].arn : null
}
