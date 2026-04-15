# CloudWatch Module for Shortas URL Shortener

# SNS Topic for Alarms
resource "aws_sns_topic" "alarms" {
  name = "${var.environment}-shortas-alarms"

  tags = {
    Name        = "${var.environment}-shortas-alarms"
    Environment = var.environment
  }
}

# Email subscription (optional)
resource "aws_sns_topic_subscription" "email" {
  for_each  = toset(var.alarm_email_endpoints)
  topic_arn = aws_sns_topic.alarms.arn
  protocol  = "email"
  endpoint  = each.value
}

# Dashboard
resource "aws_cloudwatch_dashboard" "main" {
  dashboard_name = "${var.environment}-shortas-dashboard"

  dashboard_body = jsonencode({
    widgets = [
      {
        type   = "metric"
        x      = 0
        y      = 0
        width  = 12
        height = 6
        properties = {
          title  = "ECS Service CPU Utilization"
          region = var.region
          metrics = [
            for service in var.ecs_service_names : [
              "AWS/ECS",
              "CPUUtilization",
              "ClusterName", var.ecs_cluster_name,
              "ServiceName", "${var.environment}-${service}"
            ]
          ]
          period = 300
          stat   = "Average"
        }
      },
      {
        type   = "metric"
        x      = 12
        y      = 0
        width  = 12
        height = 6
        properties = {
          title  = "ECS Service Memory Utilization"
          region = var.region
          metrics = [
            for service in var.ecs_service_names : [
              "AWS/ECS",
              "MemoryUtilization",
              "ClusterName", var.ecs_cluster_name,
              "ServiceName", "${var.environment}-${service}"
            ]
          ]
          period = 300
          stat   = "Average"
        }
      },
      {
        type   = "metric"
        x      = 0
        y      = 6
        width  = 12
        height = 6
        properties = {
          title  = "ALB Request Count"
          region = var.region
          metrics = [
            ["AWS/ApplicationELB", "RequestCount", "LoadBalancer", var.alb_arn_suffix]
          ]
          period = 60
          stat   = "Sum"
        }
      },
      {
        type   = "metric"
        x      = 12
        y      = 6
        width  = 12
        height = 6
        properties = {
          title  = "ALB Target Response Time"
          region = var.region
          metrics = [
            ["AWS/ApplicationELB", "TargetResponseTime", "LoadBalancer", var.alb_arn_suffix]
          ]
          period = 60
          stat   = "Average"
        }
      },
      {
        type   = "metric"
        x      = 0
        y      = 12
        width  = 8
        height = 6
        properties = {
          title  = "DynamoDB Read/Write Capacity"
          region = var.region
          metrics = [
            ["AWS/DynamoDB", "ConsumedReadCapacityUnits", "TableName", "core-routes-${var.environment}"],
            ["AWS/DynamoDB", "ConsumedWriteCapacityUnits", "TableName", "core-routes-${var.environment}"]
          ]
          period = 300
          stat   = "Sum"
        }
      },
      {
        type   = "metric"
        x      = 8
        y      = 12
        width  = 8
        height = 6
        properties = {
          title  = "ElastiCache CPU & Memory"
          region = var.region
          metrics = [
            ["AWS/ElastiCache", "CPUUtilization", "CacheClusterId", "${var.environment}-shortas-redis-001"],
            ["AWS/ElastiCache", "DatabaseMemoryUsagePercentage", "CacheClusterId", "${var.environment}-shortas-redis-001"]
          ]
          period = 300
          stat   = "Average"
        }
      },
      {
        type   = "metric"
        x      = 16
        y      = 12
        width  = 8
        height = 6
        properties = {
          title  = "RDS CPU & Connections"
          region = var.region
          metrics = [
            ["AWS/RDS", "CPUUtilization", "DBClusterIdentifier", "${var.environment}-shortas-aurora-cluster"],
            ["AWS/RDS", "DatabaseConnections", "DBClusterIdentifier", "${var.environment}-shortas-aurora-cluster"]
          ]
          period = 300
          stat   = "Average"
        }
      }
    ]
  })
}

