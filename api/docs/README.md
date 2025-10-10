# Shortas Proxy API

A public API proxy for Shortas click router and aggregator services, built with C# .NET 8, minimal API, and clean architecture principles.

## 🚀 Features

### Core Functionality
- **Route Management**: Complete CRUD operations for routing configurations
- **SSL Certificate Management**: Automated certificate handling
- **User Settings**: Comprehensive user preference management
- **Click Stream Analytics**: Analytics and click stream data access
- **Bulk Operations**: Efficient batch processing for multiple resources

### Security & Authentication
- **JWT Authentication**: Secure token-based authentication via Keycloak
- **Role-Based Access Control**: Fine-grained permissions and authorization
- **Rate Limiting**: Built-in protection against abuse
- **Security Headers**: Automatic security header injection
- **CORS Support**: Configurable cross-origin resource sharing

### API Documentation
- **OpenAPI 3.0**: Complete API specification with interactive documentation
- **Swagger UI**: User-friendly API exploration interface
- **Comprehensive Schemas**: Detailed request/response documentation
- **Authentication Examples**: Clear authentication flow documentation

### Architecture
- **Clean Architecture**: Domain, Application, Infrastructure, and Presentation layers
- **Dependency Injection**: Loose coupling and testability
- **Resilience Patterns**: Retry policies and circuit breakers
- **Logging**: Structured logging with Serilog

## 📋 Table of Contents

