---
layout: page
title: Click Router API
permalink: /api/click-router/
---

# Click Router API

A high-performance, secure click aggregation API built with Rust, featuring JWT authentication via Keycloak, comprehensive OpenAPI documentation, and support for multiple database backends.

## 🚀 Features

### Core Functionality
- **Route Management**: Complete CRUD operations for routing configurations
- **SSL Certificate Management**: Automated certificate handling with PEM encoding
- **User Settings**: Comprehensive user preference management
- **Bulk Operations**: Efficient batch processing for multiple resources

### Security & Authentication
- **JWT Authentication**: Secure token-based authentication via Keycloak
- **Role-Based Access Control**: Fine-grained permissions and authorization
- **Rate Limiting**: Built-in protection against abuse
- **Input Validation**: Comprehensive request validation and sanitization
- **Security Headers**: Automatic security header injection

### API Documentation
- **OpenAPI 3.0**: Complete API specification with interactive documentation
- **Swagger UI**: User-friendly API exploration interface
- **Comprehensive Schemas**: Detailed request/response documentation
- **Authentication Examples**: Clear authentication flow documentation

### Database Support
- **MongoDB**: High-performance document storage
- **DynamoDB**: Scalable NoSQL database (AWS)
- **Flexible Architecture**: Easy to add additional database backends

## 🔗 API Endpoints

### Routes Management

#### Individual Routes
- **GET** `/v1/routes/{switch}/{domain}/{path}` - Get route information
- **POST** `/v1/routes/{switch}/{domain}/{path}` - Create new route
- **PUT** `/v1/routes/{switch}/{domain}/{path}` - Update existing route
- **DELETE** `/v1/routes/{switch}/{domain}/{path}` - Delete route

#### Bulk Operations
- **POST** `/v1/routes/bulk` - Create multiple routes
- **PUT** `/v1/routes/bulk` - Update multiple routes
- **DELETE** `/v1/routes/bulk` - Delete multiple routes

### SSL Certificate Management

- **GET** `/v1/certificates/{domain}` - Get certificate information
- **POST** `/v1/certificates/{domain}` - Create new certificate
- **PUT** `/v1/certificates/{domain}` - Update existing certificate
- **DELETE** `/v1/certificates/{domain}` - Delete certificate

### User Settings Management

- **GET** `/v1/user-settings/{user_id}` - Get user settings
- **POST** `/v1/user-settings/{user_id}` - Create user settings
- **PUT** `/v1/user-settings/{user_id}` - Update user settings
- **DELETE** `/v1/user-settings/{user_id}` - Delete user settings

### Public Endpoints

- **GET** `/health` - Health check
- **GET** `/swagger-ui` - Interactive API documentation
- **GET** `/api-doc/openapi.json` - OpenAPI specification

## 🔐 Authentication

The API uses JWT authentication with Keycloak integration:

### Authentication Methods

#### JWT Bearer Token
```http
Authorization: Bearer eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCJ9...
```

#### RPT Token (Fine-grained Authorization)
```http
Authorization: Bearer rpt_token_for_uma_authorization
```

### Required Permissions

| Resource | Permission | Description |
|----------|------------|-------------|
| Routes | `read:routes` | Read route information |
| Routes | `write:routes` | Create/update routes |
| Routes | `delete:routes` | Delete routes |
| Certificates | `read:certificates` | Read certificate information |
| Certificates | `write:certificates` | Create/update certificates |
| Certificates | `delete:certificates` | Delete certificates |
| User Settings | `read:user_settings` | Read user settings |
| User Settings | `write:user_settings` | Update user settings |

### Keycloak Configuration

```toml
[jwt]
keycloak_url = "http://keycloak:8080"
realm = "click-router"
client_id = "click-router-api"
jwks_url = "http://keycloak:8080/realms/click-aggregator/protocol/openid-connect/certs"
```

## 📊 Data Transfer Objects (DTOs)

The API uses DTOs for clean, type-safe data exchange:

### RouteDto
```json
{
  "switch": "main",
  "link": "https://example.com",
  "dest": "https://target.com",
  "dest_format": "Http",
  "code": 301,
  "ttl": 3600,
  "status": "Active",
  "terminal": "External",
  "properties": {
    "route_id": "route-123",
    "domain_id": "domain-456",
    "owner_id": "user-789",
    "scripts": ["script1.js"],
    "tags": ["api"],
    "custom": {"key": "value"},
    "opengraph": true,
    "allow_debug": false
  }
}
```

### KeycertDto
```json
{
  "key": "-----BEGIN PRIVATE KEY-----\n...\n-----END PRIVATE KEY-----",
  "cert": "-----BEGIN CERTIFICATE-----\n...\n-----END CERTIFICATE-----",
  "ocsp_resp": "-----BEGIN OCSP RESPONSE-----\n...\n-----END OCSP RESPONSE-----"
}
```

