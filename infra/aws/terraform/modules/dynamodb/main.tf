# DynamoDB Module for Shortas URL Shortener

# Routes Table - Main table for URL mappings
resource "aws_dynamodb_table" "routes" {
  name         = "core-routes-${var.environment}"
  billing_mode = var.billing_mode
  hash_key     = "link"
  range_key    = "switch"

  # Provisioned capacity (only used if billing_mode is PROVISIONED)
  read_capacity  = var.billing_mode == "PROVISIONED" ? var.routes_read_capacity : null
  write_capacity = var.billing_mode == "PROVISIONED" ? var.routes_write_capacity : null

  attribute {
    name = "link"
    type = "S"
  }

  attribute {
    name = "switch"
    type = "S"
  }

  # Global Secondary Index for owner lookups
  global_secondary_index {
    name            = "owner-index"
    hash_key        = "switch"
    projection_type = "ALL"
    read_capacity   = var.billing_mode == "PROVISIONED" ? var.routes_read_capacity : null
    write_capacity  = var.billing_mode == "PROVISIONED" ? var.routes_write_capacity : null
  }

  point_in_time_recovery {
    enabled = var.enable_point_in_time_recovery
  }

  server_side_encryption {
    enabled = true
  }

  ttl {
    attribute_name = "TimeToExist"
    enabled        = var.enable_ttl
  }

  tags = {
    Name        = "core-routes-${var.environment}"
    Environment = var.environment
  }
}

# Routes Encryption Table - Stores encryption keys per hostname
resource "aws_dynamodb_table" "routes_encryption" {
  name         = "core-routes-encryption-${var.environment}"
  billing_mode = var.billing_mode
  hash_key     = "hostname"

  read_capacity  = var.billing_mode == "PROVISIONED" ? var.encryption_read_capacity : null
  write_capacity = var.billing_mode == "PROVISIONED" ? var.encryption_write_capacity : null

  attribute {
    name = "hostname"
    type = "S"
  }

  point_in_time_recovery {
    enabled = var.enable_point_in_time_recovery
  }

  server_side_encryption {
    enabled = true
  }

  ttl {
    attribute_name = "TimeToExist"
    enabled        = var.enable_ttl
  }

  tags = {
    Name        = "core-routes-encryption-${var.environment}"
    Environment = var.environment
  }
}

# Hostname Mapping Table - Maps custom domains to accounts
resource "aws_dynamodb_table" "hostname_mapping" {
  name         = "core-routes-hostname-mapping-${var.environment}"
  billing_mode = var.billing_mode
  hash_key     = "hostname"

  read_capacity  = var.billing_mode == "PROVISIONED" ? var.hostname_read_capacity : null
  write_capacity = var.billing_mode == "PROVISIONED" ? var.hostname_write_capacity : null

  attribute {
    name = "hostname"
    type = "S"
  }

  point_in_time_recovery {
    enabled = var.enable_point_in_time_recovery
  }

  server_side_encryption {
    enabled = true
  }

  ttl {
    attribute_name = "TimeToExist"
    enabled        = var.enable_ttl
  }

  tags = {
    Name        = "core-routes-hostname-mapping-${var.environment}"
    Environment = var.environment
  }
}

# User Settings Table - Stores per-user configuration
resource "aws_dynamodb_table" "user_settings" {
  name         = "core-user-settings-${var.environment}"
  billing_mode = var.billing_mode
  hash_key     = "user_id"

  read_capacity  = var.billing_mode == "PROVISIONED" ? var.user_settings_read_capacity : null
  write_capacity = var.billing_mode == "PROVISIONED" ? var.user_settings_write_capacity : null

  attribute {
    name = "user_id"
    type = "S"
  }

  point_in_time_recovery {
    enabled = var.enable_point_in_time_recovery
  }

  server_side_encryption {
    enabled = true
  }

  ttl {
    attribute_name = "TimeToExist"
    enabled        = var.enable_ttl
  }

  tags = {
    Name        = "core-user-settings-${var.environment}"
    Environment = var.environment
  }
}

