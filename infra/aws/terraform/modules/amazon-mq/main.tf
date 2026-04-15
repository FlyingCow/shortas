# Amazon MQ RabbitMQ Module for Shortas

# Random password for RabbitMQ admin
resource "random_password" "rabbitmq" {
  length           = 32
  special          = true
  override_special = "!#$%&*()-_=+[]{}?"
}

# Store credentials in Secrets Manager
resource "aws_secretsmanager_secret" "rabbitmq" {
  name                    = "shortas/${var.environment}/rabbitmq"
  description             = "Amazon MQ RabbitMQ credentials for Shortas"
  recovery_window_in_days = var.environment == "prod" ? 30 : 0

  tags = {
    Name        = "shortas-${var.environment}-rabbitmq-secret"
    Environment = var.environment
  }
}

resource "aws_secretsmanager_secret_version" "rabbitmq" {
  secret_id = aws_secretsmanager_secret.rabbitmq.id
  secret_string = jsonencode({
    username          = var.admin_username
    password          = random_password.rabbitmq.result
    amqp_endpoint     = aws_mq_broker.main.instances[0].endpoints[0]
    amqps_endpoint    = try(aws_mq_broker.main.instances[0].endpoints[1], "")
    console_url       = aws_mq_broker.main.instances[0].console_url
    broker_id         = aws_mq_broker.main.id
    amqp_uri          = "amqps://${var.admin_username}:${random_password.rabbitmq.result}@${replace(try(aws_mq_broker.main.instances[0].endpoints[1], aws_mq_broker.main.instances[0].endpoints[0]), "amqps://", "")}/%2f"
  })

  depends_on = [aws_mq_broker.main]
}

# CloudWatch Log Group for RabbitMQ
resource "aws_cloudwatch_log_group" "rabbitmq" {
  name              = "/aws/amazonmq/broker/${var.environment}-shortas-rabbitmq"
  retention_in_days = var.log_retention_days

  tags = {
    Name        = "${var.environment}-shortas-rabbitmq-logs"
    Environment = var.environment
  }
}

# RabbitMQ Broker
resource "aws_mq_broker" "main" {
  broker_name        = "${var.environment}-shortas-rabbitmq"
  engine_type        = "RabbitMQ"
  engine_version     = var.engine_version
  host_instance_type = var.host_instance_type
  deployment_mode    = var.deployment_mode

  security_groups = [var.security_group_id]
  subnet_ids      = var.deployment_mode == "CLUSTER_MULTI_AZ" ? var.subnet_ids : [var.subnet_ids[0]]

  publicly_accessible = false
  auto_minor_version_upgrade = true

  user {
    username = var.admin_username
    password = random_password.rabbitmq.result
  }

  maintenance_window_start_time {
    day_of_week = "SUNDAY"
    time_of_day = "04:00"
    time_zone   = "UTC"
  }

  logs {
    general = true
  }

  encryption_options {
    use_aws_owned_key = var.use_aws_owned_key
    kms_key_id        = var.use_aws_owned_key ? null : var.kms_key_id
  }

  tags = {
    Name        = "${var.environment}-shortas-rabbitmq"
    Environment = var.environment
  }
}

# CloudWatch Alarms
resource "aws_cloudwatch_metric_alarm" "message_count" {
  count               = var.enable_cloudwatch_alarms ? 1 : 0
  alarm_name          = "${var.environment}-shortas-rabbitmq-message-count"
  comparison_operator = "GreaterThanThreshold"
  evaluation_periods  = 3
  metric_name         = "MessageCount"
  namespace           = "AWS/AmazonMQ"
  period              = 300
  statistic           = "Average"
  threshold           = var.message_count_threshold
  alarm_description   = "RabbitMQ message count is high - potential consumer issue"

  dimensions = {
    Broker = aws_mq_broker.main.id
  }

  alarm_actions = var.alarm_actions
  ok_actions    = var.alarm_actions

  tags = {
    Name        = "${var.environment}-shortas-rabbitmq-message-count-alarm"
    Environment = var.environment
  }
}

resource "aws_cloudwatch_metric_alarm" "consumer_count" {
  count               = var.enable_cloudwatch_alarms ? 1 : 0
  alarm_name          = "${var.environment}-shortas-rabbitmq-consumer-count"
  comparison_operator = "LessThanThreshold"
  evaluation_periods  = 2
  metric_name         = "ConsumerCount"
  namespace           = "AWS/AmazonMQ"
  period              = 300
  statistic           = "Minimum"
  threshold           = 1
  alarm_description   = "RabbitMQ has no consumers - services may be down"

  dimensions = {
    Broker = aws_mq_broker.main.id
  }

  alarm_actions = var.alarm_actions

  tags = {
    Name        = "${var.environment}-shortas-rabbitmq-consumer-count-alarm"
    Environment = var.environment
  }
}
