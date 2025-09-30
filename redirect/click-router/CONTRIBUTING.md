# Contributing to Click Router

Thank you for your interest in contributing to Click Router! This document provides guidelines and information for contributors.

## 🤝 How to Contribute

### Reporting Issues

Before creating an issue, please:

1. **Search existing issues** to avoid duplicates
2. **Check the documentation** for solutions
3. **Test with the latest version**

When creating an issue, please include:

- **Clear description** of the problem
- **Steps to reproduce** the issue
- **Expected vs actual behavior**
- **Environment details** (OS, Rust version, etc.)
- **Relevant logs** or error messages

### Feature Requests

For feature requests, please:

1. **Check existing issues** for similar requests
2. **Describe the use case** and benefits
3. **Provide examples** of how it would work
4. **Consider implementation complexity**

### Pull Requests

We welcome pull requests! Please follow these guidelines:

## 🚀 Getting Started

### Prerequisites

- **Rust 1.75+** (stable channel)
- **Git** for version control
- **Docker** (optional, for testing)
- **MongoDB** or **DynamoDB** for testing

### Development Setup

1. **Fork the repository**
   ```bash
   # Fork on GitHub, then clone your fork
   git clone https://github.com/YOUR_USERNAME/shortas.git
   cd shortas/redirect/click-router
   ```

2. **Add upstream remote**
   ```bash
   git remote add upstream https://github.com/FlyingCow/shortas.git
   ```

3. **Install dependencies**
   ```bash
   cargo build
   ```

4. **Run tests**
   ```bash
   cargo test
   ```

5. **Start development server**
   ```bash
   cargo run
   ```

## 📋 Development Guidelines

### Code Style

#### Rust Conventions

- Follow **Rust naming conventions**
- Use **rustfmt** for formatting
- Use **clippy** for linting
- Write **comprehensive documentation**

#### Formatting

```bash
# Format code
cargo fmt

# Check formatting
cargo fmt -- --check
```

#### Linting

```bash
# Run clippy
cargo clippy

# Run clippy with all warnings
cargo clippy -- -W clippy::all
```

#### Documentation

```bash
# Generate documentation
cargo doc --open

# Check documentation
cargo doc --no-deps
```

### Project Structure

```
src/
├── adapters/          # External service integrations
│   ├── aws/          # AWS services (DynamoDB, etc.)
│   ├── mongodb/      # MongoDB integration
│   ├── moka/         # Caching layer
│   └── fluvio/       # Analytics streaming
├── core/             # Core business logic
│   ├── flow_router.rs # Main routing engine
│   ├── modules/      # Processing modules
│   └── expression.rs # Expression evaluation
├── model/            # Data models and types
├── settings.rs       # Configuration management
└── utils/            # Utility functions
```

### Architecture Principles

1. **Modularity**: Keep components loosely coupled
2. **Testability**: Write testable code with clear interfaces
3. **Performance**: Consider performance implications
4. **Security**: Follow security best practices
5. **Documentation**: Document public APIs thoroughly

## 🧪 Testing

### Running Tests

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run with output
cargo test -- --nocapture

# Run integration tests
cargo test --test integration
```

### Test Structure

#### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_function_name() {
        // Arrange
        let input = create_test_input();
        
        // Act
        let result = function_under_test(input);
        
        // Assert
        assert_eq!(result, expected_output);
    }
}
```

#### Integration Tests

```rust
#[tokio::test]
async fn test_flow_router_integration() {
    // Setup
    let router = create_test_router().await;
    let request = create_test_request();
    
    // Execute
    let result = router.handle(&request).await;
    
    // Verify
    assert!(result.is_ok());
    assert!(matches!(result.unwrap(), FlowRouterResult::Redirect(_, _)));
}
```

#### Performance Tests

```rust
#[bench]
fn bench_route_matching(b: &mut Bencher) {
    let router = create_benchmark_router();
    let request = create_benchmark_request();
    
    b.iter(|| {
        router.match_route(&request)
    });
}
```

### Test Data

- Use **realistic test data**
- Include **edge cases**
- Test **error conditions**
- Verify **performance characteristics**

## 📝 Commit Guidelines

### Commit Messages

Follow the **Conventional Commits** specification:

```
<type>[optional scope]: <description>

[optional body]

[optional footer(s)]
```

#### Types

- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting, etc.)
- `refactor`: Code refactoring
- `test`: Adding or updating tests
- `chore`: Maintenance tasks

#### Examples

```
feat(routing): add conditional routing support

Add support for conditional routing based on user agent,
device type, and geographic location. Includes expression
evaluation engine and comprehensive test coverage.

Closes #123
```

```
fix(cache): resolve memory leak in Moka cache

Fix memory leak caused by improper cache invalidation
in the Moka routes cache. Add proper cleanup in
cache invalidation methods.

Fixes #456
```

### Branch Naming

- `feature/description` - New features
- `fix/description` - Bug fixes
- `docs/description` - Documentation updates
- `refactor/description` - Code refactoring

## 🔄 Pull Request Process

### Before Submitting

1. **Create a feature branch**
   ```bash
   git checkout -b feature/your-feature-name
   ```

2. **Make your changes**
   - Write code following guidelines
   - Add comprehensive tests
   - Update documentation
   - Run all tests and checks

3. **Commit your changes**
   ```bash
   git add .
   git commit -m "feat: add your feature"
   ```

4. **Push to your fork**
   ```bash
   git push origin feature/your-feature-name
   ```

### Pull Request Template