# Auto Scaling for Routes Table (only if PROVISIONED billing mode)
resource "aws_appautoscaling_target" "routes_read" {
  count              = var.billing_mode == "PROVISIONED" && var.enable_autoscaling ? 1 : 0
  max_capacity       = var.routes_read_max_capacity
  min_capacity       = var.routes_read_capacity
  resource_id        = "table/${aws_dynamodb_table.routes.name}"
  scalable_dimension = "dynamodb:table:ReadCapacityUnits"
  service_namespace  = "dynamodb"
}

resource "aws_appautoscaling_policy" "routes_read" {
  count              = var.billing_mode == "PROVISIONED" && var.enable_autoscaling ? 1 : 0
  name               = "${var.environment}-routes-read-autoscaling"
  policy_type        = "TargetTrackingScaling"
  resource_id        = aws_appautoscaling_target.routes_read[0].resource_id
  scalable_dimension = aws_appautoscaling_target.routes_read[0].scalable_dimension
  service_namespace  = aws_appautoscaling_target.routes_read[0].service_namespace

  target_tracking_scaling_policy_configuration {
    predefined_metric_specification {
      predefined_metric_type = "DynamoDBReadCapacityUtilization"
    }
    target_value = 70.0
  }
}

resource "aws_appautoscaling_target" "routes_write" {
  count              = var.billing_mode == "PROVISIONED" && var.enable_autoscaling ? 1 : 0
  max_capacity       = var.routes_write_max_capacity
  min_capacity       = var.routes_write_capacity
  resource_id        = "table/${aws_dynamodb_table.routes.name}"
  scalable_dimension = "dynamodb:table:WriteCapacityUnits"
  service_namespace  = "dynamodb"
}

resource "aws_appautoscaling_policy" "routes_write" {
  count              = var.billing_mode == "PROVISIONED" && var.enable_autoscaling ? 1 : 0
  name               = "${var.environment}-routes-write-autoscaling"
  policy_type        = "TargetTrackingScaling"
  resource_id        = aws_appautoscaling_target.routes_write[0].resource_id
  scalable_dimension = aws_appautoscaling_target.routes_write[0].scalable_dimension
  service_namespace  = aws_appautoscaling_target.routes_write[0].service_namespace

  target_tracking_scaling_policy_configuration {
    predefined_metric_specification {
      predefined_metric_type = "DynamoDBWriteCapacityUtilization"
    }
    target_value = 70.0
  }
}

# Domains Table - Domain verification tracking
resource "aws_dynamodb_table" "domains" {
  name         = "core-domains-${var.environment}"
  billing_mode = var.billing_mode
  hash_key     = "id"

  read_capacity  = var.billing_mode == "PROVISIONED" ? var.domains_read_capacity : null
  write_capacity = var.billing_mode == "PROVISIONED" ? var.domains_write_capacity : null

  attribute {
    name = "id"
    type = "S"
  }

  attribute {
    name = "owner_name"
    type = "S"
  }

  attribute {
    name = "owner_id"
    type = "S"
  }

  # GSI for looking up domains by owner_id + name
  global_secondary_index {
    name            = "owner_name-index"
    hash_key        = "owner_name"
    projection_type = "ALL"
    read_capacity   = var.billing_mode == "PROVISIONED" ? var.domains_read_capacity : null
    write_capacity  = var.billing_mode == "PROVISIONED" ? var.domains_write_capacity : null
  }

  # GSI for listing domains by owner
  global_secondary_index {
    name            = "owner-index"
    hash_key        = "owner_id"
    projection_type = "ALL"
    read_capacity   = var.billing_mode == "PROVISIONED" ? var.domains_read_capacity : null
    write_capacity  = var.billing_mode == "PROVISIONED" ? var.domains_write_capacity : null
  }

  point_in_time_recovery {
    enabled = var.enable_point_in_time_recovery
  }

  server_side_encryption {
    enabled = true
  }

  tags = {
    Name        = "core-domains-${var.environment}"
    Environment = var.environment
  }
}

