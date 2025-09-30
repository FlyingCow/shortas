# Crypto Controller Endpoints

This document describes the SSL certificate management endpoints in the Click Router API.

## Overview

The crypto controller provides full CRUD operations for SSL certificates, allowing you to create, read, update, and delete SSL certificate information for domains.

## Route Structure

The crypto controller uses a nested route structure where the domain is a path parameter:

```
/v1/certificates/{domain}
```

This structure allows for:
- Clear resource identification by domain
- RESTful URL patterns
- Easy parameter extraction
- Consistent API design

## Endpoints

All endpoints use the domain as a path parameter in the URL structure: `/v1/certificates/{domain}`

### 1. Get Certificate

**GET** `/v1/certificates/{domain}`

Retrieves SSL certificate information for a specific domain.

#### Parameters
- `domain` (path, required): The domain name for the certificate
  - **Type**: String
  - **Format**: Valid domain name (RFC 1123)
  - **Examples**: `example.com`, `api.example.com`, `subdomain.example.org`
  - **Validation**: 
    - Must be non-empty
    - Must be a valid domain name format
    - Supports subdomains and internationalized domain names
    - Case-insensitive (will be normalized to lowercase)
  - **Constraints**:
    - Maximum length: 253 characters
    - Must contain at least one dot (.)
    - Cannot start or end with a dot
    - Cannot contain consecutive dots
    - Must not contain spaces or special characters except hyphens

#### Response
- **200 OK**: Certificate found
  ```json
  {
    "key": "-----BEGIN PRIVATE KEY-----\nMIIEvgIBADANBgkqhkiG9w0BAQEFAASCBKgwggSkAgEAAoIBAQC...\n-----END PRIVATE KEY-----",
    "cert": "-----BEGIN CERTIFICATE-----\nMIIDXTCCAkWgAwIBAgIJAKoK/OvD8WqKMA0GCSqGSIb3DQEBCwUAMEUxCzAJBgNV...\n-----END CERTIFICATE-----",
    "ocsp_resp": "-----BEGIN OCSP RESPONSE-----\nMIIBpAoBAKCCAZkwggGVBgkrBgEFBQcwAQEEggGGMIIBgjCBg6EWMBQxEjAQBgNV...\n-----END OCSP RESPONSE-----"
  }
  ```

- **404 Not Found**: Certificate not found
- **401 Unauthorized**: Invalid or missing JWT token
- **403 Forbidden**: Insufficient permissions
- **500 Internal Server Error**: Server error

### 2. Create Certificate

**POST** `/v1/certificates/{domain}`

Creates a new SSL certificate entry for the specified domain.

#### Parameters
- `domain` (path, required): The domain name for the certificate
  - **Type**: String
  - **Format**: Valid domain name (RFC 1123)
  - **Examples**: `example.com`, `api.example.com`, `subdomain.example.org`
  - **Validation**: 
    - Must be non-empty
    - Must be a valid domain name format
    - Supports subdomains and internationalized domain names
    - Case-insensitive (will be normalized to lowercase)
  - **Constraints**:
    - Maximum length: 253 characters
    - Must contain at least one dot (.)
    - Cannot start or end with a dot
    - Cannot contain consecutive dots
    - Must not contain spaces or special characters except hyphens

#### Request Body
```json
{
  "key": "-----BEGIN PRIVATE KEY-----\nMIIEvgIBADANBgkqhkiG9w0BAQEFAASCBKgwggSkAgEAAoIBAQC...\n-----END PRIVATE KEY-----",
  "cert": "-----BEGIN CERTIFICATE-----\nMIIDXTCCAkWgAwIBAgIJAKoK/OvD8WqKMA0GCSqGSIb3DQEBCwUAMEUxCzAJBgNV...\n-----END CERTIFICATE-----",
  "ocsp_resp": "-----BEGIN OCSP RESPONSE-----\nMIIBpAoBAKCCAZkwggGVBgkrBgEFBQcwAQEEggGGMIIBgjCBg6EWMBQxEjAQBgNV...\n-----END OCSP RESPONSE-----"
}
```

#### Response
- **201 Created**: Certificate created successfully
  ```json
  {
    "message": "Certificate created successfully",
    "domain": "example.com"
  }
  ```

- **400 Bad Request**: Invalid input data
- **401 Unauthorized**: Invalid or missing JWT token
- **403 Forbidden**: Insufficient permissions
- **409 Conflict**: Certificate already exists
- **500 Internal Server Error**: Server error

### 3. Update Certificate

**PUT** `/v1/certificates/{domain}`

Updates the SSL certificate information for the specified domain.