- [Quick Start](#quick-start)
- [API Endpoints](#api-endpoints)
- [Authentication](#authentication)
- [Configuration](#configuration)
- [Security](#security)
- [Development](#development)
- [Deployment](#deployment)
- [Contributing](#contributing)

## 🚀 Quick Start

### Prerequisites

- .NET 8.0 SDK
- Docker and Docker Compose (optional)
- Keycloak server running on localhost:8080

### Installation

1. **Clone the repository**
   ```bash
   git clone <repository-url>
   cd api
   ```

2. **Restore dependencies**
   ```bash
   dotnet restore
   ```

3. **Configure environment**
   ```bash
   cp appsettings.Development.json appsettings.Local.json
   # Edit configuration as needed
   ```

4. **Run the application**
   ```bash
   dotnet run
   ```

The API will be available at `http://localhost:5050`

## 🔗 API Endpoints

### Routes Management

#### Individual Routes
- **GET** `/api/v1/routes/{domain}/{path}` - Get route information
- **POST** `/api/v1/routes` - Create new route
- **PUT** `/api/v1/routes/{domain}/{path}` - Update existing route
- **DELETE** `/api/v1/routes/{domain}/{path}` - Delete route

#### Bulk Operations
- **POST** `/api/v1/routes/bulk` - Create multiple routes
- **PUT** `/api/v1/routes/bulk` - Update multiple routes
- **DELETE** `/api/v1/routes/bulk` - Delete multiple routes

### SSL Certificate Management

- **GET** `/api/v1/certificates/{domain}` - Get certificate information
- **POST** `/api/v1/certificates/{domain}` - Create new certificate
- **PUT** `/api/v1/certificates/{domain}` - Update existing certificate
- **DELETE** `/api/v1/certificates/{domain}` - Delete certificate

### User Settings Management

- **GET** `/api/v1/user-settings/{userId}` - Get user settings
- **POST** `/api/v1/user-settings/{userId}` - Create user settings
- **PUT** `/api/v1/user-settings/{userId}` - Update user settings
- **DELETE** `/api/v1/user-settings/{userId}` - Delete user settings

### Click Stream Analytics

- **GET** `/api/v1/clickstream` - Get click stream data
- **GET** `/api/v1/clickstream/{routeId}` - Get route-specific analytics
- **GET** `/api/v1/clickstream/stats` - Get aggregated statistics

### Public Endpoints

- **GET** `/api/health` - Health check
- **GET** `/api/health/ready` - Readiness check
- **GET** `/api/health/live` - Liveness check
- **GET** `/swagger` - Interactive API documentation

## 🔐 Authentication

The API uses JWT authentication with Keycloak integration:

### Authentication Methods

#### JWT Bearer Token
```http
Authorization: Bearer eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCJ9...
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

```json
{
  "Keycloak": {
    "Authority": "http://localhost:8080/realms/shortas-dev",
    "Audience": "shortas-api",
    "RequireHttpsMetadata": false
  }
}
```

## ⚙️ Configuration

### Environment Variables

```bash
# API Settings
ApiSettings__ClickRouterApi__BaseUrl=http://localhost:8081
ApiSettings__ClickAggregatorApi__BaseUrl=http://localhost:8082

# JWT Configuration
Keycloak__Authority=http://localhost:8080/realms/shortas-dev
Keycloak__Audience=shortas-api
Keycloak__RequireHttpsMetadata=false

# Server Configuration
ASPNETCORE_URLS=http://+:5050
ASPNETCORE_ENVIRONMENT=Production
```

### Configuration Files

#### `appsettings.json`
```json
{
  "ApiSettings": {
    "ClickRouterApi": {
      "BaseUrl": "http://localhost:8081",
      "Timeout": 30
    },
    "ClickAggregatorApi": {
      "BaseUrl": "http://localhost:8082",
      "Timeout": 30
    }
  },
  "Keycloak": {
    "Authority": "http://localhost:8080/realms/shortas-dev",
    "Audience": "shortas-api",
    "RequireHttpsMetadata": false
  },
  "RateLimiting": {
    "RequestsPerMinute": 100,
    "BurstLimit": 20
  }
}
```

## 🔒 Security

### Security Features

- **JWT Authentication**: Secure token-based authentication
- **Role-Based Access Control**: Fine-grained permissions
- **Rate Limiting**: Protection against abuse (100 requests/minute, 20 burst)
- **Security Headers**: Automatic security header injection
- **CORS Support**: Configurable cross-origin resource sharing
- **Resilience Patterns**: Retry policies and circuit breakers

### Security Headers

The API automatically includes security headers:

```http
X-Content-Type-Options: nosniff
X-Frame-Options: DENY
X-XSS-Protection: 1; mode=block
Strict-Transport-Security: max-age=31536000; includeSubDomains
Content-Security-Policy: default-src 'self'
Referrer-Policy: strict-origin-when-cross-origin
Permissions-Policy: geolocation=(), microphone=(), camera=()
```

## 🛠️ Development

### Project Structure

```
src/
├── Domain/              # Domain entities and interfaces
├── Application/         # Application services and DTOs
├── Infrastructure/      # External adapters and infrastructure
├── Presentation/        # Controllers and API endpoints
└── Program.cs          # Application entry point
```

### Building

```bash
# Development build
dotnet build

# Release build
dotnet build -c Release

# Run tests
dotnet test

# Run with logging
dotnet run --environment Development
```

### Code Quality

```bash
# Format code
dotnet format

# Analyze code
dotnet analyze

# Check for security issues
dotnet list package --vulnerable
```

## 🚀 Deployment

### Docker Deployment

1. **Build Docker Image**
   ```bash
   docker build -t shortas-api .
   ```

2. **Run Container**
   ```bash
   docker run -p 5050:5050 \
     -e ApiSettings__ClickRouterApi__BaseUrl=http://click-router-api:8080 \
     -e ApiSettings__ClickAggregatorApi__BaseUrl=http://click-aggregator-api:8080 \
     -e Keycloak__Authority=http://keycloak:8080/realms/shortas \
     shortas-api
   ```

### Docker Compose

```bash
docker-compose up -d
```

### Production Deployment

1. **Environment Setup**
   ```bash
   export ApiSettings__ClickRouterApi__BaseUrl="https://api.router.shortas.com"
   export ApiSettings__ClickAggregatorApi__BaseUrl="https://api.analytics.shortas.com"
   export Keycloak__Authority="https://auth.shortas.com/realms/shortas"
   export ASPNETCORE_ENVIRONMENT="Production"
   ```

2. **Run Application**
   ```bash
   dotnet ShortasProxyApi.dll
   ```

3. **Reverse Proxy (Nginx)**
   ```nginx
   server {
       listen 80;
       server_name api.shortas.com;
       
       location / {
           proxy_pass http://localhost:5050;
           proxy_set_header Host $host;
           proxy_set_header X-Real-IP $remote_addr;
           proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
           proxy_set_header X-Forwarded-Proto $scheme;
       }
   }
   ```

## 📚 API Documentation

### Interactive Documentation

- **Swagger UI**: `http://localhost:5050/swagger`
- **OpenAPI Spec**: `http://localhost:5050/swagger/v1/swagger.json`

### API Information

- **Title**: Shortas Proxy API
- **Version**: 1.0.0
- **Description**: Public API proxy for Shortas services
- **License**: MIT
- **Contact**: API Support (support@shortas.com)

### Authentication in Documentation

The Swagger UI supports JWT authentication:

1. Click "Authorize" button
2. Enter JWT token: `Bearer your_jwt_token`
3. All authenticated endpoints will use the token

## 🚨 Error Handling

### Error Response Format

```json
{
  "type": "https://tools.ietf.org/html/rfc7231#section-6.5.1",
  "title": "One or more validation errors occurred.",
  "status": 400,
  "traceId": "0HMQ8VJJA7U2P:00000001",
  "errors": {
    "field": ["The field is required."]
  }
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

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

### Development Guidelines

- Follow C# naming conventions
- Add tests for new features
- Update documentation
- Ensure all tests pass
- Follow security best practices

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🆘 Support

- **Documentation**: Check the comprehensive documentation files
- **Issues**: Report bugs and feature requests via GitHub Issues
- **Security**: Report security issues privately to security@shortas.com

## 🏗️ Architecture

### Clean Architecture

The application follows clean architecture principles:

- **Domain**: Business logic and domain models
- **Application**: Application services and DTOs
- **Infrastructure**: External interfaces (HTTP, databases)
- **Presentation**: Controllers and API endpoints

### Key Components

- **API Layer**: HTTP endpoints and request handling
- **Business Logic**: Core domain logic
- **Security Layer**: Authentication and authorization
- **Resilience Layer**: Retry policies and circuit breakers
- **Documentation Layer**: OpenAPI/Swagger integration

---

**Built with ❤️ using C# .NET 8 and modern web technologies**
