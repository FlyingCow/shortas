# DTO Usage Guide

This document explains how to use the Data Transfer Objects (DTOs) in the Click Router API.

## KeycertDto

The `KeycertDto` provides a more API-friendly representation of SSL certificates using PEM encoded strings instead of raw byte vectors.

### Structure

```rust
pub struct KeycertDto {
    /// Private key in PEM format
    pub key: String,
    
    /// Certificate in PEM format
    pub cert: String,
    
    /// OCSP response in PEM format
    pub ocsp_resp: String,
}
```

### Usage Examples

#### Creating a KeycertDto

```rust
use crate::dto::KeycertDto;

// Create with empty data
let keycert = KeycertDto::new();

// Create with specific PEM data
let keycert = KeycertDto::with_pem_data(
    "-----BEGIN PRIVATE KEY-----\n...\n-----END PRIVATE KEY-----",
    "-----BEGIN CERTIFICATE-----\n...\n-----END CERTIFICATE-----",
    "-----BEGIN OCSP RESPONSE-----\n...\n-----END OCSP RESPONSE-----"
);

// Builder pattern
let keycert = KeycertDto::new()
    .key("-----BEGIN PRIVATE KEY-----\n...\n-----END PRIVATE KEY-----")
    .cert("-----BEGIN CERTIFICATE-----\n...\n-----END CERTIFICATE-----")
    .ocsp_resp("-----BEGIN OCSP RESPONSE-----\n...\n-----END OCSP RESPONSE-----");
```

#### Converting from Internal Keycert

```rust
use crate::model::keycert::Keycert;
use crate::dto::KeycertDto;

// From owned Keycert
let internal_keycert = Keycert::new();
let dto = KeycertDto::from(internal_keycert);

// From reference to Keycert
let internal_keycert = Keycert::new();
let dto = KeycertDto::from(&internal_keycert);
```

#### Converting to Internal Keycert

```rust
use crate::dto::KeycertDto;
use crate::model::keycert::Keycert;

let dto = KeycertDto::new();
let internal_keycert = Keycert::from(dto);
```

#### Validation Methods

```rust
use crate::dto::KeycertDto;

let keycert = KeycertDto::new();

// Check if all fields are non-empty
if keycert.is_valid() {
    println!("Keycert has all required data");
}

// Check individual components
if keycert.has_valid_key() {
    println!("Private key is valid PEM format");
}

if keycert.has_valid_cert() {
    println!("Certificate is valid PEM format");
}

if keycert.has_valid_ocsp() {
    println!("OCSP response is valid PEM format");
}
```

### API Response Example

When calling the `/certificates` endpoint, the response will now use the KeycertDto format:

```json
{
  "key": "-----BEGIN PRIVATE KEY-----\nMIIEvgIBADANBgkqhkiG9w0BAQEFAASCBKgwggSkAgEAAoIBAQC...\n-----END PRIVATE KEY-----",
  "cert": "-----BEGIN CERTIFICATE-----\nMIIDXTCCAkWgAwIBAgIJAKoK/OvD8WqKMA0GCSqGSIb3DQEBCwUAMEUxCzAJBgNV...\n-----END CERTIFICATE-----",
  "ocsp_resp": "-----BEGIN OCSP RESPONSE-----\nMIIBpAoBAKCCAZkwggGVBgkrBgEFBQcwAQEEggGGMIIBgjCBg6EWMBQxEjAQBgNV...\n-----END OCSP RESPONSE-----"
}
```

### Benefits

1. **Human Readable**: PEM format is easier to read and debug
2. **API Friendly**: String format is more suitable for JSON serialization
3. **Validation**: Built-in methods to validate PEM format
4. **Conversion**: Easy conversion to/from internal byte vector format
5. **Type Safety**: Strong typing with helpful methods

### File Structure

```
src/
├── dto/
│   ├── mod.rs
│   └── keycert_dto.rs
├── model/
│   └── keycert.rs (original internal format)
└── ...
```

The DTOs are kept separate from the internal models to maintain clear separation of concerns between internal data structures and API representations.
