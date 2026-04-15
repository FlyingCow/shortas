# ECS Services for Shortas

# Generic service module for standard services
resource "aws_ecs_service" "services" {
  for_each = { for svc in var.services : svc.name => svc }

  name            = "${var.environment}-${each.value.name}"
  cluster         = aws_ecs_cluster.main.id
  task_definition = aws_ecs_task_definition.services[each.key].arn
  desired_count   = each.value.desired_count
  launch_type     = "FARGATE"

  enable_execute_command = var.enable_ecs_exec

  network_configuration {
    subnets          = var.private_subnet_ids
    security_groups  = [var.ecs_security_group_id]
    assign_public_ip = false
  }

  # Load balancer configuration (if service has a target group)
  dynamic "load_balancer" {
    for_each = each.value.target_group_arn != null ? [1] : []
    content {
      target_group_arn = each.value.target_group_arn
      container_name   = each.value.name
      container_port   = each.value.container_port
    }
  }

  # Service discovery
  service_registries {
    registry_arn = aws_service_discovery_service.services[each.value.name].arn
  }

  deployment_configuration {
    maximum_percent         = 200
    minimum_healthy_percent = 100
  }

  deployment_circuit_breaker {
    enable   = true
    rollback = true
  }

  propagate_tags = "SERVICE"

  tags = {
    Name        = "${var.environment}-${each.value.name}"
    Environment = var.environment
    Service     = each.value.name
  }

  lifecycle {
    ignore_changes = [desired_count]
  }
}

# Task Definitions for all services
resource "aws_ecs_task_definition" "services" {
  for_each = { for svc in var.services : svc.name => svc }

  family                   = "${var.environment}-${each.value.name}"
  network_mode             = "awsvpc"
  requires_compatibilities = ["FARGATE"]
  cpu                      = each.value.cpu
  memory                   = each.value.memory
  execution_role_arn       = var.task_execution_role_arn
  task_role_arn            = var.task_role_arn

  container_definitions = jsonencode([
    {
      name      = each.value.name
      image     = "${var.ecr_repository_url}/${each.value.name}:${var.image_tag}"
      essential = true

      portMappings = each.value.container_port != null ? [
        {
          containerPort = each.value.container_port
          protocol      = "tcp"
        }
      ] : []

      environment = concat(
        [
          { name = "APP_ENVIRONMENT", value = var.environment },
          { name = "RUN_MODE", value = var.environment == "prod" ? "production" : "development" },
          { name = "AWS_REGION", value = var.region },
        ],
        each.value.environment_variables
      )

      secrets = concat(
        var.common_secrets,
        each.value.secrets
      )

      logConfiguration = {
        logDriver = "awslogs"
        options = {
          "awslogs-group"         = aws_cloudwatch_log_group.services[each.value.name].name
          "awslogs-region"        = var.region
          "awslogs-stream-prefix" = "ecs"
        }
      }

      healthCheck = each.value.health_check != null ? {
        command     = each.value.health_check.command
        interval    = each.value.health_check.interval
        timeout     = each.value.health_check.timeout
        retries     = each.value.health_check.retries
        startPeriod = each.value.health_check.start_period
      } : null
    }
  ])

  tags = {
    Name        = "${var.environment}-${each.value.name}-task"
    Environment = var.environment
    Service     = each.value.name
  }
}

# Fluvio SC Service
resource "aws_ecs_service" "fluvio_sc" {
  count = var.enable_fluvio ? 1 : 0

  name            = "${var.environment}-fluvio-sc"
  cluster         = aws_ecs_cluster.main.id
  task_definition = aws_ecs_task_definition.fluvio_sc[0].arn
  desired_count   = 1
  launch_type     = "FARGATE"

  platform_version       = "1.4.0" # Required for EFS
  enable_execute_command = var.enable_ecs_exec

  network_configuration {
    subnets          = var.private_subnet_ids
    security_groups  = [var.fluvio_security_group_id]
    assign_public_ip = false
  }

  service_registries {
    registry_arn = aws_service_discovery_service.services["fluvio-sc"].arn
  }

  tags = {
    Name        = "${var.environment}-fluvio-sc"
    Environment = var.environment
    Service     = "fluvio-sc"
  }
}

