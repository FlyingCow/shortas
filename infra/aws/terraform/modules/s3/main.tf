# S3 Module for Shortas URL Shortener

# Route Images Bucket - Public read for serving favicons/QR codes
resource "aws_s3_bucket" "route_images" {
  bucket = "${var.project_name}-route-images-${var.environment}"

  tags = {
    Name        = "${var.project_name}-route-images-${var.environment}"
    Environment = var.environment
  }
}

resource "aws_s3_bucket_ownership_controls" "route_images" {
  bucket = aws_s3_bucket.route_images.id
  rule {
    object_ownership = "BucketOwnerPreferred"
  }
}

resource "aws_s3_bucket_public_access_block" "route_images" {
  bucket = aws_s3_bucket.route_images.id

  block_public_acls       = false
  block_public_policy     = false
  ignore_public_acls      = false
  restrict_public_buckets = false
}

resource "aws_s3_bucket_policy" "route_images_public_read" {
  bucket = aws_s3_bucket.route_images.id

  policy = jsonencode({
    Version = "2012-10-17"
    Statement = [
      {
        Sid       = "PublicReadGetObject"
        Effect    = "Allow"
        Principal = "*"
        Action    = "s3:GetObject"
        Resource  = "${aws_s3_bucket.route_images.arn}/*"
      }
    ]
  })

  depends_on = [aws_s3_bucket_public_access_block.route_images]
}

resource "aws_s3_bucket_cors_configuration" "route_images" {
  bucket = aws_s3_bucket.route_images.id

  cors_rule {
    allowed_headers = ["*"]
    allowed_methods = ["GET", "HEAD"]
    allowed_origins = var.cors_allowed_origins
    expose_headers  = ["ETag"]
    max_age_seconds = 3600
  }
}

resource "aws_s3_bucket_versioning" "route_images" {
  bucket = aws_s3_bucket.route_images.id
  versioning_configuration {
    status = var.enable_versioning ? "Enabled" : "Disabled"
  }
}

resource "aws_s3_bucket_lifecycle_configuration" "route_images" {
  count  = var.enable_lifecycle_rules ? 1 : 0
  bucket = aws_s3_bucket.route_images.id

  rule {
    id     = "cleanup-old-versions"
    status = "Enabled"

    noncurrent_version_expiration {
      noncurrent_days = 30
    }
  }

  rule {
    id     = "abort-incomplete-uploads"
    status = "Enabled"

    abort_incomplete_multipart_upload {
      days_after_initiation = 7
    }
  }
}

# ClickHouse Backups Bucket - Private
resource "aws_s3_bucket" "clickhouse_backups" {
  bucket = "${var.project_name}-clickhouse-backup-${var.environment}"

  tags = {
    Name        = "${var.project_name}-clickhouse-backup-${var.environment}"
    Environment = var.environment
  }
}

resource "aws_s3_bucket_public_access_block" "clickhouse_backups" {
  bucket = aws_s3_bucket.clickhouse_backups.id

  block_public_acls       = true
  block_public_policy     = true
  ignore_public_acls      = true
  restrict_public_buckets = true
}

resource "aws_s3_bucket_versioning" "clickhouse_backups" {
  bucket = aws_s3_bucket.clickhouse_backups.id
  versioning_configuration {
    status = "Enabled"
  }
}

resource "aws_s3_bucket_server_side_encryption_configuration" "clickhouse_backups" {
  bucket = aws_s3_bucket.clickhouse_backups.id

  rule {
    apply_server_side_encryption_by_default {
      sse_algorithm = "aws:kms"
    }
  }
}

resource "aws_s3_bucket_lifecycle_configuration" "clickhouse_backups" {
  bucket = aws_s3_bucket.clickhouse_backups.id

  rule {
    id     = "transition-to-glacier"
    status = "Enabled"

    transition {
      days          = var.backup_transition_days
      storage_class = "GLACIER"
    }

    expiration {
      days = var.backup_expiration_days
    }
  }

  rule {
    id     = "cleanup-old-versions"
    status = "Enabled"

    noncurrent_version_expiration {
      noncurrent_days = 30
    }
  }
}

# Terraform State Bucket (optional)
resource "aws_s3_bucket" "terraform_state" {
  count  = var.create_terraform_state_bucket ? 1 : 0
  bucket = "${var.project_name}-terraform-state-${var.environment}"

  tags = {
    Name        = "${var.project_name}-terraform-state-${var.environment}"
    Environment = var.environment
    Purpose     = "Terraform State"
  }
}

resource "aws_s3_bucket_public_access_block" "terraform_state" {
  count  = var.create_terraform_state_bucket ? 1 : 0
  bucket = aws_s3_bucket.terraform_state[0].id

  block_public_acls       = true
  block_public_policy     = true
  ignore_public_acls      = true
  restrict_public_buckets = true
}

resource "aws_s3_bucket_versioning" "terraform_state" {
  count  = var.create_terraform_state_bucket ? 1 : 0
  bucket = aws_s3_bucket.terraform_state[0].id
  versioning_configuration {
    status = "Enabled"
  }
}

resource "aws_s3_bucket_server_side_encryption_configuration" "terraform_state" {
  count  = var.create_terraform_state_bucket ? 1 : 0
  bucket = aws_s3_bucket.terraform_state[0].id

  rule {
    apply_server_side_encryption_by_default {
      sse_algorithm = "aws:kms"
    }
  }
}

# DynamoDB table for Terraform state locking
resource "aws_dynamodb_table" "terraform_locks" {
  count        = var.create_terraform_state_bucket ? 1 : 0
  name         = "${var.project_name}-terraform-locks-${var.environment}"
  billing_mode = "PAY_PER_REQUEST"
  hash_key     = "LockID"

  attribute {
    name = "LockID"
    type = "S"
  }

  tags = {
    Name        = "${var.project_name}-terraform-locks-${var.environment}"
    Environment = var.environment
    Purpose     = "Terraform State Locking"
  }
}

# Application Logs Bucket (optional - for long-term log archival)
resource "aws_s3_bucket" "logs" {
  count  = var.create_logs_bucket ? 1 : 0
  bucket = "${var.project_name}-logs-${var.environment}"

  tags = {
    Name        = "${var.project_name}-logs-${var.environment}"
    Environment = var.environment
  }
}

resource "aws_s3_bucket_public_access_block" "logs" {
  count  = var.create_logs_bucket ? 1 : 0
  bucket = aws_s3_bucket.logs[0].id

  block_public_acls       = true
  block_public_policy     = true
  ignore_public_acls      = true
  restrict_public_buckets = true
}

resource "aws_s3_bucket_lifecycle_configuration" "logs" {
  count  = var.create_logs_bucket ? 1 : 0
  bucket = aws_s3_bucket.logs[0].id

  rule {
    id     = "transition-logs"
    status = "Enabled"

    transition {
      days          = 30
      storage_class = "STANDARD_IA"
    }

    transition {
      days          = 90
      storage_class = "GLACIER"
    }

    expiration {
      days = 365
    }
  }
}