#### Parameters
- `domain` (path, required): The domain name for the certificate
  - **Type**: String
  - **Format**: Valid domain name (RFC 1123)
  - **Examples**: `example.com`, `api.example.com`, `subdomain.example.org`
  - **Validation**: 
    - Must be non-empty
    - Must be a valid domain name format
    - Supports subdomains and internationalized domain names
    - Case-insensitive (will be normalized to lowercase)
  - **Constraints**:
    - Maximum length: 253 characters
    - Must contain at least one dot (.)
    - Cannot start or end with a dot
    - Cannot contain consecutive dots
    - Must not contain spaces or special characters except hyphens

#### Request Body
```json
{
  "key": "-----BEGIN PRIVATE KEY-----\nMIIEvgIBADANBgkqhkiG9w0BAQEFAASCBKgwggSkAgEAAoIBAQC...\n-----END PRIVATE KEY-----",
  "cert": "-----BEGIN CERTIFICATE-----\nMIIDXTCCAkWgAwIBAgIJAKoK/OvD8WqKMA0GCSqGSIb3DQEBCwUAMEUxCzAJBgNV...\n-----END CERTIFICATE-----",
  "ocsp_resp": "-----BEGIN OCSP RESPONSE-----\nMIIBpAoBAKCCAZkwggGVBgkrBgEFBQcwAQEEggGGMIIBgjCBg6EWMBQxEjAQBgNV...\n-----END OCSP RESPONSE-----"
}
```

#### Response
- **200 OK**: Certificate updated successfully
  ```json
  {
    "message": "Certificate updated successfully",
    "domain": "example.com"
  }
  ```

- **400 Bad Request**: Invalid input data
- **401 Unauthorized**: Invalid or missing JWT token
- **403 Forbidden**: Insufficient permissions
- **404 Not Found**: Certificate not found
- **500 Internal Server Error**: Server error

### 4. Delete Certificate

**DELETE** `/v1/certificates/{domain}`

Deletes the SSL certificate information for the specified domain.

#### Parameters
- `domain` (path, required): The domain name for the certificate
  - **Type**: String
  - **Format**: Valid domain name (RFC 1123)
  - **Examples**: `example.com`, `api.example.com`, `subdomain.example.org`
  - **Validation**: 
    - Must be non-empty
    - Must be a valid domain name format
    - Supports subdomains and internationalized domain names
    - Case-insensitive (will be normalized to lowercase)
  - **Constraints**:
    - Maximum length: 253 characters
    - Must contain at least one dot (.)
    - Cannot start or end with a dot
    - Cannot contain consecutive dots
    - Must not contain spaces or special characters except hyphens

#### Response
- **200 OK**: Certificate deleted successfully
  ```json
  {
    "message": "Certificate deleted successfully",
    "domain": "example.com"
  }
  ```

- **400 Bad Request**: Invalid domain parameter
- **401 Unauthorized**: Invalid or missing JWT token
- **403 Forbidden**: Insufficient permissions
- **404 Not Found**: Certificate not found
- **500 Internal Server Error**: Server error

## Domain Parameter Details

### Valid Domain Examples

The `domain` parameter accepts various types of domain names:

#### Standard Domains
- `example.com`
- `api.example.com`
- `www.example.com`
- `subdomain.example.org`

#### Internationalized Domain Names (IDN)
- `例え.jp` (Japanese)
- `пример.рф` (Russian)
- `münchen.de` (German)

#### Special Cases
- `localhost` (for development)
- `*.example.com` (wildcard domains)
- `example.co.uk` (multi-level TLD)

### Domain Validation Rules

The API enforces the following validation rules for domain parameters:

1. **Format Requirements**:
   - Must be a valid domain name according to RFC 1123
   - Must contain at least one dot (.) for standard domains
   - Cannot start or end with a dot
   - Cannot contain consecutive dots

2. **Length Constraints**:
   - Maximum length: 253 characters
   - Minimum length: 1 character (for localhost)

3. **Character Restrictions**:
   - Must not contain spaces
   - Must not contain special characters except hyphens
   - Must not contain consecutive hyphens
   - Must not start or end with hyphens

4. **Case Handling**:
   - Domain names are case-insensitive
   - API will normalize to lowercase for storage
   - Input validation is case-insensitive

### Common Validation Errors

| Invalid Domain | Error Reason |
|----------------|--------------|
| `example` | Missing TLD (no dot) |
| `.example.com` | Starts with dot |
| `example.com.` | Ends with dot |
| `example..com` | Consecutive dots |
| `example com` | Contains space |
| `example@com` | Contains invalid character |
| `-example.com` | Starts with hyphen |
| `example-.com` | Ends with hyphen |

### Domain Normalization

The API performs the following normalization on domain parameters:

