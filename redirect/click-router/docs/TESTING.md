# Click Router Testing Guide

This document provides comprehensive testing guidelines, strategies, and examples for the Click Router project.

## 🧪 Testing Strategy

### Test Pyramid

```
    /\
   /  \
  / E2E \     End-to-End Tests
 /______\
/        \
/Integration\  Integration Tests
/____________\
/              \
/   Unit Tests   \  Unit Tests
/________________\
```

### Test Categories

1. **Unit Tests** - Test individual components in isolation
2. **Integration Tests** - Test component interactions
3. **Performance Tests** - Benchmark and load testing
4. **Mock Tests** - Test with mocked dependencies
5. **End-to-End Tests** - Test complete user workflows

## 🔧 Test Setup

### Prerequisites

- **Rust 1.75+** (stable channel)
- **Cargo** for dependency management
- **Docker** (optional, for integration tests)
- **MongoDB** or **DynamoDB** (for integration tests)

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

# Run performance tests
cargo bench

# Run with coverage
cargo test --features coverage
```

## 📋 Unit Tests

### Core Components

#### Flow Router Tests

```rust
#[cfg(test)]
mod flow_router_tests {
    use super::*;
    
    #[tokio::test]
    async fn test_flow_router_context_creation() {
        let context = create_test_context();
        assert_eq!(context.current_step, FlowStep::Initial);
        assert!(context.id.len() > 0);
    }
    
    #[tokio::test]
    async fn test_flow_router_result_types() {
        let empty_result = FlowRouterResult::Empty(StatusCode::NOT_FOUND);
        assert!(matches!(empty_result, FlowRouterResult::Empty(_)));
    }
}
```

#### Module Tests

```rust
#[cfg(test)]
mod modules_tests {
    use super::*;
    
    #[tokio::test]
    async fn test_root_module_handle_start_root_path() {
        let module = RootModule::new(redirect);
        let result = module.handle_start(&mut context, &router).await;
        assert!(result.is_ok());
    }
}
```

### Test Helpers

```rust
// Test helper functions
fn create_test_request_data() -> RequestData {
    RequestData {
        uri: "https://short.ly/test".parse().unwrap(),
        headers: http::HeaderMap::new(),
        // ... other fields
    }
}

fn create_test_route() -> Route {
    Route {
        switch: "main".to_string(),
        link: "test".to_string(),
        dest: Some("https://example.com".to_string()),
        // ... other fields
    }
}
```

## 🔗 Integration Tests

### Request Processing Pipeline

```rust
#[tokio::test]
async fn test_flow_router_integration_basic_redirect() {
    let router = create_test_flow_router().await;
    let request = create_test_request();
    let response = create_test_response();

    let result = router.handle(&request, &response).await;
    assert!(result.is_ok());
    
    let flow_result = result.unwrap();
    assert!(matches!(flow_result, FlowRouterResult::Empty(StatusCode::NOT_FOUND)));
}
```

### Database Integration

```rust
#[tokio::test]
async fn test_mongodb_integration() {
    let store = MongodbRoutesStore::new(&config).await;
    let route = create_test_route();
    
    // Test route storage and retrieval
    let result = store.get_route("main", "test").await;
    assert!(result.is_ok());
}
```

### Analytics Integration

```rust
#[tokio::test]
async fn test_kafka_integration() {
    let registrar = KafkaHitRegistrar::new(&config).await;
    let hit = create_test_hit();
    
    let result = registrar.register(&hit).await;
    assert!(result.is_ok());
}
```

## ⚡ Performance Tests

### Benchmarking

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_flow_router_basic_redirect(c: &mut Criterion) {
    let router = create_test_flow_router().await;
    let request = create_test_request();
    let response = create_test_response();

    c.bench_function("flow_router_basic_redirect", |b| {
        b.iter(|| {
            let result = router.handle(black_box(&request), black_box(&response)).await;
            assert!(result.is_ok());
            result.unwrap()
        })
    });
}

criterion_group!(benches, bench_flow_router_basic_redirect);
criterion_main!(benches);
```

### Load Testing

```rust
#[tokio::test]
async fn test_concurrent_requests() {
    let router = create_test_flow_router().await;
    let mut handles = vec![];
    
    for _ in 0..100 {
        let router_clone = router.clone();
        let handle = tokio::spawn(async move {
            router_clone.handle(&request, &response).await
        });
        handles.push(handle);
    }

    for handle in handles {
        let result = handle.await.unwrap();
        assert!(result.is_ok());
    }
}
```

### Memory Testing

```rust
#[tokio::test]
async fn test_memory_usage() {
    let router = create_test_flow_router().await;
    
    // Test memory usage with multiple requests
    for _ in 0..1000 {
        let result = router.handle(&request, &response).await;
        assert!(result.is_ok());
    }
}
```

## 🎭 Mock Tests

### Mock Implementations

```rust
struct MockRoutesStore;

#[async_trait::async_trait]
impl RoutesStore for MockRoutesStore {
    async fn get_route(&self, switch: &str, path: &str) -> Result<Option<Route>, anyhow::Error> {
        if switch == "main" && path == "test" {
            Ok(Some(create_test_route()))
        } else {
            Ok(None)
        }
    }
}
```

### Mock Testing

```rust
#[tokio::test]
async fn test_mock_flow_router() {
    let router = create_test_flow_router_with_mocks().await;
    let result = router.handle(&request, &response).await;
    assert!(result.is_ok());
}
```

## 🚀 Performance Testing

### Benchmark Configuration

```toml
# Cargo.toml
[dev-dependencies]
criterion = { version = "0.6.0", features = ["html_reports", "async_futures"] }

[[bench]]
name = "flow_router"
harness = false
```

