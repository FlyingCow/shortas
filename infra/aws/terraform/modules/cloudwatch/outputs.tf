output "sns_topic_arn" {
  description = "ARN of the SNS topic for alarms"
  value       = aws_sns_topic.alarms.arn
}

output "dashboard_arn" {
  description = "ARN of the CloudWatch dashboard"
  value       = aws_cloudwatch_dashboard.main.dashboard_arn
}

output "alb_5xx_alarm_arn" {
  description = "ARN of the ALB 5xx alarm"
  value       = aws_cloudwatch_metric_alarm.alb_5xx.arn
}

output "target_5xx_alarm_arn" {
  description = "ARN of the target 5xx alarm"
  value       = aws_cloudwatch_metric_alarm.target_5xx.arn
}
