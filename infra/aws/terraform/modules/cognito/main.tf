# AWS Cognito Module for Shortas URL Shortener
# Replaces Keycloak for AWS deployments

# Cognito User Pool
resource "aws_cognito_user_pool" "main" {
  name = "${var.environment}-shortas-users"

  # Username configuration
  username_attributes      = ["email"]
  auto_verified_attributes = ["email"]

  # Password policy
  password_policy {
    minimum_length                   = var.password_minimum_length
    require_lowercase                = var.password_require_lowercase
    require_uppercase                = var.password_require_uppercase
    require_numbers                  = var.password_require_numbers
    require_symbols                  = var.password_require_symbols
    temporary_password_validity_days = 7
  }

  # Account recovery
  account_recovery_setting {
    recovery_mechanism {
      name     = "verified_email"
      priority = 1
    }
  }

  # MFA configuration
  mfa_configuration = var.mfa_configuration

  dynamic "software_token_mfa_configuration" {
    for_each = var.mfa_configuration != "OFF" ? [1] : []
    content {
      enabled = true
    }
  }

  # Email configuration
  email_configuration {
    email_sending_account = var.ses_email_identity != null ? "DEVELOPER" : "COGNITO_DEFAULT"
    source_arn            = var.ses_email_identity
    from_email_address    = var.from_email_address
  }

  # User attribute schema
  schema {
    name                     = "email"
    attribute_data_type      = "String"
    required                 = true
    mutable                  = true
    developer_only_attribute = false

    string_attribute_constraints {
      min_length = 5
      max_length = 256
    }
  }

  schema {
    name                     = "name"
    attribute_data_type      = "String"
    required                 = false
    mutable                  = true
    developer_only_attribute = false

    string_attribute_constraints {
      min_length = 1
      max_length = 256
    }
  }

  # Custom attributes
  schema {
    name                     = "workspace_id"
    attribute_data_type      = "String"
    required                 = false
    mutable                  = true
    developer_only_attribute = false

    string_attribute_constraints {
      min_length = 0
      max_length = 256
    }
  }

  # Verification message customization
  verification_message_template {
    default_email_option = "CONFIRM_WITH_CODE"
    email_subject        = "Your Shortas verification code"
    email_message        = "Your verification code is {####}"
  }

  # User pool add-ons
  user_pool_add_ons {
    advanced_security_mode = var.advanced_security_mode
  }

  # Device tracking
  device_configuration {
    challenge_required_on_new_device      = false
    device_only_remembered_on_user_prompt = true
  }

  # Admin create user config
  admin_create_user_config {
    allow_admin_create_user_only = false

    invite_message_template {
      email_subject = "Welcome to Shortas"
      email_message = "Your username is {username} and temporary password is {####}."
      sms_message   = "Your username is {username} and temporary password is {####}."
    }
  }

  tags = {
    Name        = "${var.environment}-shortas-users"
    Environment = var.environment
  }
}

# User Pool Domain
resource "aws_cognito_user_pool_domain" "main" {
  domain          = var.custom_domain != null ? var.custom_domain : "${var.environment}-shortas-auth"
  user_pool_id    = aws_cognito_user_pool.main.id
  certificate_arn = var.custom_domain != null ? var.certificate_arn : null
}

# App Client for Dashboard (SPA - public client)
resource "aws_cognito_user_pool_client" "dashboard" {
  name         = "${var.environment}-shortas-dashboard"
  user_pool_id = aws_cognito_user_pool.main.id

  # OAuth configuration
  allowed_oauth_flows                  = ["code"]
  allowed_oauth_flows_user_pool_client = true
  allowed_oauth_scopes                 = ["email", "openid", "profile"]
  supported_identity_providers         = ["COGNITO"]

  # Callback URLs
  callback_urls = var.dashboard_callback_urls
  logout_urls   = var.dashboard_logout_urls

  # Token configuration
  access_token_validity  = var.access_token_validity_hours
  id_token_validity      = var.id_token_validity_hours
  refresh_token_validity = var.refresh_token_validity_days

  token_validity_units {
    access_token  = "hours"
    id_token      = "hours"
    refresh_token = "days"
  }

  # Security settings for SPA (public client)
  generate_secret                      = false
  prevent_user_existence_errors        = "ENABLED"
  enable_token_revocation              = true
  enable_propagate_additional_user_context_data = false

  # Auth flows
  explicit_auth_flows = [
    "ALLOW_REFRESH_TOKEN_AUTH",
    "ALLOW_USER_SRP_AUTH"
  ]

  # Read/write attributes
  read_attributes  = ["email", "name", "custom:workspace_id"]
  write_attributes = ["email", "name", "custom:workspace_id"]
}