resource "aws_ecs_task_definition" "fluvio_sc" {
  count = var.enable_fluvio ? 1 : 0

  family                   = "${var.environment}-fluvio-sc"
  network_mode             = "awsvpc"
  requires_compatibilities = ["FARGATE"]
  cpu                      = var.fluvio_sc_cpu
  memory                   = var.fluvio_sc_memory
  execution_role_arn       = var.task_execution_role_arn
  task_role_arn            = var.task_role_arn

  volume {
    name = "fluvio-data"

    efs_volume_configuration {
      file_system_id     = aws_efs_file_system.fluvio[0].id
      transit_encryption = "ENABLED"
      authorization_config {
        access_point_id = aws_efs_access_point.fluvio_sc[0].id
        iam             = "ENABLED"
      }
    }
  }

  container_definitions = jsonencode([
    {
      name      = "fluvio-sc"
      image     = var.fluvio_image
      essential = true

      portMappings = [
        { containerPort = 9003, protocol = "tcp" }
      ]

      environment = [
        { name = "RUST_LOG", value = "info" }
      ]

      mountPoints = [
        {
          sourceVolume  = "fluvio-data"
          containerPath = "/var/lib/fluvio"
          readOnly      = false
        }
      ]

      logConfiguration = {
        logDriver = "awslogs"
        options = {
          "awslogs-group"         = aws_cloudwatch_log_group.services["fluvio-sc"].name
          "awslogs-region"        = var.region
          "awslogs-stream-prefix" = "ecs"
        }
      }
    }
  ])

  tags = {
    Name        = "${var.environment}-fluvio-sc-task"
    Environment = var.environment
    Service     = "fluvio-sc"
  }
}

# Fluvio SPU Service
resource "aws_ecs_service" "fluvio_spu" {
  count = var.enable_fluvio ? 1 : 0

  name            = "${var.environment}-fluvio-spu"
  cluster         = aws_ecs_cluster.main.id
  task_definition = aws_ecs_task_definition.fluvio_spu[0].arn
  desired_count   = var.fluvio_spu_count
  launch_type     = "FARGATE"

  platform_version       = "1.4.0"
  enable_execute_command = var.enable_ecs_exec

  network_configuration {
    subnets          = var.private_subnet_ids
    security_groups  = [var.fluvio_security_group_id]
    assign_public_ip = false
  }

  service_registries {
    registry_arn = aws_service_discovery_service.services["fluvio-spu"].arn
  }

  tags = {
    Name        = "${var.environment}-fluvio-spu"
    Environment = var.environment
    Service     = "fluvio-spu"
  }
}

resource "aws_ecs_task_definition" "fluvio_spu" {
  count = var.enable_fluvio ? 1 : 0

  family                   = "${var.environment}-fluvio-spu"
  network_mode             = "awsvpc"
  requires_compatibilities = ["FARGATE"]
  cpu                      = var.fluvio_spu_cpu
  memory                   = var.fluvio_spu_memory
  execution_role_arn       = var.task_execution_role_arn
  task_role_arn            = var.task_role_arn

  volume {
    name = "fluvio-data"

    efs_volume_configuration {
      file_system_id     = aws_efs_file_system.fluvio[0].id
      transit_encryption = "ENABLED"
      authorization_config {
        access_point_id = aws_efs_access_point.fluvio_spu[0].id
        iam             = "ENABLED"
      }
    }
  }

  container_definitions = jsonencode([
    {
      name      = "fluvio-spu"
      image     = var.fluvio_image
      essential = true

      portMappings = [
        { containerPort = 9010, protocol = "tcp" },
        { containerPort = 9011, protocol = "tcp" }
      ]

      environment = [
        { name = "RUST_LOG", value = "info" },
        { name = "FLV_SC_PRIVATE_HOST", value = "fluvio-sc.${var.environment}.shortas.local" },
        { name = "FLV_SC_PRIVATE_PORT", value = "9003" }
      ]

      mountPoints = [
        {
          sourceVolume  = "fluvio-data"
          containerPath = "/var/lib/fluvio"
          readOnly      = false
        }
      ]

      logConfiguration = {
        logDriver = "awslogs"
        options = {
          "awslogs-group"         = aws_cloudwatch_log_group.services["fluvio-spu"].name
          "awslogs-region"        = var.region
          "awslogs-stream-prefix" = "ecs"
        }
      }
    }
  ])

  tags = {
    Name        = "${var.environment}-fluvio-spu-task"
    Environment = var.environment
    Service     = "fluvio-spu"
  }
}

