---
layout: vector-theme
title: Development Guide
permalink: /development/
---

<div class="hero-section">
  <h1>Development Guide</h1>
  <p class="lead">This guide covers development practices, contribution guidelines, and technical details for contributing to Shortas.</p>
</div>

<div class="alert alert-info">
  <strong>🤝 Contributing to Shortas</strong><br>
  We welcome contributions from the community! This guide will help you get started with development.
</div>

<div class="card">
  <div class="card-header">🚀 Getting Started</div>
  <h4>Prerequisites</h4>
  <ul>
    <li>Rust 1.75+ (stable channel)</li>
    <li>Docker & Docker Compose</li>
    <li>Make</li>
    <li>Git</li>
  </ul>
</div>

### Development Setup

```bash
# Clone the repository
git clone https://github.com/FlyingCow/shortas.git
cd shortas

# Fork the repository (if contributing)
# Then clone your fork
git clone https://github.com/yourusername/shortas.git
cd shortas

# Set up development environment
make dev-setup

# Start development services
make dev-start
```

## 🏗️ Project Structure

```
shortas/
├── redirect/                 # Redirect module
│   ├── click-router/         # Main redirect service
│   ├── click-tracker/        # Click processing service
│   ├── click-aggregator/     # Analytics aggregation
│   ├── click-router-api/     # Route management API
│   └── click-aggregator-api/ # Analytics API
├── salvo/                    # Salvo web framework
├── docs/                     # Documentation
├── infra/                    # Infrastructure
└── makefile                  # Build system
```

## 🔧 Development Workflow

### 1. Create a Feature Branch

```bash
# Create and switch to feature branch
git checkout -b feature/your-feature-name

# Or for bug fixes
git checkout -b fix/issue-description
```

### 2. Make Changes

```bash
# Make your changes
# Follow the coding standards (see below)

# Test your changes
make test

# Run linting
make lint

# Format code
make format
```

### 3. Commit Changes

```bash
# Add changes
git add .

# Commit with descriptive message
git commit -m "feat: add new routing policy for mobile devices

- Add mobile-specific routing logic
- Update tests for new functionality
- Update documentation

Closes #123"
```

### 4. Push and Create Pull Request

```bash
# Push to your fork
git push origin feature/your-feature-name

# Create pull request on GitHub
```

## 📝 Coding Standards

### Rust Code Style

We follow standard Rust conventions:

```rust
// Use snake_case for variables and functions
let user_id = "user_123";
fn get_user_settings() -> Result<UserSettings, Error> {
    // Implementation
}

// Use PascalCase for types and traits
struct UserSettings {
    user_id: String,
    debug: bool,
}

trait RouteHandler {
    async fn handle_route(&self, route: Route) -> Result<Response, Error>;
}

// Use SCREAMING_SNAKE_CASE for constants
const MAX_RETRIES: u32 = 3;
const DEFAULT_TIMEOUT: Duration = Duration::from_secs(30);
```

### Documentation

```rust
/// Handles route processing with conditional logic.
///
/// # Arguments
///
/// * `route` - The route to process
/// * `context` - Request context with user information
///
/// # Returns
///
/// * `Result<Response, Error>` - The processed response or error
///
/// # Examples
///
/// ```rust
/// let route = Route::new("main", "test", "https://example.com");
/// let context = RequestContext::default();
/// let response = handle_route(route, context).await?;
/// ```
pub async fn handle_route(
    route: Route,
    context: RequestContext,
) -> Result<Response, Error> {
    // Implementation
}
```

### Error Handling

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RouteError {
    #[error("Route not found: {route_id}")]
    NotFound { route_id: String },
    
    #[error("Invalid route configuration: {message}")]
    InvalidConfiguration { message: String },
    
    #[error("Database error: {0}")]
    Database(#[from] mongodb::error::Error),
}

// Use Result<T, Error> for fallible operations
pub async fn get_route(route_id: &str) -> Result<Route, RouteError> {
    // Implementation
}
```

## 🧪 Testing

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use tokio::test;
    
    #[test]
    async fn test_route_creation() {
        let route = Route::new("main", "test", "https://example.com");
        assert_eq!(route.switch, "main");
        assert_eq!(route.link, "test");
        assert_eq!(route.dest, "https://example.com");
    }
    
    #[test]
    async fn test_conditional_routing() {
        let route = create_test_route();
        let context = create_test_context();
        
        let result = route.evaluate_conditions(&context).await;
        assert!(result.is_ok());
    }
}
```

### Integration Tests

```rust
#[cfg(test)]
mod integration_tests {
    use super::*;
    
    #[tokio::test]
    async fn test_route_api() {
        let app = create_test_app().await;
        let client = TestClient::new(app);
        
        let response = client
            .post("/v1/routes")
            .json(&create_test_route_data())
            .send()
            .await;
            
        assert_eq!(response.status(), 201);
    }
}
```

### Running Tests

```bash
# Run all tests
make test

# Run specific service tests
make test-click-router
make test-click-tracker
make test-click-aggregator

# Run tests with coverage
make test-coverage

# Run tests in watch mode
make test-watch
```

## 🔍 Code Quality

### Linting

```bash
# Run clippy
make lint

# Fix clippy suggestions
cargo clippy --fix

# Run specific lints
cargo clippy -- -W clippy::all
```

### Formatting

```bash
# Format code
make format

