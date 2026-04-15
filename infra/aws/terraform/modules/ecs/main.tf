# ECS Module for Shortas URL Shortener

# ECS Cluster
resource "aws_ecs_cluster" "main" {
  name = "${var.environment}-shortas-cluster"

  setting {
    name  = "containerInsights"
    value = var.enable_container_insights ? "enabled" : "disabled"
  }

  configuration {
    execute_command_configuration {
      logging = "OVERRIDE"

      log_configuration {
        cloud_watch_log_group_name = aws_cloudwatch_log_group.ecs_exec.name
      }
    }
  }

  tags = {
    Name        = "${var.environment}-shortas-cluster"
    Environment = var.environment
  }
}

# ECS Cluster Capacity Providers
resource "aws_ecs_cluster_capacity_providers" "main" {
  cluster_name = aws_ecs_cluster.main.name

  capacity_providers = ["FARGATE", "FARGATE_SPOT"]

  default_capacity_provider_strategy {
    base              = 1
    weight            = 100
    capacity_provider = var.use_spot_instances ? "FARGATE_SPOT" : "FARGATE"
  }
}

# CloudWatch Log Group for ECS Exec
resource "aws_cloudwatch_log_group" "ecs_exec" {
  name              = "/ecs/${var.environment}-shortas/exec"
  retention_in_days = var.log_retention_days

  tags = {
    Name        = "${var.environment}-shortas-ecs-exec-logs"
    Environment = var.environment
  }
}

# CloudWatch Log Groups for Services
resource "aws_cloudwatch_log_group" "services" {
  for_each          = toset(var.service_names)
  name              = "/ecs/${var.environment}-shortas/${each.value}"
  retention_in_days = var.log_retention_days

  tags = {
    Name        = "${var.environment}-shortas-${each.value}-logs"
    Environment = var.environment
    Service     = each.value
  }
}

# EFS File System for Fluvio persistence
resource "aws_efs_file_system" "fluvio" {
  count          = var.enable_fluvio ? 1 : 0
  creation_token = "${var.environment}-shortas-fluvio-efs"
  encrypted      = true

  performance_mode                = "generalPurpose"
  throughput_mode                 = "bursting"

  lifecycle_policy {
    transition_to_ia = "AFTER_30_DAYS"
  }

  tags = {
    Name        = "${var.environment}-shortas-fluvio-efs"
    Environment = var.environment
  }
}

resource "aws_efs_mount_target" "fluvio" {
  count           = var.enable_fluvio ? length(var.private_subnet_ids) : 0
  file_system_id  = aws_efs_file_system.fluvio[0].id
  subnet_id       = var.private_subnet_ids[count.index]
  security_groups = [var.fluvio_security_group_id]
}

resource "aws_efs_access_point" "fluvio_sc" {
  count          = var.enable_fluvio ? 1 : 0
  file_system_id = aws_efs_file_system.fluvio[0].id

  posix_user {
    gid = 1000
    uid = 1000
  }

  root_directory {
    path = "/fluvio-sc"
    creation_info {
      owner_gid   = 1000
      owner_uid   = 1000
      permissions = "755"
    }
  }

  tags = {
    Name        = "${var.environment}-shortas-fluvio-sc-ap"
    Environment = var.environment
  }
}

resource "aws_efs_access_point" "fluvio_spu" {
  count          = var.enable_fluvio ? 1 : 0
  file_system_id = aws_efs_file_system.fluvio[0].id

  posix_user {
    gid = 1000
    uid = 1000
  }

  root_directory {
    path = "/fluvio-spu"
    creation_info {
      owner_gid   = 1000
      owner_uid   = 1000
      permissions = "755"
    }
  }

  tags = {
    Name        = "${var.environment}-shortas-fluvio-spu-ap"
    Environment = var.environment
  }
}

# Service Discovery Namespace
resource "aws_service_discovery_private_dns_namespace" "main" {
  name        = "${var.environment}.shortas.local"
  description = "Service discovery namespace for Shortas services"
  vpc         = var.vpc_id

  tags = {
    Name        = "${var.environment}-shortas-service-discovery"
    Environment = var.environment
  }
}

# Service Discovery Services
resource "aws_service_discovery_service" "services" {
  for_each = toset(var.service_names)
  name     = each.value

  dns_config {
    namespace_id = aws_service_discovery_private_dns_namespace.main.id

    dns_records {
      ttl  = 10
      type = "A"
    }

    routing_policy = "MULTIVALUE"
  }

  health_check_custom_config {
    failure_threshold = 1
  }

  tags = {
    Name        = "${var.environment}-shortas-${each.value}-discovery"
    Environment = var.environment
    Service     = each.value
  }
}
