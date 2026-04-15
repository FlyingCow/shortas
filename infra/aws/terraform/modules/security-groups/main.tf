# Security Groups Module for Shortas URL Shortener

# ALB Security Group - Public facing
resource "aws_security_group" "alb" {
  name        = "${var.environment}-shortas-alb-sg"
  description = "Security group for Application Load Balancer"
  vpc_id      = var.vpc_id

  ingress {
    description = "HTTP from internet"
    from_port   = 80
    to_port     = 80
    protocol    = "tcp"
    cidr_blocks = ["0.0.0.0/0"]
  }

  ingress {
    description = "HTTPS from internet"
    from_port   = 443
    to_port     = 443
    protocol    = "tcp"
    cidr_blocks = ["0.0.0.0/0"]
  }

  egress {
    from_port   = 0
    to_port     = 0
    protocol    = "-1"
    cidr_blocks = ["0.0.0.0/0"]
  }

  tags = {
    Name        = "${var.environment}-shortas-alb-sg"
    Environment = var.environment
  }
}

# Internal ALB Security Group - For service-to-service communication
resource "aws_security_group" "internal_alb" {
  name        = "${var.environment}-shortas-internal-alb-sg"
  description = "Security group for Internal Application Load Balancer"
  vpc_id      = var.vpc_id

  ingress {
    description = "HTTP from VPC"
    from_port   = 80
    to_port     = 80
    protocol    = "tcp"
    cidr_blocks = [var.vpc_cidr]
  }

  ingress {
    description = "HTTPS from VPC"
    from_port   = 443
    to_port     = 443
    protocol    = "tcp"
    cidr_blocks = [var.vpc_cidr]
  }

  egress {
    from_port   = 0
    to_port     = 0
    protocol    = "-1"
    cidr_blocks = ["0.0.0.0/0"]
  }

  tags = {
    Name        = "${var.environment}-shortas-internal-alb-sg"
    Environment = var.environment
  }
}

# ECS Services Security Group
resource "aws_security_group" "ecs" {
  name        = "${var.environment}-shortas-ecs-sg"
  description = "Security group for ECS services"
  vpc_id      = var.vpc_id

  # Allow traffic from public ALB
  ingress {
    description     = "Traffic from public ALB"
    from_port       = 0
    to_port         = 65535
    protocol        = "tcp"
    security_groups = [aws_security_group.alb.id]
  }

  # Allow traffic from internal ALB
  ingress {
    description     = "Traffic from internal ALB"
    from_port       = 0
    to_port         = 65535
    protocol        = "tcp"
    security_groups = [aws_security_group.internal_alb.id]
  }

  # Allow inter-service communication within ECS
  ingress {
    description = "Inter-service communication"
    from_port   = 0
    to_port     = 65535
    protocol    = "tcp"
    self        = true
  }

  egress {
    from_port   = 0
    to_port     = 0
    protocol    = "-1"
    cidr_blocks = ["0.0.0.0/0"]
  }

  tags = {
    Name        = "${var.environment}-shortas-ecs-sg"
    Environment = var.environment
  }
}

# RDS Security Group
resource "aws_security_group" "rds" {
  name        = "${var.environment}-shortas-rds-sg"
  description = "Security group for RDS PostgreSQL"
  vpc_id      = var.vpc_id

  ingress {
    description     = "PostgreSQL from ECS"
    from_port       = 5432
    to_port         = 5432
    protocol        = "tcp"
    security_groups = [aws_security_group.ecs.id]
  }

  # Allow from ClickHouse if needed for analytics
  ingress {
    description     = "PostgreSQL from ClickHouse"
    from_port       = 5432
    to_port         = 5432
    protocol        = "tcp"
    security_groups = [aws_security_group.clickhouse.id]
  }

  egress {
    from_port   = 0
    to_port     = 0
    protocol    = "-1"
    cidr_blocks = ["0.0.0.0/0"]
  }

  tags = {
    Name        = "${var.environment}-shortas-rds-sg"
    Environment = var.environment
  }
}

# ElastiCache Security Group
resource "aws_security_group" "elasticache" {
  name        = "${var.environment}-shortas-elasticache-sg"
  description = "Security group for ElastiCache Redis"
  vpc_id      = var.vpc_id

  ingress {
    description     = "Redis from ECS"
    from_port       = 6379
    to_port         = 6379
    protocol        = "tcp"
    security_groups = [aws_security_group.ecs.id]
  }

  egress {
    from_port   = 0
    to_port     = 0
    protocol    = "-1"
    cidr_blocks = ["0.0.0.0/0"]
  }

  tags = {
    Name        = "${var.environment}-shortas-elasticache-sg"
    Environment = var.environment
  }
}