### Performance Metrics

- **Throughput**: Requests per second
- **Latency**: Response time percentiles
- **Memory**: Memory usage patterns
- **CPU**: CPU utilization

### Benchmark Results

```bash
# Run benchmarks
cargo bench

# Generate HTML report
cargo bench -- --output-format html
```

## 🔍 Test Coverage

### Coverage Tools

```bash
# Install coverage tools
cargo install cargo-tarpaulin

# Run coverage
cargo tarpaulin --out Html

# Generate coverage report
cargo tarpaulin --out Html --output-dir coverage/
```

### Coverage Goals

- **Unit Tests**: 90%+ coverage
- **Integration Tests**: 80%+ coverage
- **Critical Paths**: 100% coverage

## 🐛 Test Debugging

### Debug Mode

```bash
# Enable debug logging
export RUST_LOG=debug
cargo test

# Debug specific test
cargo test test_name -- --nocapture
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
RUST_BACKTRACE=1 cargo test

# Run with detailed logging
RUST_LOG=trace cargo test
```

#### Test Failures

```bash
# Run specific test with output
cargo test test_name -- --nocapture

# Run tests in single thread
cargo test -- --test-threads=1
```

## 📊 Test Metrics

### Quality Metrics

- **Test Coverage**: Percentage of code covered
- **Test Execution Time**: How long tests take to run
- **Test Reliability**: Percentage of tests that pass consistently
- **Test Maintainability**: How easy tests are to update

### Performance Metrics

- **Test Throughput**: Tests per second
- **Memory Usage**: Peak memory during tests
- **CPU Usage**: CPU utilization during tests

## 🔧 Test Configuration

### Environment Variables

```bash
# Test environment
export RUST_LOG=debug
export TEST_DATABASE_URL=mongodb://localhost:27017/test
export TEST_KAFKA_BROKERS=localhost:9092
```

### Test Configuration Files

```toml
# config/test.toml
[server]
threads = 2

[debug]
enabled = true
verbose = true

[mongodb]
uri = "mongodb://localhost:27017/test"
database = "shortas_test"
```

## 🚀 Continuous Integration

### GitHub Actions

```yaml
name: Tests

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v2
    - name: Install Rust
      uses: actions-rs/toolchain@v1
      with:
        toolchain: stable
    - name: Run tests
      run: cargo test
    - name: Run benchmarks
      run: cargo bench
```

### Test Automation

```bash
# Pre-commit hooks
#!/bin/bash
cargo test
cargo clippy
cargo fmt --check
```

## 📚 Best Practices

### Test Organization

1. **Group related tests** in modules
2. **Use descriptive test names**
3. **Keep tests independent**
4. **Use setup/teardown functions**

### Test Data

1. **Use realistic test data**
2. **Include edge cases**
3. **Test error conditions**
4. **Verify performance characteristics**

### Mock Usage

1. **Mock external dependencies**
2. **Use mocks for isolation**
3. **Verify mock interactions**
4. **Keep mocks simple**

### Performance Testing

1. **Measure before optimizing**
2. **Profile with realistic data**
3. **Test with concurrent load**
4. **Document performance characteristics**

## 🔒 Security Testing

### Input Validation

```rust
#[tokio::test]
async fn test_malicious_input() {
    let malicious_uri = "https://short.ly/../../../etc/passwd";
    let request = create_request_with_uri(malicious_uri);
    
    let result = router.handle(&request, &response).await;
    assert!(result.is_ok());
    // Should sanitize the input
}
```

### Authentication

```rust
#[tokio::test]
async fn test_unauthorized_access() {
    let request = create_unauthorized_request();
    let result = router.handle(&request, &response).await;
    
    assert!(result.is_ok());
    let flow_result = result.unwrap();
    assert!(matches!(flow_result, FlowRouterResult::Empty(StatusCode::UNAUTHORIZED)));
}
```

## 📈 Test Reporting

### Test Reports

```bash
# Generate test report
cargo test -- --format=pretty

# Generate coverage report
cargo tarpaulin --out Html --output-dir coverage/

# Generate benchmark report
cargo bench -- --output-format html
```

### Test Metrics Dashboard

- **Test Results**: Pass/fail rates
- **Coverage**: Code coverage percentages
- **Performance**: Benchmark results
- **Trends**: Historical test data

## 🛠️ Test Maintenance

### Test Updates

1. **Update tests when code changes**
2. **Remove obsolete tests**
3. **Refactor test code**
4. **Add new test cases**

### Test Documentation

1. **Document test purpose**
2. **Explain test setup**
3. **Describe expected behavior**
4. **Update when requirements change**

## 🚀 Advanced Testing

### Property-Based Testing

```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_route_serialization(route in any::<Route>()) {
        let serialized = serde_json::to_string(&route)?;
        let deserialized: Route = serde_json::from_str(&serialized)?;
        assert_eq!(route, deserialized);
    }
}
```

### Fuzz Testing

```rust
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if let Ok(route) = serde_json::from_slice::<Route>(data) {
        // Test route processing
        let result = router.handle_route(&route).await;
        assert!(result.is_ok());
    }
});
```

### Chaos Testing

```rust
#[tokio::test]
async fn test_database_failure() {
    // Simulate database failure
    let router = create_router_with_failing_db().await;
    let result = router.handle(&request, &response).await;
    
    // Should handle failure gracefully
    assert!(result.is_ok());
}
```

This testing guide provides comprehensive coverage of testing strategies, tools, and best practices for the Click Router project. Follow these guidelines to ensure high-quality, reliable, and maintainable tests.

