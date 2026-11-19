# cognito module

A Cognito user pool that replaces Keycloak on AWS. It creates:

- email sign-in, with an optional TOTP second factor;
- a **dashboard client**: public SPA, authorization code + PKCE, scopes `openid email profile`;
- an **API client**: confidential, client credentials, scope `shortas-api/full_access`;
- groups `admin` (precedence 1) and `user` (10), exposed in the `cognito:groups` claim;
- a custom attribute `workspace_id`;
- optionally, an identity pool.

To validate tokens, point the Management API's `jwt` settings at the `issuer_url` and `jwks_url` outputs.

## Usage

```hcl
module "cognito" {
  source      = "../../modules/cognito"
  environment = "dev"

  mfa_configuration      = "OPTIONAL"   # prod: "ON"
  advanced_security_mode = "AUDIT"      # prod: "ENFORCED"

  dashboard_callback_urls = ["https://app.example.com/callback", "http://localhost:3000/callback"]
  dashboard_logout_urls   = ["https://app.example.com", "http://localhost:3000"]
}
```

## Inputs

| Name | Default | Notes |
|---|---|---|
| `environment` | (required) | `dev`, `prod` |
| `password_minimum_length` | `8` | prod uses 12 |
| `password_require_lowercase` / `_uppercase` / `_numbers` | `true` | |
| `password_require_symbols` | `false` | |
| `mfa_configuration` | `"OPTIONAL"` | `OFF`, `ON`, `OPTIONAL` |
| `advanced_security_mode` | `"AUDIT"` | `OFF`, `AUDIT`, `ENFORCED` |
| `ses_email_identity`, `from_email_address` | `null` | send Cognito emails through SES |
| `custom_domain`, `certificate_arn` | `null` | custom hosted-UI domain with an ACM certificate |
| `dashboard_callback_urls` | `["http://localhost:3000/callback"]` | |
| `dashboard_logout_urls` | `["http://localhost:3000"]` | |
| `access_token_validity_hours`, `id_token_validity_hours` | `1` | |
| `refresh_token_validity_days` | `30` | |
| `enable_identity_pool` | `false` | |

## Outputs

`user_pool_id`, `user_pool_arn`, `user_pool_endpoint`, `user_pool_domain`, `hosted_ui_url`, `dashboard_client_id`, `api_client_id`, `api_client_secret` (sensitive), `issuer_url`, `jwks_url`, and, if the identity pool is enabled, `identity_pool_id` and `authenticated_role_arn`.

## Moving users from Keycloak

Passwords can't be exported. Import the users, then have them reset their passwords, or add a "migrate user" Lambda trigger to move them over when they next sign in.
