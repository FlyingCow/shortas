# Route Form Guide

This guide explains the comprehensive route creation and editing form that now includes all properties from the Routes API DTOs.

## 📋 Form Sections

### 1. Basic Settings

**Switch**
- **Purpose**: Route switch identifier
- **Options**: Main, Secondary, Backup
- **Default**: Main
- **Description**: Determines which routing logic to use

**Short URL Path**
- **Purpose**: The path part of the short URL
- **Format**: Alphanumeric with hyphens (e.g., `my-short-url`)
- **Required**: Yes
- **Example**: `my-short-url` → accessible at `/my-short-url`
- **Validation**: Must be unique within the domain

**Destination URL**
- **Purpose**: The full URL where users will be redirected
- **Format**: Complete URL with protocol
- **Required**: Yes
- **Example**: `https://example.com/target-page`
- **Validation**: Must be a valid URL

**Redirect Code**
- **Purpose**: HTTP status code for the redirect
- **Options**:
  - `301` - Permanent Redirect (SEO friendly)
  - `302` - Temporary Redirect
  - `307` - Temporary Redirect (Preserve Method)
  - `308` - Permanent Redirect (Preserve Method)
- **Default**: 301
- **Description**: Tells browsers and search engines how to handle the redirect

**TTL (Time To Live)**
- **Purpose**: How long the redirect should be cached
- **Range**: 60-86400 seconds (1 minute to 24 hours)
- **Default**: 3600 seconds (1 hour)
- **Description**: Controls caching behavior for the redirect

**Status**
- **Purpose**: Route status
- **Options**: Active, Inactive, Paused
- **Default**: Active
- **Description**: Controls whether the route is functional

**Destination Format**
- **Purpose**: Protocol for the destination
- **Options**: HTTP, HTTPS, SFTP, FTP
- **Default**: HTTP
- **Description**: Specifies the protocol for the target URL

**Terminal**
- **Purpose**: Route type classification
- **Options**: External, Internal, API
- **Default**: External
- **Description**: Categorizes the route for management purposes

### 2. Route Properties

**Route ID**
- **Purpose**: Unique identifier for the route
- **Format**: String
- **Auto-generated**: Yes (if empty)
- **Editable**: Only during creation
- **Description**: Internal system identifier

**Domain ID**
- **Purpose**: Associated domain
- **Format**: String
- **Default**: "default"
- **Description**: Links the route to a specific domain

**Owner ID**
- **Purpose**: Route owner identifier
- **Format**: String
- **Default**: "user-1"
- **Description**: Associates the route with a user

**Tags**
- **Purpose**: Categorization and filtering
- **Format**: Comma-separated list
- **Example**: "marketing, campaign, social"
- **Description**: Helps organize and filter routes

**Scripts**
- **Purpose**: JavaScript files to load
- **Format**: One script path per line
- **Example**:
  ```
  script1.js
  script2.js
  script3.js
  ```
- **Description**: Scripts to execute when the route is accessed

**Custom Properties**
- **Purpose**: Additional metadata
- **Format**: Valid JSON
- **Example**:
  ```json
  {
    "key1": "value1",
    "key2": "value2"
  }
  ```
- **Description**: Flexible storage for custom route data

### 3. Advanced Settings

**Enable OpenGraph**
- **Purpose**: Generate OpenGraph metadata
- **Type**: Boolean
- **Default**: True
- **Description**: Creates social media sharing metadata

**Allow Debug**
- **Purpose**: Enable debug mode
- **Type**: Boolean
- **Default**: False
- **Description**: Enables additional logging and debugging

## 🔧 Form Features

### Validation

**Required Fields**
- Short URL Path
- Destination URL

**Format Validation**
- URL format for destination
- JSON format for custom properties
- Numeric range for TTL (60-86400)

**Real-time Feedback**
- Dynamic help text
- Field-specific guidance
- Error highlighting

### User Experience

**Smart Defaults**
- Pre-filled with sensible values
- Context-aware placeholders
- Auto-generation for IDs