# App Client for API (confidential client)
resource "aws_cognito_user_pool_client" "api" {
  name         = "${var.environment}-shortas-api"
  user_pool_id = aws_cognito_user_pool.main.id

  # OAuth configuration
  allowed_oauth_flows                  = ["client_credentials"]
  allowed_oauth_flows_user_pool_client = true
  allowed_oauth_scopes                 = [aws_cognito_resource_server.api.scope_identifiers[0]]
  supported_identity_providers         = ["COGNITO"]

  # Token configuration
  access_token_validity  = var.access_token_validity_hours
  id_token_validity      = var.id_token_validity_hours
  refresh_token_validity = var.refresh_token_validity_days

  token_validity_units {
    access_token  = "hours"
    id_token      = "hours"
    refresh_token = "days"
  }

  # Security settings for API (confidential client)
  generate_secret               = true
  prevent_user_existence_errors = "ENABLED"
  enable_token_revocation       = true

  # Auth flows
  explicit_auth_flows = [
    "ALLOW_REFRESH_TOKEN_AUTH"
  ]
}

# Resource Server for API scopes
resource "aws_cognito_resource_server" "api" {
  identifier   = "shortas-api"
  name         = "Shortas API"
  user_pool_id = aws_cognito_user_pool.main.id

  scope {
    scope_name        = "full_access"
    scope_description = "Full access to Shortas API"
  }
}

# User Pool Groups
resource "aws_cognito_user_group" "admin" {
  name         = "admin"
  user_pool_id = aws_cognito_user_pool.main.id
  description  = "Administrator users with full access"
  precedence   = 1
}

resource "aws_cognito_user_group" "user" {
  name         = "user"
  user_pool_id = aws_cognito_user_pool.main.id
  description  = "Regular users"
  precedence   = 10
}

# Identity Pool for federated access (optional - for AWS resource access)
resource "aws_cognito_identity_pool" "main" {
  count = var.enable_identity_pool ? 1 : 0

  identity_pool_name               = "${var.environment}-shortas-identity"
  allow_unauthenticated_identities = false
  allow_classic_flow               = false

  cognito_identity_providers {
    client_id               = aws_cognito_user_pool_client.dashboard.id
    provider_name           = aws_cognito_user_pool.main.endpoint
    server_side_token_check = true
  }

  tags = {
    Name        = "${var.environment}-shortas-identity"
    Environment = var.environment
  }
}

# IAM roles for Identity Pool
resource "aws_iam_role" "authenticated" {
  count = var.enable_identity_pool ? 1 : 0

  name = "${var.environment}-shortas-cognito-authenticated"

  assume_role_policy = jsonencode({
    Version = "2012-10-17"
    Statement = [
      {
        Effect = "Allow"
        Principal = {
          Federated = "cognito-identity.amazonaws.com"
        }
        Action = "sts:AssumeRoleWithWebIdentity"
        Condition = {
          StringEquals = {
            "cognito-identity.amazonaws.com:aud" = aws_cognito_identity_pool.main[0].id
          }
          "ForAnyValue:StringLike" = {
            "cognito-identity.amazonaws.com:amr" = "authenticated"
          }
        }
      }
    ]
  })

  tags = {
    Name        = "${var.environment}-shortas-cognito-authenticated"
    Environment = var.environment
  }
}

resource "aws_iam_role_policy" "authenticated" {
  count = var.enable_identity_pool ? 1 : 0

  name = "${var.environment}-shortas-cognito-authenticated-policy"
  role = aws_iam_role.authenticated[0].id

  policy = jsonencode({
    Version = "2012-10-17"
    Statement = [
      {
        Effect = "Allow"
        Action = [
          "cognito-sync:*",
          "cognito-identity:*"
        ]
        Resource = "*"
      }
    ]
  })
}

resource "aws_cognito_identity_pool_roles_attachment" "main" {
  count = var.enable_identity_pool ? 1 : 0

  identity_pool_id = aws_cognito_identity_pool.main[0].id

  roles = {
    "authenticated" = aws_iam_role.authenticated[0].arn
  }
}
