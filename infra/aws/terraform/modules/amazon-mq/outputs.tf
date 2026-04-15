output "broker_id" {
  description = "The broker ID"
  value       = aws_mq_broker.main.id
}

output "broker_arn" {
  description = "The broker ARN"
  value       = aws_mq_broker.main.arn
}

output "amqp_endpoints" {
  description = "AMQP endpoints"
  value       = aws_mq_broker.main.instances[0].endpoints
}

output "console_url" {
  description = "RabbitMQ console URL"
  value       = aws_mq_broker.main.instances[0].console_url
}

output "primary_amqp_endpoint" {
  description = "Primary AMQP endpoint"
  value       = aws_mq_broker.main.instances[0].endpoints[0]
}

output "amqps_endpoint" {
  description = "AMQPS (TLS) endpoint"
  value       = try(aws_mq_broker.main.instances[0].endpoints[1], "")
}

output "secret_arn" {
  description = "ARN of the Secrets Manager secret containing credentials"
  value       = aws_secretsmanager_secret.rabbitmq.arn
}

output "secret_name" {
  description = "Name of the Secrets Manager secret"
  value       = aws_secretsmanager_secret.rabbitmq.name
}

output "admin_username" {
  description = "Admin username"
  value       = var.admin_username
}
