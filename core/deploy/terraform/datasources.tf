resource "aws_dynamodb_table" "core-routes-table" {
  name           = "core-routes-${var.stage}"
  billing_mode   = "PAY_PER_REQUEST"
  hash_key       = "link"
  range_key      = "switch"

  attribute {
    name = "link"
    type = "S"
  }

  attribute {
    name = "switch"
    type = "S"
  }

  ttl {
    attribute_name = "TimeToExist"
    enabled        = false
  }

  tags = {
    Name        = "core-routes-${var.stage}"
    Environment = "${var.stage}"
  }
}

resource "aws_dynamodb_table" "core-routes-encryption-table" {
  name           = "core-routes-encryption-${var.stage}"
  billing_mode   = "PAY_PER_REQUEST"
  hash_key       = "hostname"

  attribute {
    name = "hostname"
    type = "S"
  }

  ttl {
    attribute_name = "TimeToExist"
    enabled        = false
  }

  tags = {
    Name        = "core-routes-encryption-${var.stage}"
    Environment = "${var.stage}"
  }
}

resource "aws_dynamodb_table" "core-routes-hostname-mapping-table" {
  name           = "core-routes-hostname-mapping-${var.stage}"
  billing_mode   = "PAY_PER_REQUEST"
  hash_key       = "hostname"

  attribute {
    name = "hostname"
    type = "S"
  }

  ttl {
    attribute_name = "TimeToExist"
    enabled        = false
  }

  tags = {
    Name        = "core-routes-hostname-mapping-${var.stage}"
    Environment = "${var.stage}"
  }
}

resource "aws_dynamodb_table" "core-user-settings-table" {
  name           = "core-user-settings-${var.stage}"
  billing_mode   = "PAY_PER_REQUEST"
  hash_key       = "userid"

  attribute {
    name = "userid"
    type = "S"
  }

  ttl {
    attribute_name = "TimeToExist"
    enabled        = false
  }

  tags = {
    Name        = "core-user-settings-${var.stage}"
    Environment = "${var.stage}"
  }
}