```markdown
## Description
Brief description of changes

## Type of Change
- [ ] Bug fix
- [ ] New feature
- [ ] Breaking change
- [ ] Documentation update

## Testing
- [ ] Unit tests added/updated
- [ ] Integration tests added/updated
- [ ] Manual testing completed

## Checklist
- [ ] Code follows style guidelines
- [ ] Self-review completed
- [ ] Documentation updated
- [ ] Tests pass locally
- [ ] No merge conflicts
```

### Review Process

1. **Automated checks** must pass
2. **Code review** by maintainers
3. **Testing** in staging environment
4. **Approval** from maintainers
5. **Merge** to main branch

## 🏗️ Adding New Features

### New Modules

1. **Create module file**
   ```rust
   // src/core/modules/your_module.rs
   use crate::core::flow_module::{FlowModule, FlowStepContinuation};
   
   pub struct YourModule {
       // Module state
   }
   
   impl YourModule {
       pub fn new() -> Self {
           Self {
               // Initialize
           }
       }
   }
   
   #[async_trait::async_trait]
   impl FlowModule for YourModule {
       // Implement trait methods
       async fn handle_start(&self, context: &mut FlowRouterContext, router: &FlowRouter) -> Result<FlowStepContinuation> {
           // Implementation
           Ok(FlowStepContinuation::Continue)
       }
   }
   ```

2. **Add to module enum**
   ```rust
   // src/core/modules/mod.rs
   pub enum FlowModules {
       // ... existing modules
       YourModule(YourModule),
   }
   ```

3. **Update module handling**
   ```rust
   impl FlowModule for FlowModules {
       async fn handle_start(&self, context: &mut FlowRouterContext, router: &FlowRouter) -> Result<FlowStepContinuation> {
           match self {
               // ... existing cases
               FlowModules::YourModule(module) => module.handle_start(context, router).await,
           }
       }
   }
   ```

### New Adapters

1. **Create adapter file**
   ```rust
   // src/adapters/your_service/mod.rs
   use crate::core::your_trait::YourTrait;
   
   pub struct YourServiceAdapter {
       // Adapter state
   }
   
   impl YourServiceAdapter {
       pub fn new(config: &YourConfig) -> Self {
           Self {
               // Initialize
           }
       }
   }
   
   #[async_trait::async_trait]
   impl YourTrait for YourServiceAdapter {
       async fn your_method(&self, input: Input) -> Result<Output> {
           // Implementation
       }
   }
   ```

2. **Add to adapter enum**
   ```rust
   // src/adapters/mod.rs
   pub enum YourAdapterType {
       YourService(YourServiceAdapter),
   }
   ```

3. **Update trait implementation**
   ```rust
   #[async_trait::async_trait]
   impl YourTrait for YourAdapterType {
       async fn your_method(&self, input: Input) -> Result<Output> {
           match self {
               YourAdapterType::YourService(adapter) => adapter.your_method(input).await,
           }
       }
   }
   ```

## 🐛 Debugging

### Debug Mode

```bash
# Enable debug logging
export RUST_LOG=debug
cargo run

# Debug specific module
export RUST_LOG=click_router::core::flow_router=debug
```

### Common Issues

#### Compilation Errors

```bash
# Check for unused imports
cargo check

# Check for dead code
cargo check --features dead_code
```

#### Runtime Issues

```bash
# Run with backtrace
RUST_BACKTRACE=1 cargo run

# Run with detailed logging
RUST_LOG=trace cargo run
```

#### Test Failures

```bash
# Run specific test with output
cargo test test_name -- --nocapture

# Run tests in single thread
cargo test -- --test-threads=1
```

## 📚 Documentation

### Code Documentation

```rust
/// Brief description of the function
///
/// Longer description if needed, explaining the purpose,
/// behavior, and any important details.
///
/// # Arguments
///
/// * `param1` - Description of parameter 1
/// * `param2` - Description of parameter 2
///
/// # Returns
///
/// Description of return value
///
/// # Errors
///
/// Description of possible errors
///
/// # Examples
///
/// ```
/// let result = your_function(input);
/// assert_eq!(result, expected);
/// ```
pub fn your_function(param1: Type1, param2: Type2) -> Result<Output> {
    // Implementation
}
```

### API Documentation

- Document all **public APIs**
- Include **examples** where helpful
- Explain **error conditions**
- Document **performance characteristics**

### README Updates

- Update **installation instructions**
- Add **new features** to feature list
- Update **configuration examples**
- Add **troubleshooting** information

## 🚀 Performance Considerations

### Benchmarking

```bash
# Run benchmarks
cargo bench

# Run specific benchmark
cargo bench --bench your_benchmark
```

### Performance Guidelines

1. **Measure before optimizing**
2. **Profile with realistic data**
3. **Consider memory usage**
4. **Test with concurrent load**
5. **Document performance characteristics**

## 🔒 Security

### Security Guidelines

1. **Validate all inputs**
2. **Sanitize user data**
3. **Use secure defaults**
4. **Follow OWASP guidelines**
5. **Regular security audits**

### Security Testing

```bash
# Run security checks
cargo audit

# Check for vulnerabilities
cargo audit --deny warnings
```

## 📞 Getting Help

### Community

- **GitHub Issues**: For bugs and feature requests
- **GitHub Discussions**: For questions and discussions
- **Discord**: Real-time chat (if available)

### Maintainers

- **@FlyingCow**: Project maintainer
- **@contributors**: Active contributors

## 📄 License

By contributing to Click Router, you agree that your contributions will be licensed under the MIT License.

## 🙏 Recognition

Contributors will be recognized in:

- **README.md** contributors section
- **Release notes** for significant contributions
- **GitHub** contributor statistics

Thank you for contributing to Click Router! 🚀