### UserSettingsDto
```json
{
  "email": "user@example.com",
  "status": "active",
  "debug": false,
  "overflow": true,
  "skip_tracking": ["tracking", "analytics"],
  "allowed_request_params": ["utm_source", "utm_medium"],
  "allowed_destination_params": ["redirect", "target"]
}
```

## ⚙️ Configuration

### Environment Variables

```bash
# Database Configuration
DATABASE_URL=mongodb://localhost:27017/click_router
# or for DynamoDB
AWS_REGION=us-east-1
AWS_ACCESS_KEY_ID=your_access_key
AWS_SECRET_ACCESS_KEY=your_secret_key

# JWT Configuration
KEYCLOAK_URL=http://keycloak:8080
KEYCLOAK_REALM=click-router
KEYCLOAK_CLIENT_ID=click-router-api

# Server Configuration
SERVER_HOST=0.0.0.0
SERVER_PORT=8080
LOG_LEVEL=info
```

### Configuration Files

#### `config/development.toml`
```toml
[server]
host = "0.0.0.0"
port = 8080

[database]
url = "mongodb://localhost:27017/click_router_dev"

[jwt]
keycloak_url = "http://localhost:8080"
realm = "click-router"
client_id = "click-router-api"
```

#### `config/production.toml`
```toml
[server]
host = "0.0.0.0"
port = 8080

[database]
url = "mongodb://mongodb:27017/click_router"

[jwt]
keycloak_url = "https://keycloak.example.com"
realm = "click-router"
client_id = "click-router-api"
```

## 🗄️ Database Setup

### MongoDB Setup

1. **Install MongoDB**
   ```bash
   # Ubuntu/Debian
   sudo apt-get install mongodb
   
   # macOS
   brew install mongodb-community
   ```

2. **Start MongoDB**
   ```bash
   sudo systemctl start mongod
   # or
   mongod --dbpath /data/db
   ```

3. **Create Database**
   ```bash
   mongo
   > use click_router
   > db.createUser({user: "api_user", pwd: "password", roles: ["readWrite"]})
   ```

### DynamoDB Setup (AWS)

1. **Create DynamoDB Tables**
   ```bash
   aws dynamodb create-table \
     --table-name routes \
     --attribute-definitions \
       AttributeName=switch,AttributeType=S \
       AttributeName=domain,AttributeType=S \
       AttributeName=path,AttributeType=S \
     --key-schema \
       AttributeName=switch,KeyType=HASH \
       AttributeName=domain,KeyType=RANGE \
     --billing-mode PAY_PER_REQUEST
   ```

2. **Configure AWS Credentials**
   ```bash
   aws configure
   ```

## 🔒 Security

### Security Features

- **JWT Authentication**: Secure token-based authentication
- **Role-Based Access Control**: Fine-grained permissions
- **Rate Limiting**: Protection against abuse
- **Input Validation**: Comprehensive request validation
- **Security Headers**: Automatic security header injection
- **CORS Support**: Configurable cross-origin resource sharing

### Security Headers

The API automatically includes security headers:

```http
X-Content-Type-Options: nosniff
X-Frame-Options: DENY
X-XSS-Protection: 1; mode=block
Strict-Transport-Security: max-age=31536000; includeSubDomains
Content-Security-Policy: default-src 'self'
```

### Rate Limiting

- **Default**: 100 requests per minute per IP
- **Configurable**: Per-endpoint rate limits
- **Burst Protection**: Temporary blocking for excessive requests

## 📚 OpenAPI Documentation

### Interactive Documentation

- **Swagger UI**: `http://localhost:8080/swagger-ui`
- **OpenAPI Spec**: `http://localhost:8080/api-doc/openapi.json`

### API Information

- **Title**: Click Router API
- **Version**: 0.1.0
- **Description**: High-performance click aggregation API
- **License**: MIT
- **Contact**: API Support (support@example.com)

### Authentication in Documentation

The Swagger UI supports JWT authentication:

1. Click "Authorize" button
2. Enter JWT token: `Bearer your_jwt_token`
3. All authenticated endpoints will use the token

## 🚨 Error Handling

### Error Response Format

```json
{
  "status_code": 400,
  "error": "Validation Error",
  "details": "Invalid input data: field 'email' is required"
}
```

### HTTP Status Codes

| Code | Description |
|------|-------------|
| 200 | Success |
| 201 | Created |
| 400 | Bad Request |
| 401 | Unauthorized |
| 403 | Forbidden |
| 404 | Not Found |
| 409 | Conflict |
| 422 | Unprocessable Entity |
| 429 | Too Many Requests |
| 500 | Internal Server Error |

### Error Types

- **ValidationError**: Input validation failures
- **AuthenticationError**: Authentication failures
- **AuthorizationError**: Permission denied
- **NotFoundError**: Resource not found
- **ConflictError**: Resource conflicts
- **DatabaseError**: Database operation failures
- **NetworkError**: Network communication failures

