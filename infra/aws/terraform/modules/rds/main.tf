# RDS Aurora PostgreSQL Serverless v2 Module for Shortas

# Random password for RDS
resource "random_password" "master" {
  length           = 32
  special          = true
  override_special = "!#$%&*()-_=+[]{}<>:?"
}

# Store credentials in Secrets Manager
resource "aws_secretsmanager_secret" "rds" {
  name                    = "shortas/${var.environment}/rds"
  description             = "RDS Aurora PostgreSQL credentials for Shortas"
  recovery_window_in_days = var.environment == "prod" ? 30 : 0

  tags = {
    Name        = "shortas-${var.environment}-rds-secret"
    Environment = var.environment
  }
}

resource "aws_secretsmanager_secret_version" "rds" {
  secret_id = aws_secretsmanager_secret.rds.id
  secret_string = jsonencode({
    username             = var.master_username
    password             = random_password.master.result
    engine               = "postgres"
    host                 = aws_rds_cluster.main.endpoint
    port                 = aws_rds_cluster.main.port
    dbname               = var.database_name
    dbClusterIdentifier  = aws_rds_cluster.main.cluster_identifier
    reader_endpoint      = aws_rds_cluster.main.reader_endpoint
  })

  depends_on = [aws_rds_cluster.main]
}

# DB Cluster Parameter Group
resource "aws_rds_cluster_parameter_group" "main" {
  name        = "${var.environment}-shortas-aurora-pg-params"
  family      = "aurora-postgresql15"
  description = "Aurora PostgreSQL parameter group for Shortas"

  parameter {
    name  = "log_statement"
    value = "ddl"
  }

  parameter {
    name  = "log_min_duration_statement"
    value = "1000" # Log queries taking more than 1 second
  }

  parameter {
    name         = "shared_preload_libraries"
    value        = "pg_stat_statements"
    apply_method = "pending-reboot"
  }

  tags = {
    Name        = "${var.environment}-shortas-aurora-pg-params"
    Environment = var.environment
  }
}

# DB Parameter Group for instances
resource "aws_db_parameter_group" "main" {
  name        = "${var.environment}-shortas-aurora-pg-instance-params"
  family      = "aurora-postgresql15"
  description = "Aurora PostgreSQL instance parameter group for Shortas"

  tags = {
    Name        = "${var.environment}-shortas-aurora-pg-instance-params"
    Environment = var.environment
  }
}

# Aurora PostgreSQL Cluster
resource "aws_rds_cluster" "main" {
  cluster_identifier = "${var.environment}-shortas-aurora-cluster"
  engine             = "aurora-postgresql"
  engine_mode        = "provisioned"
  engine_version     = var.engine_version
  database_name      = var.database_name
  master_username    = var.master_username
  master_password    = random_password.master.result

  db_subnet_group_name            = var.db_subnet_group_name
  vpc_security_group_ids          = [var.security_group_id]
  db_cluster_parameter_group_name = aws_rds_cluster_parameter_group.main.name

  storage_encrypted   = true
  deletion_protection = var.environment == "prod"
  skip_final_snapshot = var.environment != "prod"
  final_snapshot_identifier = var.environment == "prod" ? "${var.environment}-shortas-final-snapshot-${formatdate("YYYY-MM-DD-hhmm", timestamp())}" : null

  backup_retention_period      = var.backup_retention_period
  preferred_backup_window      = "03:00-04:00"
  preferred_maintenance_window = "sun:04:00-sun:05:00"

  enabled_cloudwatch_logs_exports = ["postgresql"]

  serverlessv2_scaling_configuration {
    min_capacity = var.min_capacity
    max_capacity = var.max_capacity
  }

  tags = {
    Name        = "${var.environment}-shortas-aurora-cluster"
    Environment = var.environment
  }

  lifecycle {
    ignore_changes = [final_snapshot_identifier]
  }
}

# Aurora Cluster Instances
resource "aws_rds_cluster_instance" "main" {
  count = var.instance_count

  identifier           = "${var.environment}-shortas-aurora-${count.index + 1}"
  cluster_identifier   = aws_rds_cluster.main.id
  instance_class       = "db.serverless"
  engine               = aws_rds_cluster.main.engine
  engine_version       = aws_rds_cluster.main.engine_version
  db_parameter_group_name = aws_db_parameter_group.main.name

  publicly_accessible     = false
  db_subnet_group_name    = var.db_subnet_group_name

  performance_insights_enabled          = var.enable_performance_insights
  performance_insights_retention_period = var.enable_performance_insights ? 7 : null

  monitoring_interval = var.enable_enhanced_monitoring ? 60 : 0
  monitoring_role_arn = var.enable_enhanced_monitoring ? aws_iam_role.rds_monitoring[0].arn : null

  auto_minor_version_upgrade = true

  tags = {
    Name        = "${var.environment}-shortas-aurora-${count.index + 1}"
    Environment = var.environment
  }
}

# IAM Role for Enhanced Monitoring
resource "aws_iam_role" "rds_monitoring" {
  count = var.enable_enhanced_monitoring ? 1 : 0
  name  = "${var.environment}-shortas-rds-monitoring-role"

  assume_role_policy = jsonencode({
    Version = "2012-10-17"
    Statement = [
      {
        Action = "sts:AssumeRole"
        Effect = "Allow"
        Principal = {
          Service = "monitoring.rds.amazonaws.com"
        }
      }
    ]
  })

  tags = {
    Name        = "${var.environment}-shortas-rds-monitoring-role"
    Environment = var.environment
  }
}

resource "aws_iam_role_policy_attachment" "rds_monitoring" {
  count      = var.enable_enhanced_monitoring ? 1 : 0
  role       = aws_iam_role.rds_monitoring[0].name
  policy_arn = "arn:aws:iam::aws:policy/service-role/AmazonRDSEnhancedMonitoringRole"
}