1. **Case Normalization**: Converts to lowercase
2. **Whitespace Trimming**: Removes leading/trailing whitespace
3. **Encoding**: Handles internationalized domain names properly

## Authentication

All endpoints require JWT authentication. Include the JWT token in the Authorization header:

```
Authorization: Bearer <your-jwt-token>
```

## Data Format

All certificate data is provided in PEM format as strings:

- **Private Key**: PEM encoded private key (RSA, ECDSA, etc.)
- **Certificate**: PEM encoded X.509 certificate
- **OCSP Response**: PEM encoded OCSP (Online Certificate Status Protocol) response

## Validation

The API validates:

1. **Domain Parameter**: Must be non-empty
2. **Certificate Data**: All fields (key, cert, ocsp_resp) must be non-empty
3. **PEM Format**: Basic validation of PEM format structure
4. **JSON Format**: Valid JSON in request body

## Error Handling

The API uses structured error responses:

```json
{
  "status_code": 400,
  "error": "Validation error: Missing field 'domain'",
  "details": "ValidationError::MissingField(\"domain\")"
}
```

## Usage Examples

### Create a Certificate

```bash
# Standard domain
curl -X POST "https://api.example.com/v1/certificates/example.com" \
  -H "Authorization: Bearer <your-jwt-token>" \
  -H "Content-Type: application/json" \
  -d '{
    "key": "-----BEGIN PRIVATE KEY-----\n...\n-----END PRIVATE KEY-----",
    "cert": "-----BEGIN CERTIFICATE-----\n...\n-----END CERTIFICATE-----",
    "ocsp_resp": "-----BEGIN OCSP RESPONSE-----\n...\n-----END OCSP RESPONSE-----"
  }'

# Subdomain
curl -X POST "https://api.example.com/v1/certificates/api.example.com" \
  -H "Authorization: Bearer <your-jwt-token>" \
  -H "Content-Type: application/json" \
  -d '{...}'

# Internationalized domain
curl -X POST "https://api.example.com/v1/certificates/例え.jp" \
  -H "Authorization: Bearer <your-jwt-token>" \
  -H "Content-Type: application/json" \
  -d '{...}'
```

### Get a Certificate

```bash
# Standard domain
curl -X GET "https://api.example.com/v1/certificates/example.com" \
  -H "Authorization: Bearer <your-jwt-token>"

# Subdomain
curl -X GET "https://api.example.com/v1/certificates/api.example.com" \
  -H "Authorization: Bearer <your-jwt-token>"

# Case-insensitive (will be normalized)
curl -X GET "https://api.example.com/v1/certificates/EXAMPLE.COM" \
  -H "Authorization: Bearer <your-jwt-token>"
```

### Update a Certificate

```bash
# Standard domain
curl -X PUT "https://api.example.com/v1/certificates/example.com" \
  -H "Authorization: Bearer <your-jwt-token>" \
  -H "Content-Type: application/json" \
  -d '{
    "key": "-----BEGIN PRIVATE KEY-----\n...\n-----END PRIVATE KEY-----",
    "cert": "-----BEGIN CERTIFICATE-----\n...\n-----END CERTIFICATE-----",
    "ocsp_resp": "-----BEGIN OCSP RESPONSE-----\n...\n-----END OCSP RESPONSE-----"
  }'

# Multi-level TLD
curl -X PUT "https://api.example.com/v1/certificates/example.co.uk" \
  -H "Authorization: Bearer <your-jwt-token>" \
  -H "Content-Type: application/json" \
  -d '{...}'
```

### Delete a Certificate

```bash
# Standard domain
curl -X DELETE "https://api.example.com/v1/certificates/example.com" \
  -H "Authorization: Bearer <your-jwt-token>"

# Subdomain
curl -X DELETE "https://api.example.com/v1/certificates/api.example.com" \
  -H "Authorization: Bearer <your-jwt-token>"

# Development localhost
curl -X DELETE "https://api.example.com/v1/certificates/localhost" \
  -H "Authorization: Bearer <your-jwt-token>"
```

## Security Considerations

1. **JWT Authentication**: All endpoints require valid JWT tokens
2. **Permission-Based Access**: Users must have appropriate permissions
3. **Input Validation**: All input data is validated before processing
4. **Error Handling**: Sensitive information is not exposed in error messages
5. **Rate Limiting**: Endpoints are subject to rate limiting policies

## Implementation Notes

- The API uses the `KeycertDto` for request/response bodies
- Internal storage uses the `Keycert` model with byte vectors
- Automatic conversion between DTO and internal formats
- Comprehensive error handling with structured error responses
- Full CRUD operations supported through the `CryptoStore` trait
