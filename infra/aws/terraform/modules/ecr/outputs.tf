output "repository_urls" {
  description = "Map of service names to repository URLs"
  value       = { for k, v in aws_ecr_repository.services : k => v.repository_url }
}

output "repository_arns" {
  description = "Map of service names to repository ARNs"
  value       = { for k, v in aws_ecr_repository.services : k => v.arn }
}

output "registry_id" {
  description = "Registry ID (AWS account ID)"
  value       = values(aws_ecr_repository.services)[0].registry_id
}

output "base_repository_url" {
  description = "Base URL for ECR repositories (without service name)"
  value       = split("/", values(aws_ecr_repository.services)[0].repository_url)[0]
}
