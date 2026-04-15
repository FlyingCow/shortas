# AWS Cognito Module

This module creates an AWS Cognito User Pool for authentication, replacing Keycloak in AWS deployments.

## Features

- User Pool with email-based authentication
- Configurable password policy
- Optional MFA (TOTP)
- Advanced security features (brute force protection)
- Dashboard app client (public SPA with PKCE)
- API app client (confidential with client credentials)
- Resource server with API scopes
- User groups (admin, user)
- Optional Identity Pool for federated AWS access

## Usage

```hcl
module "cognito" {
  source = "../../modules/cognito"

  environment = "dev"

  # Password policy
  password_minimum_length    = 8
  password_require_lowercase = true
  password_require_uppercase = true
  password_require_numbers   = true
  password_require_symbols   = false

  # MFA configuration
  mfa_configuration = "OPTIONAL"  # OFF, ON, or OPTIONAL

  # Advanced security
  advanced_security_mode = "AUDIT"  # OFF, AUDIT, or ENFORCED

  # Callback URLs for dashboard SPA
  dashboard_callback_urls = [
    "https://app.example.com/callback",
    "http://localhost:3000/callback"
  ]
  dashboard_logout_urls = [
    "https://app.example.com",
    "http://localhost:3000"
  ]

  # Token validity
  access_token_validity_hours  = 1
  id_token_validity_hours      = 1
  refresh_token_validity_days  = 30
}
```

## Inputs

| Name | Description | Type | Default | Required |
|------|-------------|------|---------|----------|
| environment | Environment name (dev, prod) | string | - | yes |
| password_minimum_length | Minimum password length | number | 8 | no |
| password_require_lowercase | Require lowercase letters | bool | true | no |
| password_require_uppercase | Require uppercase letters | bool | true | no |
| password_require_numbers | Require numbers | bool | true | no |
| password_require_symbols | Require symbols | bool | false | no |
| mfa_configuration | MFA configuration (OFF, ON, OPTIONAL) | string | "OPTIONAL" | no |
| advanced_security_mode | Security mode (OFF, AUDIT, ENFORCED) | string | "AUDIT" | no |
| ses_email_identity | SES email identity ARN for custom emails | string | null | no |
| from_email_address | From email address for Cognito emails | string | null | no |
| custom_domain | Custom domain for hosted UI | string | null | no |
| certificate_arn | ACM certificate for custom domain | string | null | no |
| dashboard_callback_urls | OAuth callback URLs for dashboard | list(string) | ["http://localhost:3000/callback"] | no |
| dashboard_logout_urls | Logout URLs for dashboard | list(string) | ["http://localhost:3000"] | no |
| access_token_validity_hours | Access token validity in hours | number | 1 | no |
| id_token_validity_hours | ID token validity in hours | number | 1 | no |
| refresh_token_validity_days | Refresh token validity in days | number | 30 | no |
| enable_identity_pool | Enable Identity Pool for AWS access | bool | false | no |

## Outputs

| Name | Description |
|------|-------------|
| user_pool_id | Cognito User Pool ID |
| user_pool_arn | Cognito User Pool ARN |
| user_pool_endpoint | Cognito User Pool endpoint |
| user_pool_domain | Cognito User Pool domain |
| hosted_ui_url | Cognito Hosted UI URL |
| dashboard_client_id | Dashboard app client ID |
| api_client_id | API app client ID |
| api_client_secret | API app client secret (sensitive) |
| issuer_url | OIDC issuer URL for token validation |
| jwks_url | JWKS URL for token validation |
| identity_pool_id | Identity Pool ID (if enabled) |
| authenticated_role_arn | IAM role for authenticated users (if enabled) |

## App Clients

### Dashboard Client (Public SPA)

For the React dashboard application:
- OAuth 2.0 Authorization Code flow with PKCE
- No client secret (public client)
- Scopes: `email`, `openid`, `profile`
- Token revocation enabled