# Keycloak Service
resource "aws_ecs_service" "keycloak" {
  count = var.enable_keycloak ? 1 : 0

  name            = "${var.environment}-keycloak"
  cluster         = aws_ecs_cluster.main.id
  task_definition = aws_ecs_task_definition.keycloak[0].arn
  desired_count   = var.keycloak_desired_count
  launch_type     = "FARGATE"

  enable_execute_command = var.enable_ecs_exec

  network_configuration {
    subnets          = var.private_subnet_ids
    security_groups  = [var.keycloak_security_group_id]
    assign_public_ip = false
  }

  load_balancer {
    target_group_arn = var.keycloak_target_group_arn
    container_name   = "keycloak"
    container_port   = 8080
  }

  service_registries {
    registry_arn = aws_service_discovery_service.services["keycloak"].arn
  }

  tags = {
    Name        = "${var.environment}-keycloak"
    Environment = var.environment
    Service     = "keycloak"
  }
}

resource "aws_ecs_task_definition" "keycloak" {
  count = var.enable_keycloak ? 1 : 0

  family                   = "${var.environment}-keycloak"
  network_mode             = "awsvpc"
  requires_compatibilities = ["FARGATE"]
  cpu                      = var.keycloak_cpu
  memory                   = var.keycloak_memory
  execution_role_arn       = var.task_execution_role_arn
  task_role_arn            = var.task_role_arn

  container_definitions = jsonencode([
    {
      name      = "keycloak"
      image     = var.keycloak_image
      essential = true

      portMappings = [
        { containerPort = 8080, protocol = "tcp" },
        { containerPort = 8443, protocol = "tcp" }
      ]

      environment = [
        { name = "KC_DB", value = "postgres" },
        { name = "KC_PROXY", value = "edge" },
        { name = "KC_HOSTNAME_STRICT", value = "false" },
        { name = "KC_HTTP_ENABLED", value = "true" }
      ]

      secrets = [
        {
          name      = "KC_DB_URL"
          valueFrom = "${var.rds_secret_arn}:host::"
        },
        {
          name      = "KC_DB_USERNAME"
          valueFrom = "${var.rds_secret_arn}:username::"
        },
        {
          name      = "KC_DB_PASSWORD"
          valueFrom = "${var.rds_secret_arn}:password::"
        },
        {
          name      = "KEYCLOAK_ADMIN"
          valueFrom = "${var.keycloak_secret_arn}:admin_username::"
        },
        {
          name      = "KEYCLOAK_ADMIN_PASSWORD"
          valueFrom = "${var.keycloak_secret_arn}:admin_password::"
        }
      ]

      logConfiguration = {
        logDriver = "awslogs"
        options = {
          "awslogs-group"         = aws_cloudwatch_log_group.services["keycloak"].name
          "awslogs-region"        = var.region
          "awslogs-stream-prefix" = "ecs"
        }
      }

      healthCheck = {
        command     = ["CMD-SHELL", "curl -f http://localhost:8080/health/ready || exit 1"]
        interval    = 30
        timeout     = 5
        retries     = 3
        startPeriod = 120
      }
    }
  ])

  tags = {
    Name        = "${var.environment}-keycloak-task"
    Environment = var.environment
    Service     = "keycloak"
  }
}

# Auto Scaling
resource "aws_appautoscaling_target" "services" {
  for_each = { for svc in var.services : svc.name => svc if svc.enable_autoscaling }

  max_capacity       = each.value.max_count
  min_capacity       = each.value.min_count
  resource_id        = "service/${aws_ecs_cluster.main.name}/${aws_ecs_service.services[each.key].name}"
  scalable_dimension = "ecs:service:DesiredCount"
  service_namespace  = "ecs"
}

resource "aws_appautoscaling_policy" "services_cpu" {
  for_each = { for svc in var.services : svc.name => svc if svc.enable_autoscaling }

  name               = "${var.environment}-${each.value.name}-cpu-scaling"
  policy_type        = "TargetTrackingScaling"
  resource_id        = aws_appautoscaling_target.services[each.key].resource_id
  scalable_dimension = aws_appautoscaling_target.services[each.key].scalable_dimension
  service_namespace  = aws_appautoscaling_target.services[each.key].service_namespace

  target_tracking_scaling_policy_configuration {
    predefined_metric_specification {
      predefined_metric_type = "ECSServiceAverageCPUUtilization"
    }
    target_value       = each.value.cpu_target_value
    scale_in_cooldown  = 300
    scale_out_cooldown = 60
  }
}