# Amazon MQ Security Group
resource "aws_security_group" "mq" {
  name        = "${var.environment}-shortas-mq-sg"
  description = "Security group for Amazon MQ RabbitMQ"
  vpc_id      = var.vpc_id

  # AMQP
  ingress {
    description     = "AMQP from ECS"
    from_port       = 5672
    to_port         = 5672
    protocol        = "tcp"
    security_groups = [aws_security_group.ecs.id]
  }

  # AMQPS (TLS)
  ingress {
    description     = "AMQPS from ECS"
    from_port       = 5671
    to_port         = 5671
    protocol        = "tcp"
    security_groups = [aws_security_group.ecs.id]
  }

  # Management UI
  ingress {
    description     = "RabbitMQ Management from ECS"
    from_port       = 15672
    to_port         = 15672
    protocol        = "tcp"
    security_groups = [aws_security_group.ecs.id]
  }

  # Management UI over HTTPS
  ingress {
    description     = "RabbitMQ Management HTTPS from ECS"
    from_port       = 443
    to_port         = 443
    protocol        = "tcp"
    security_groups = [aws_security_group.ecs.id]
  }

  egress {
    from_port   = 0
    to_port     = 0
    protocol    = "-1"
    cidr_blocks = ["0.0.0.0/0"]
  }

  tags = {
    Name        = "${var.environment}-shortas-mq-sg"
    Environment = var.environment
  }
}

# ClickHouse EC2 Security Group
resource "aws_security_group" "clickhouse" {
  name        = "${var.environment}-shortas-clickhouse-sg"
  description = "Security group for ClickHouse EC2 instance"
  vpc_id      = var.vpc_id

  # HTTP interface
  ingress {
    description     = "ClickHouse HTTP from ECS"
    from_port       = 8123
    to_port         = 8123
    protocol        = "tcp"
    security_groups = [aws_security_group.ecs.id]
  }

  # Native protocol
  ingress {
    description     = "ClickHouse Native from ECS"
    from_port       = 9000
    to_port         = 9000
    protocol        = "tcp"
    security_groups = [aws_security_group.ecs.id]
  }

  # Inter-node communication (for clustering)
  ingress {
    description = "ClickHouse inter-node"
    from_port   = 9009
    to_port     = 9009
    protocol    = "tcp"
    self        = true
  }

  # SSH for management (optional, from bastion/VPN)
  dynamic "ingress" {
    for_each = var.enable_clickhouse_ssh ? [1] : []
    content {
      description = "SSH from bastion"
      from_port   = 22
      to_port     = 22
      protocol    = "tcp"
      cidr_blocks = var.ssh_cidr_blocks
    }
  }

  egress {
    from_port   = 0
    to_port     = 0
    protocol    = "-1"
    cidr_blocks = ["0.0.0.0/0"]
  }

  tags = {
    Name        = "${var.environment}-shortas-clickhouse-sg"
    Environment = var.environment
  }
}

# Fluvio Security Group (for SC and SPU communication)
resource "aws_security_group" "fluvio" {
  name        = "${var.environment}-shortas-fluvio-sg"
  description = "Security group for Fluvio streaming platform"
  vpc_id      = var.vpc_id

  # SC API port
  ingress {
    description     = "Fluvio SC from ECS"
    from_port       = 9003
    to_port         = 9003
    protocol        = "tcp"
    security_groups = [aws_security_group.ecs.id]
  }

  # SPU public port
  ingress {
    description     = "Fluvio SPU public from ECS"
    from_port       = 9010
    to_port         = 9010
    protocol        = "tcp"
    security_groups = [aws_security_group.ecs.id]
  }

  # SPU private port
  ingress {
    description     = "Fluvio SPU private from ECS"
    from_port       = 9011
    to_port         = 9011
    protocol        = "tcp"
    security_groups = [aws_security_group.ecs.id]
  }

  # Inter-Fluvio communication
  ingress {
    description = "Fluvio inter-node"
    from_port   = 9003
    to_port     = 9011
    protocol    = "tcp"
    self        = true
  }

  egress {
    from_port   = 0
    to_port     = 0
    protocol    = "-1"
    cidr_blocks = ["0.0.0.0/0"]
  }

  tags = {
    Name        = "${var.environment}-shortas-fluvio-sg"
    Environment = var.environment
  }
}

# Keycloak Security Group (conditional - only when not using Cognito)
resource "aws_security_group" "keycloak" {
  count = var.enable_keycloak ? 1 : 0

  name        = "${var.environment}-shortas-keycloak-sg"
  description = "Security group for Keycloak authentication service"
  vpc_id      = var.vpc_id

  # HTTP from ALB
  ingress {
    description     = "HTTP from ALB"
    from_port       = 8080
    to_port         = 8080
    protocol        = "tcp"
    security_groups = [aws_security_group.alb.id, aws_security_group.internal_alb.id]
  }

  # HTTPS from ALB
  ingress {
    description     = "HTTPS from ALB"
    from_port       = 8443
    to_port         = 8443
    protocol        = "tcp"
    security_groups = [aws_security_group.alb.id, aws_security_group.internal_alb.id]
  }

  egress {
    from_port   = 0
    to_port     = 0
    protocol    = "-1"
    cidr_blocks = ["0.0.0.0/0"]
  }

  tags = {
    Name        = "${var.environment}-shortas-keycloak-sg"
    Environment = var.environment
  }
}