# ALB 5xx Error Alarm
resource "aws_cloudwatch_metric_alarm" "alb_5xx" {
  alarm_name          = "${var.environment}-shortas-alb-5xx-errors"
  comparison_operator = "GreaterThanThreshold"
  evaluation_periods  = 2
  metric_name         = "HTTPCode_ELB_5XX_Count"
  namespace           = "AWS/ApplicationELB"
  period              = 300
  statistic           = "Sum"
  threshold           = var.alb_5xx_threshold
  alarm_description   = "ALB 5xx errors exceeded threshold"
  treat_missing_data  = "notBreaching"

  dimensions = {
    LoadBalancer = var.alb_arn_suffix
  }

  alarm_actions = [aws_sns_topic.alarms.arn]
  ok_actions    = [aws_sns_topic.alarms.arn]

  tags = {
    Name        = "${var.environment}-shortas-alb-5xx-alarm"
    Environment = var.environment
  }
}

# ALB Target 5xx Error Alarm
resource "aws_cloudwatch_metric_alarm" "target_5xx" {
  alarm_name          = "${var.environment}-shortas-target-5xx-errors"
  comparison_operator = "GreaterThanThreshold"
  evaluation_periods  = 2
  metric_name         = "HTTPCode_Target_5XX_Count"
  namespace           = "AWS/ApplicationELB"
  period              = 300
  statistic           = "Sum"
  threshold           = var.target_5xx_threshold
  alarm_description   = "Target 5xx errors exceeded threshold"
  treat_missing_data  = "notBreaching"

  dimensions = {
    LoadBalancer = var.alb_arn_suffix
  }

  alarm_actions = [aws_sns_topic.alarms.arn]
  ok_actions    = [aws_sns_topic.alarms.arn]

  tags = {
    Name        = "${var.environment}-shortas-target-5xx-alarm"
    Environment = var.environment
  }
}

# ALB Response Time Alarm
resource "aws_cloudwatch_metric_alarm" "response_time" {
  alarm_name          = "${var.environment}-shortas-response-time"
  comparison_operator = "GreaterThanThreshold"
  evaluation_periods  = 3
  metric_name         = "TargetResponseTime"
  namespace           = "AWS/ApplicationELB"
  period              = 300
  statistic           = "Average"
  threshold           = var.response_time_threshold
  alarm_description   = "Response time exceeded threshold"
  treat_missing_data  = "notBreaching"

  dimensions = {
    LoadBalancer = var.alb_arn_suffix
  }

  alarm_actions = [aws_sns_topic.alarms.arn]

  tags = {
    Name        = "${var.environment}-shortas-response-time-alarm"
    Environment = var.environment
  }
}

# Log Metric Filter for Error Logs
resource "aws_cloudwatch_log_metric_filter" "errors" {
  for_each       = toset(var.ecs_service_names)
  name           = "${var.environment}-${each.value}-errors"
  pattern        = "[timestamp, request_id, level=ERROR, ...]"
  log_group_name = "/ecs/${var.environment}-shortas/${each.value}"

  metric_transformation {
    name      = "${each.value}-error-count"
    namespace = "Shortas/${var.environment}"
    value     = "1"
  }
}

# Composite Alarm for Service Health
resource "aws_cloudwatch_composite_alarm" "service_health" {
  count             = var.create_composite_alarm ? 1 : 0
  alarm_name        = "${var.environment}-shortas-service-health"
  alarm_description = "Composite alarm for overall service health"

  alarm_rule = "ALARM(${aws_cloudwatch_metric_alarm.alb_5xx.alarm_name}) OR ALARM(${aws_cloudwatch_metric_alarm.target_5xx.alarm_name})"

  alarm_actions = [aws_sns_topic.alarms.arn]
  ok_actions    = [aws_sns_topic.alarms.arn]

  tags = {
    Name        = "${var.environment}-shortas-service-health-alarm"
    Environment = var.environment
  }
}