### API Client (Confidential)

For server-to-server authentication:
- OAuth 2.0 Client Credentials flow
- Client secret generated
- Custom scope: `shortas-api/full_access`

## Integration

### React Dashboard

Configure the dashboard to use Cognito:

```typescript
// src/config/auth.ts
export const authConfig = {
  authority: process.env.REACT_APP_COGNITO_ISSUER_URL,
  client_id: process.env.REACT_APP_COGNITO_CLIENT_ID,
  redirect_uri: `${window.location.origin}/callback`,
  post_logout_redirect_uri: window.location.origin,
  scope: 'openid email profile',
  response_type: 'code',
};
```

Environment variables:
```bash
REACT_APP_COGNITO_ISSUER_URL=https://cognito-idp.us-east-1.amazonaws.com/us-east-1_xxxxxxxx
REACT_APP_COGNITO_CLIENT_ID=xxxxxxxxxxxxxxxxxxxxxxxxxx
```

### .NET API

Configure ASP.NET Core to validate Cognito tokens:

```json
// appsettings.Aws.json
{
  "Authentication": {
    "Provider": "Cognito",
    "Cognito": {
      "Region": "us-east-1",
      "UserPoolId": "${COGNITO_USER_POOL_ID}",
      "AppClientId": "${COGNITO_APP_CLIENT_ID}",
      "Authority": "${COGNITO_ISSUER_URL}",
      "JwksUrl": "${COGNITO_JWKS_URL}"
    }
  }
}
```

```csharp
// Program.cs
services.AddAuthentication(JwtBearerDefaults.AuthenticationScheme)
    .AddJwtBearer(options =>
    {
        options.Authority = configuration["Authentication:Cognito:Authority"];
        options.TokenValidationParameters = new TokenValidationParameters
        {
            ValidateIssuer = true,
            ValidIssuer = configuration["Authentication:Cognito:Authority"],
            ValidateAudience = true,
            ValidAudience = configuration["Authentication:Cognito:AppClientId"],
            ValidateLifetime = true
        };
    });
```

### Rust Services

For Rust services that need to validate tokens:

```rust
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};

// Fetch JWKS from Cognito
let jwks_url = std::env::var("COGNITO_JWKS_URL")?;
let issuer = std::env::var("COGNITO_ISSUER_URL")?;

// Validate token
let mut validation = Validation::new(Algorithm::RS256);
validation.set_issuer(&[issuer]);
validation.set_audience(&[client_id]);
```

## User Groups

Two default groups are created:

| Group | Precedence | Description |
|-------|------------|-------------|
| admin | 1 | Administrators with full access |
| user | 10 | Regular users |

Group membership is included in the `cognito:groups` claim in tokens.

## Custom Attributes

| Attribute | Type | Description |
|-----------|------|-------------|
| workspace_id | String | User's default workspace ID |

## Security Recommendations

### Development
- `mfa_configuration = "OPTIONAL"`
- `advanced_security_mode = "AUDIT"`
- 8-character minimum password

### Production
- `mfa_configuration = "ON"` (required)
- `advanced_security_mode = "ENFORCED"`
- 12-character minimum password with symbols
- Custom domain with ACM certificate
- SES for email delivery

## Migrating from Keycloak

1. Export users from Keycloak
2. Import users to Cognito using AWS CLI or SDK
3. Update application configuration to use Cognito endpoints
4. Update callback URLs in Cognito app client
5. Test authentication flows

User passwords cannot be migrated; users will need to reset passwords or use the "migrate user" Lambda trigger for just-in-time migration.

## Hosted UI

Cognito provides a hosted UI for sign-in/sign-up:

```
https://{domain}.auth.{region}.amazoncognito.com/login?
  client_id={client_id}&
  response_type=code&
  scope=openid+email+profile&
  redirect_uri={callback_url}
```

For custom branding, you can customize the hosted UI in the AWS Console or use a custom domain with your own UI.
