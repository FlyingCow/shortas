output "endpoint" {
  description = "Domain-specific endpoint (hostname only, no scheme)"
  value       = aws_opensearch_domain.main.endpoint
}

output "url" {
  description = "Full HTTPS URL for the domain, ready for APP__ELASTICSEARCH__URL"
  value       = "https://${aws_opensearch_domain.main.endpoint}"
}

output "domain_name" {
  description = "OpenSearch domain name"
  value       = aws_opensearch_domain.main.domain_name
}

output "domain_arn" {
  description = "OpenSearch domain ARN"
  value       = aws_opensearch_domain.main.arn
}