## 🛠️ Development

### Project Structure

```
src/
├── adapters/           # External adapters
│   ├── api/           # HTTP API layer
│   ├── mongodb/       # MongoDB adapter
│   └── aws/            # AWS/DynamoDB adapter
├── core/              # Core business logic
├── dto/               # Data Transfer Objects
├── model/             # Domain models
└── main.rs            # Application entry point
```

### Building

```bash
# Development build
cargo build

# Release build
cargo build --release

# Run tests
cargo test

# Run with logging
RUST_LOG=debug cargo run
```

### Code Quality

```bash
# Format code
cargo fmt

# Lint code
cargo clippy

# Check for security issues
cargo audit
```

### Testing

```bash
# Run all tests
cargo test

# Run integration tests
cargo test --test integration

# Run with coverage
cargo tarpaulin --out html
```

## 🚀 Deployment

### Docker Deployment

1. **Build Docker Image**
   ```bash
   docker build -t click-router-api .
   ```

2. **Run Container**
   ```bash
   docker run -p 8080:8080 \
     -e DATABASE_URL=mongodb://mongodb:27017/click_router \
     -e KEYCLOAK_URL=http://keycloak:8080 \
     click-router-api
   ```

### Docker Compose

```yaml
version: '3.8'
services:
  api:
    build: .
    ports:
      - "8080:8080"
    environment:
      - DATABASE_URL=mongodb://mongodb:27017/click_router
      - KEYCLOAK_URL=http://keycloak:8080
    depends_on:
      - mongodb
      - keycloak

  mongodb:
    image: mongo:latest
    ports:
      - "27017:27017"
    volumes:
      - mongodb_data:/data/db

  keycloak:
    image: quay.io/keycloak/keycloak:latest
    ports:
      - "8080:8080"
    environment:
      - KEYCLOAK_ADMIN=admin
      - KEYCLOAK_ADMIN_PASSWORD=admin
    command: start-dev

volumes:
  mongodb_data:
```

### Production Deployment

1. **Environment Setup**
   ```bash
   export DATABASE_URL="mongodb://production-db:27017/click_router"
   export KEYCLOAK_URL="https://keycloak.example.com"
   export LOG_LEVEL="info"
   ```

2. **Run Application**
   ```bash
   ./target/release/click-router-api
   ```

3. **Reverse Proxy (Nginx)**
   ```nginx
   server {
       listen 80;
       server_name api.example.com;
       
       location / {
           proxy_pass http://localhost:8080;
           proxy_set_header Host $host;
           proxy_set_header X-Real-IP $remote_addr;
       }
   }
   ```

## 📖 Additional Documentation

All detailed documentation is organized in the `docs/` directory:

### Core Documentation
- [Documentation Index](docs/README.md) - Complete documentation overview
- [Security Implementation](docs/SECURITY.md) - Comprehensive security guide
- [Error Handling](docs/ERROR_HANDLING.md) - Error handling patterns

### API Documentation
- [OpenAPI Documentation](docs/OPENAPI_DOCUMENTATION.md) - API documentation details
- [OpenAPI Parameters](docs/OPENAPI_PARAMETERS.md) - Detailed parameter specifications
- [Crypto Endpoints](docs/CRYPTO_ENDPOINTS.md) - SSL certificate management
- [User Settings Endpoints](docs/USER_SETTINGS_ENDPOINTS.md) - User settings management

### Data Transfer Objects
- [DTO Usage](docs/DTO_USAGE.md) - General DTO patterns
- [Route DTO](docs/ROUTE_DTO.md) - Route data transfer object documentation
- [User Settings DTO](docs/USER_SETTINGS_DTO.md) - User settings DTO documentation

### Authentication & Security
- [JWT Authentication](docs/JWT_AUTHENTICATION.md) - JWT authentication implementation
- [User ID Parameter](docs/USER_ID_PARAMETER.md) - User ID parameter specifications

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

### Development Guidelines

- Follow Rust naming conventions
- Add tests for new features
- Update documentation
- Ensure all tests pass
- Follow security best practices

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🆘 Support

- **Documentation**: Check the comprehensive documentation files
- **Issues**: Report bugs and feature requests via GitHub Issues
- **Security**: Report security issues privately to security@example.com

## 🏗️ Architecture

### Hexagonal Architecture

The application follows hexagonal architecture principles:

- **Core**: Business logic and domain models
- **Adapters**: External interfaces (HTTP, databases)
- **Ports**: Interfaces between core and adapters

### Key Components

- **API Layer**: HTTP endpoints and request handling
- **Business Logic**: Core domain logic
- **Data Layer**: Database operations and persistence
- **Security Layer**: Authentication and authorization
- **Documentation Layer**: OpenAPI/Swagger integration

---

**Built with ❤️ using Rust and modern web technologies**
