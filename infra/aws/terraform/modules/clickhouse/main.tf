# ClickHouse EC2 Module for Shortas URL Shortener

# Get latest Amazon Linux 2 AMI
data "aws_ami" "amazon_linux_2" {
  most_recent = true
  owners      = ["amazon"]

  filter {
    name   = "name"
    values = ["amzn2-ami-hvm-*-x86_64-gp2"]
  }

  filter {
    name   = "virtualization-type"
    values = ["hvm"]
  }
}

# EBS Volume for ClickHouse data
resource "aws_ebs_volume" "clickhouse_data" {
  availability_zone = var.availability_zone
  size              = var.data_volume_size
  type              = var.data_volume_type
  iops              = var.data_volume_type == "io1" || var.data_volume_type == "io2" ? var.data_volume_iops : null
  encrypted         = true

  tags = {
    Name        = "${var.environment}-shortas-clickhouse-data"
    Environment = var.environment
  }
}

# EC2 Instance
resource "aws_instance" "clickhouse" {
  ami                    = var.ami_id != "" ? var.ami_id : data.aws_ami.amazon_linux_2.id
  instance_type          = var.instance_type
  subnet_id              = var.subnet_id
  vpc_security_group_ids = [var.security_group_id]
  iam_instance_profile   = var.instance_profile_name
  key_name               = var.key_name

  root_block_device {
    volume_size           = 30
    volume_type           = "gp3"
    encrypted             = true
    delete_on_termination = true
  }

  user_data = base64encode(templatefile("${path.module}/user_data.sh", {
    environment           = var.environment
    clickhouse_version    = var.clickhouse_version
    clickhouse_password   = var.clickhouse_password
    data_volume_device    = "/dev/xvdf"
    s3_backup_bucket      = var.s3_backup_bucket
    region                = var.region
  }))

  monitoring = var.enable_detailed_monitoring

  metadata_options {
    http_endpoint               = "enabled"
    http_tokens                 = "required" # IMDSv2
    http_put_response_hop_limit = 1
  }

  tags = {
    Name        = "${var.environment}-shortas-clickhouse"
    Environment = var.environment
  }

  lifecycle {
    ignore_changes = [ami, user_data]
  }
}

# Attach data volume
resource "aws_volume_attachment" "clickhouse_data" {
  device_name = "/dev/xvdf"
  volume_id   = aws_ebs_volume.clickhouse_data.id
  instance_id = aws_instance.clickhouse.id
}

# Elastic IP (optional)
resource "aws_eip" "clickhouse" {
  count  = var.assign_elastic_ip ? 1 : 0
  domain = "vpc"

  tags = {
    Name        = "${var.environment}-shortas-clickhouse-eip"
    Environment = var.environment
  }
}

resource "aws_eip_association" "clickhouse" {
  count         = var.assign_elastic_ip ? 1 : 0
  instance_id   = aws_instance.clickhouse.id
  allocation_id = aws_eip.clickhouse[0].id
}

# CloudWatch Alarms
resource "aws_cloudwatch_metric_alarm" "cpu" {
  count               = var.enable_cloudwatch_alarms ? 1 : 0
  alarm_name          = "${var.environment}-shortas-clickhouse-cpu"
  comparison_operator = "GreaterThanThreshold"
  evaluation_periods  = 2
  metric_name         = "CPUUtilization"
  namespace           = "AWS/EC2"
  period              = 300
  statistic           = "Average"
  threshold           = 80
  alarm_description   = "ClickHouse CPU utilization is high"

  dimensions = {
    InstanceId = aws_instance.clickhouse.id
  }

  alarm_actions = var.alarm_actions

  tags = {
    Name        = "${var.environment}-shortas-clickhouse-cpu-alarm"
    Environment = var.environment
  }
}

resource "aws_cloudwatch_metric_alarm" "disk" {
  count               = var.enable_cloudwatch_alarms ? 1 : 0
  alarm_name          = "${var.environment}-shortas-clickhouse-disk"
  comparison_operator = "GreaterThanThreshold"
  evaluation_periods  = 2
  metric_name         = "disk_used_percent"
  namespace           = "CWAgent"
  period              = 300
  statistic           = "Average"
  threshold           = 80
  alarm_description   = "ClickHouse disk usage is high"

  dimensions = {
    InstanceId = aws_instance.clickhouse.id
    path       = "/var/lib/clickhouse"
    device     = "xvdf"
    fstype     = "ext4"
  }

  alarm_actions = var.alarm_actions

  tags = {
    Name        = "${var.environment}-shortas-clickhouse-disk-alarm"
    Environment = var.environment
  }
}