# Check formatting
cargo fmt --check
```

### Security

```bash
# Run security audit
make audit

# Check for vulnerabilities
cargo audit
```

## 📚 Documentation

### Code Documentation

```rust
/// Route handler for processing HTTP requests.
///
/// This handler processes incoming requests and determines the appropriate
/// redirect based on the route configuration and request context.
///
/// # Architecture
///
/// The handler follows a pipeline pattern:
/// 1. Extract route information
/// 2. Load context data
/// 3. Evaluate conditions
/// 4. Generate response
///
/// # Performance
///
/// - Uses async/await for non-blocking I/O
/// - Implements caching for frequently accessed routes
/// - Supports connection pooling for database operations
///
/// # Examples
///
/// ```rust
/// let handler = RouteHandler::new(config).await?;
/// let response = handler.handle_request(request).await?;
/// ```
pub struct RouteHandler {
    config: Config,
    cache: Arc<dyn Cache>,
    db: Arc<dyn Database>,
}
```

### API Documentation

```rust
/// Create a new route.
///
/// # Arguments
///
/// * `route_data` - Route configuration data
///
/// # Returns
///
/// * `Result<Route, RouteError>` - Created route or error
///
/// # Examples
///
/// ```bash
/// curl -X POST /v1/routes \
///   -H "Content-Type: application/json" \
///   -d '{"switch": "main", "link": "test", "dest": "https://example.com"}'
/// ```
#[post("/v1/routes")]
pub async fn create_route(
    route_data: Json<RouteData>,
) -> Result<Json<Route>, RouteError> {
    // Implementation
}
```

## 🚀 Performance

### Benchmarking

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark_route_processing(c: &mut Criterion) {
    c.bench_function("route_processing", |b| {
        b.iter(|| {
            let route = create_test_route();
            let context = create_test_context();
            black_box(route.process(&context))
        })
    });
}

criterion_group!(benches, benchmark_route_processing);
criterion_main!(benches);
```

### Running Benchmarks

```bash
# Run all benchmarks
make bench

# Run specific service benchmarks
make bench-click-router
make bench-click-tracker
```

### Performance Guidelines

- Use `async/await` for I/O operations
- Implement proper caching strategies
- Use connection pooling for databases
- Profile code with `cargo flamegraph`
- Monitor memory usage with `cargo valgrind`

## 🔧 Development Tools

### VS Code Extensions

```json
{
  "recommendations": [
    "rust-lang.rust-analyzer",
    "vadimcn.vscode-lldb",
    "serayuzgur.crates",
    "tamasfe.even-better-toml",
    "ms-vscode.vscode-json"
  ]
}
```

### Git Hooks

```bash
# Install pre-commit hooks
make install-hooks

# This will run:
# - cargo fmt
# - cargo clippy
# - cargo test
# - cargo audit
```

### Debugging

```bash
# Run with debug logging
RUST_LOG=debug cargo run

# Run specific module in debug
RUST_LOG=click_router::core=debug cargo run

# Use debugger
cargo run --bin click-router
# Then attach debugger to process
```

## 📋 Pull Request Guidelines

### PR Template

```markdown
## Description
Brief description of changes

## Type of Change
- [ ] Bug fix
- [ ] New feature
- [ ] Breaking change
- [ ] Documentation update

## Testing
- [ ] Unit tests pass
- [ ] Integration tests pass
- [ ] Manual testing completed

## Checklist
- [ ] Code follows style guidelines
- [ ] Self-review completed
- [ ] Documentation updated
- [ ] Tests added/updated
```

### Review Process

1. **Automated Checks**: CI/CD pipeline runs tests and linting
2. **Code Review**: At least one maintainer reviews the code
3. **Testing**: Manual testing in development environment
4. **Approval**: Maintainer approves the PR
5. **Merge**: PR is merged to main branch

## 🐛 Bug Reports

### Bug Report Template

```markdown
## Bug Description
Clear description of the bug

## Steps to Reproduce
1. Go to '...'
2. Click on '....'
3. See error

## Expected Behavior
What you expected to happen

## Actual Behavior
What actually happened

## Environment
- OS: [e.g. Ubuntu 20.04]
- Rust version: [e.g. 1.75.0]
- Docker version: [e.g. 20.10.0]

## Additional Context
Any other context about the problem
```

## 💡 Feature Requests

### Feature Request Template

```markdown
## Feature Description
Clear description of the feature

## Use Case
Why is this feature needed?

## Proposed Solution
How should this feature work?

## Alternatives
Other solutions you've considered

## Additional Context
Any other context about the feature request
```

## 📚 Additional Resources

- [Code Style Guide](code-style/) - Detailed coding standards
- [Testing Guide](testing/) - Testing best practices
- [Debugging Guide](debugging/) - Debugging techniques
- [Performance Guide](performance/) - Performance optimization

## 🔗 Links

- [GitHub Repository](https://github.com/FlyingCow/shortas)
- [Issue Tracker](https://github.com/FlyingCow/shortas/issues)
- [Discussions](https://github.com/FlyingCow/shortas/discussions)
- [Contributing Guide](https://github.com/FlyingCow/shortas/blob/main/CONTRIBUTING.md)

---

**Ready to contribute?** Check out our [code style guide](code-style/) or [open an issue](https://github.com/FlyingCow/shortas/issues) to get started!
