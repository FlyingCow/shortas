# ElastiCache Redis Module for Shortas

# Random auth token
resource "random_password" "redis_auth" {
  length           = 32
  special          = false # ElastiCache auth token has restrictions
}

# Store credentials in Secrets Manager
resource "aws_secretsmanager_secret" "redis" {
  name                    = "shortas/${var.environment}/redis"
  description             = "ElastiCache Redis credentials for Shortas"
  recovery_window_in_days = var.environment == "prod" ? 30 : 0

  tags = {
    Name        = "shortas-${var.environment}-redis-secret"
    Environment = var.environment
  }
}

resource "aws_secretsmanager_secret_version" "redis" {
  secret_id = aws_secretsmanager_secret.redis.id
  secret_string = jsonencode({
    auth_token        = random_password.redis_auth.result
    primary_endpoint  = aws_elasticache_replication_group.main.primary_endpoint_address
    reader_endpoint   = aws_elasticache_replication_group.main.reader_endpoint_address
    port              = var.port
    connection_string = "rediss://:${random_password.redis_auth.result}@${aws_elasticache_replication_group.main.primary_endpoint_address}:${var.port}"
  })

  depends_on = [aws_elasticache_replication_group.main]
}

# Parameter Group
resource "aws_elasticache_parameter_group" "main" {
  name        = "${var.environment}-shortas-redis-params"
  family      = "redis7"
  description = "Redis parameter group for Shortas"

  parameter {
    name  = "maxmemory-policy"
    value = "volatile-lru"
  }

  parameter {
    name  = "notify-keyspace-events"
    value = "Ex" # Enable expired key events
  }

  tags = {
    Name        = "${var.environment}-shortas-redis-params"
    Environment = var.environment
  }
}

# Replication Group
resource "aws_elasticache_replication_group" "main" {
  replication_group_id = "${var.environment}-shortas-redis"
  description          = "Redis replication group for Shortas URL shortener"

  engine               = "redis"
  engine_version       = var.engine_version
  node_type            = var.node_type
  port                 = var.port
  parameter_group_name = aws_elasticache_parameter_group.main.name

  # Cluster mode disabled - simple replication
  num_cache_clusters = var.num_cache_clusters

  # Security
  subnet_group_name          = var.subnet_group_name
  security_group_ids         = [var.security_group_id]
  at_rest_encryption_enabled = true
  transit_encryption_enabled = true
  auth_token                 = random_password.redis_auth.result

  # High availability
  automatic_failover_enabled = var.num_cache_clusters > 1
  multi_az_enabled           = var.num_cache_clusters > 1 && var.multi_az_enabled

  # Maintenance
  maintenance_window       = "sun:05:00-sun:06:00"
  snapshot_window          = "03:00-04:00"
  snapshot_retention_limit = var.snapshot_retention_limit
  auto_minor_version_upgrade = true

  # Apply changes immediately in dev, during maintenance window in prod
  apply_immediately = var.environment != "prod"

  # Notifications (optional)
  notification_topic_arn = var.sns_topic_arn

  tags = {
    Name        = "${var.environment}-shortas-redis"
    Environment = var.environment
  }

  lifecycle {
    ignore_changes = [num_cache_clusters]
  }
}

# CloudWatch Alarms
resource "aws_cloudwatch_metric_alarm" "cpu" {
  count               = var.enable_cloudwatch_alarms ? 1 : 0
  alarm_name          = "${var.environment}-shortas-redis-cpu"
  comparison_operator = "GreaterThanThreshold"
  evaluation_periods  = 2
  metric_name         = "CPUUtilization"
  namespace           = "AWS/ElastiCache"
  period              = 300
  statistic           = "Average"
  threshold           = 75
  alarm_description   = "Redis cluster CPU utilization is high"

  dimensions = {
    CacheClusterId = aws_elasticache_replication_group.main.id
  }

  alarm_actions = var.alarm_actions
  ok_actions    = var.alarm_actions

  tags = {
    Name        = "${var.environment}-shortas-redis-cpu-alarm"
    Environment = var.environment
  }
}

resource "aws_cloudwatch_metric_alarm" "memory" {
  count               = var.enable_cloudwatch_alarms ? 1 : 0
  alarm_name          = "${var.environment}-shortas-redis-memory"
  comparison_operator = "GreaterThanThreshold"
  evaluation_periods  = 2
  metric_name         = "DatabaseMemoryUsagePercentage"
  namespace           = "AWS/ElastiCache"
  period              = 300
  statistic           = "Average"
  threshold           = 80
  alarm_description   = "Redis cluster memory usage is high"

  dimensions = {
    CacheClusterId = aws_elasticache_replication_group.main.id
  }

  alarm_actions = var.alarm_actions
  ok_actions    = var.alarm_actions

  tags = {
    Name        = "${var.environment}-shortas-redis-memory-alarm"
    Environment = var.environment
  }
}