# Routes to Verify Table - Safety verification queue
resource "aws_dynamodb_table" "routes_to_verify" {
  name         = "core-routes-to-verify-${var.environment}"
  billing_mode = var.billing_mode
  hash_key     = "id"

  read_capacity  = var.billing_mode == "PROVISIONED" ? var.routes_verify_read_capacity : null
  write_capacity = var.billing_mode == "PROVISIONED" ? var.routes_verify_write_capacity : null

  attribute {
    name = "id"
    type = "S"
  }

  attribute {
    name = "owner_id"
    type = "S"
  }

  # GSI for listing routes by owner
  global_secondary_index {
    name            = "owner-index"
    hash_key        = "owner_id"
    projection_type = "ALL"
    read_capacity   = var.billing_mode == "PROVISIONED" ? var.routes_verify_read_capacity : null
    write_capacity  = var.billing_mode == "PROVISIONED" ? var.routes_verify_write_capacity : null
  }

  point_in_time_recovery {
    enabled = var.enable_point_in_time_recovery
  }

  server_side_encryption {
    enabled = true
  }

  tags = {
    Name        = "core-routes-to-verify-${var.environment}"
    Environment = var.environment
  }
}

# Certificate Orders Table - ACME certificate order tracking
resource "aws_dynamodb_table" "certificate_orders" {
  name         = "core-certificate-orders-${var.environment}"
  billing_mode = var.billing_mode
  hash_key     = "order_id"

  read_capacity  = var.billing_mode == "PROVISIONED" ? var.orders_read_capacity : null
  write_capacity = var.billing_mode == "PROVISIONED" ? var.orders_write_capacity : null

  attribute {
    name = "order_id"
    type = "S"
  }

  attribute {
    name = "status_key"
    type = "S"
  }

  attribute {
    name = "domain_active"
    type = "S"
  }

  # GSI for querying orders by status
  global_secondary_index {
    name            = "status-index"
    hash_key        = "status_key"
    projection_type = "ALL"
    read_capacity   = var.billing_mode == "PROVISIONED" ? var.orders_read_capacity : null
    write_capacity  = var.billing_mode == "PROVISIONED" ? var.orders_write_capacity : null
  }

  # GSI for finding active orders by domain
  global_secondary_index {
    name            = "domain-active-index"
    hash_key        = "domain_active"
    projection_type = "ALL"
    read_capacity   = var.billing_mode == "PROVISIONED" ? var.orders_read_capacity : null
    write_capacity  = var.billing_mode == "PROVISIONED" ? var.orders_write_capacity : null
  }

  point_in_time_recovery {
    enabled = var.enable_point_in_time_recovery
  }

  server_side_encryption {
    enabled = true
  }

  ttl {
    attribute_name = "expires_at_ttl"
    enabled        = true
  }

  tags = {
    Name        = "core-certificate-orders-${var.environment}"
    Environment = var.environment
  }
}

# ACME Challenges Table - HTTP-01 challenge responses
resource "aws_dynamodb_table" "challenges" {
  name         = "core-challenges-${var.environment}"
  billing_mode = var.billing_mode
  hash_key     = "domain"
  range_key    = "token"

  read_capacity  = var.billing_mode == "PROVISIONED" ? var.challenges_read_capacity : null
  write_capacity = var.billing_mode == "PROVISIONED" ? var.challenges_write_capacity : null

  attribute {
    name = "domain"
    type = "S"
  }

  attribute {
    name = "token"
    type = "S"
  }

  point_in_time_recovery {
    enabled = false # Challenges are short-lived
  }

  server_side_encryption {
    enabled = true
  }

  ttl {
    attribute_name = "expires_at"
    enabled        = true
  }

  tags = {
    Name        = "core-challenges-${var.environment}"
    Environment = var.environment
  }
}

# CloudWatch Alarms
resource "aws_cloudwatch_metric_alarm" "routes_throttled_requests" {
  count               = var.enable_cloudwatch_alarms ? 1 : 0
  alarm_name          = "${var.environment}-shortas-dynamodb-routes-throttled"
  comparison_operator = "GreaterThanThreshold"
  evaluation_periods  = 2
  metric_name         = "ThrottledRequests"
  namespace           = "AWS/DynamoDB"
  period              = 60
  statistic           = "Sum"
  threshold           = 0
  alarm_description   = "DynamoDB routes table is being throttled"

  dimensions = {
    TableName = aws_dynamodb_table.routes.name
  }

  alarm_actions = var.alarm_actions

  tags = {
    Name        = "${var.environment}-shortas-dynamodb-routes-throttled"
    Environment = var.environment
  }
}