**Progressive Disclosure**
- Basic settings first
- Advanced options in separate section
- Clear section headers

**Accessibility**
- Proper form labels
- Help text for all fields
- Keyboard navigation support

## 📊 Data Structure

The form maps to the following DTO structure:

```typescript
interface RouteDto {
  // Basic Properties
  switch: string;
  link: string;
  dest: string;
  destFormat: string;
  code: number;
  ttl: number;
  status: string;
  terminal: string;
  
  // Route Properties
  properties: {
    routeId: string;
    domainId: string;
    ownerId: string;
    scripts: string[];
    tags: string[];
    custom: Record<string, any>;
    opengraph: boolean;
    allowDebug: boolean;
  };
}
```

## 🚀 Usage Examples

### Creating a Simple Route

1. **Short URL Path**: `product-launch`
2. **Destination URL**: `https://example.com/products/new`
3. **Redirect Code**: `301`
4. **Status**: `Active`
5. **Tags**: `marketing, launch`

### Creating an Advanced Route

1. **Basic Settings**:
   - Switch: `main`
   - Path: `api-docs`
   - Destination: `https://docs.example.com/api`
   - Code: `301`
   - TTL: `7200`
   - Status: `Active`
   - Format: `Https`
   - Terminal: `API`

2. **Properties**:
   - Domain ID: `api-domain`
   - Owner ID: `dev-team`
   - Tags: `documentation, api, public`
   - Scripts: `analytics.js`, `tracking.js`

3. **Advanced**:
   - OpenGraph: `true`
   - Debug: `false`

## 🎯 Best Practices

### Route Naming
- Use descriptive, memorable paths
- Avoid special characters
- Keep paths short but meaningful
- Use hyphens instead of spaces

### Destination URLs
- Always use HTTPS when possible
- Include full URLs with protocols
- Test destinations before creating routes
- Consider mobile-friendly destinations

### TTL Settings
- Use longer TTL for stable content
- Use shorter TTL for dynamic content
- Consider cache invalidation needs
- Balance performance vs. flexibility

### Tags and Organization
- Use consistent tagging conventions
- Group related routes with similar tags
- Use hierarchical tags when appropriate
- Regular cleanup of unused tags

## 🔍 Troubleshooting

### Common Issues

**Invalid URL Format**
- Ensure protocol is included (http:// or https://)
- Check for typos in the domain
- Verify the destination is accessible

**JSON Validation Errors**
- Ensure proper JSON syntax
- Use double quotes for strings
- Check for trailing commas
- Validate with JSON linter

**Route ID Conflicts**
- Route IDs must be unique
- Use descriptive, unique identifiers
- Consider using timestamps or UUIDs
- Check existing routes for conflicts

### Form Validation

**Required Field Errors**
- All required fields must be filled
- Empty strings are not valid
- Whitespace-only values are rejected

**Format Validation**
- URLs must be properly formatted
- JSON must be valid syntax
- Numbers must be within valid ranges
- Tags must be properly formatted

## 📚 Related Documentation

- [Routes API Guide](../api/README.md)
- [Routes Integration Guide](./ROUTES_INTEGRATION.md)
- [API Testing Script](../api/test-auth.sh)
- [Keycloak Setup Guide](../api/KEYCLOAK_SETUP.md)

## 🎉 Features Summary

### ✅ Complete DTO Coverage
- All Route properties supported
- All RouteProperties fields included
- Proper data type handling
- Full validation support

### ✅ Enhanced User Experience
- Organized form sections
- Clear field labels and help text
- Real-time validation
- Smart defaults and auto-generation

### ✅ Advanced Features
- JSON editor for custom properties
- Multi-line script support
- Tag management
- Debug and OpenGraph toggles

### ✅ Form Validation
- Required field validation
- Format validation (URL, JSON)
- Range validation (TTL)
- Real-time error feedback

The route form now provides comprehensive coverage of all DTO properties with an intuitive, well-organized interface! 🚀

