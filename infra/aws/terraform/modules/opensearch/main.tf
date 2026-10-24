# OpenSearch Module for Shortas (route search index)
#
# VPC-only domain, access controlled purely by security group membership
# (same trust model as RDS/ElastiCache/Amazon MQ in this stack) - no
# fine-grained access control, no master user, no SigV4 signing required.
# management-api's Elasticsearch client talks to it over plain HTTPS.

resource "aws_opensearch_domain" "main" {
  domain_name    = "${var.environment}-shortas-search"
  engine_version = var.engine_version

  cluster_config {
    instance_type          = var.instance_type
    instance_count         = var.instance_count
    zone_awareness_enabled = var.zone_awareness_enabled

    dynamic "zone_awareness_config" {
      for_each = var.zone_awareness_enabled ? [1] : []
      content {
        availability_zone_count = length(var.subnet_ids)
      }
    }
  }

  vpc_options {
    subnet_ids         = var.zone_awareness_enabled ? var.subnet_ids : [var.subnet_ids[0]]
    security_group_ids = [var.security_group_id]
  }

  ebs_options {
    ebs_enabled = true
    volume_type = "gp3"
    volume_size = var.ebs_volume_size
  }

  encrypt_at_rest {
    enabled = true
  }

  node_to_node_encryption {
    enabled = true
  }

  domain_endpoint_options {
    enforce_https       = true
    tls_security_policy = "Policy-Min-TLS-1-2-2019-07"
  }

  # No fine-grained access control: the VPC + security group already
  # restrict access to ECS tasks on this network, matching every other
  # datastore in this stack.
  access_policies = jsonencode({
    Version = "2012-10-17"
    Statement = [
      {
        Effect    = "Allow"
        Principal = { AWS = "*" }
        Action    = "es:*"
        Resource  = "arn:aws:es:*:*:domain/${var.environment}-shortas-search/*"
      }
    ]
  })

  tags = {
    Name        = "${var.environment}-shortas-search"
    Environment = var.environment
  }
}

resource "aws_cloudwatch_metric_alarm" "cluster_status_red" {
  count               = var.enable_cloudwatch_alarms ? 1 : 0
  alarm_name          = "${var.environment}-shortas-opensearch-status-red"
  comparison_operator = "GreaterThanThreshold"
  evaluation_periods  = 1
  metric_name         = "ClusterStatus.red"
  namespace           = "AWS/ES"
  period              = 60
  statistic           = "Maximum"
  threshold           = 0
  alarm_description   = "OpenSearch cluster status is red"

  dimensions = {
    DomainName = aws_opensearch_domain.main.domain_name
    ClientId   = data.aws_caller_identity.current.account_id
  }

  alarm_actions = var.alarm_actions
  ok_actions    = var.alarm_actions

  tags = {
    Name        = "${var.environment}-shortas-opensearch-status-alarm"
    Environment = var.environment
  }
}

resource "aws_cloudwatch_metric_alarm" "free_storage_low" {
  count               = var.enable_cloudwatch_alarms ? 1 : 0
  alarm_name          = "${var.environment}-shortas-opensearch-storage-low"
  comparison_operator = "LessThanThreshold"
  evaluation_periods  = 2
  metric_name         = "FreeStorageSpace"
  namespace           = "AWS/ES"
  period              = 300
  statistic           = "Minimum"
  threshold           = var.ebs_volume_size * 1024 * 0.15 # alarm below 15% free
  alarm_description   = "OpenSearch free storage space is low"

  dimensions = {
    DomainName = aws_opensearch_domain.main.domain_name
    ClientId   = data.aws_caller_identity.current.account_id
  }

  alarm_actions = var.alarm_actions
  ok_actions    = var.alarm_actions

  tags = {
    Name        = "${var.environment}-shortas-opensearch-storage-alarm"
    Environment = var.environment
  }
}

data "aws_caller_identity" "current" {}
