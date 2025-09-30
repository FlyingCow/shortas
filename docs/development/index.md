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

# Complete development setup (installs dependencies, starts infrastructure, builds services, runs tests)
make dev-setup

# Start all services
make dev-start
```

## 🏗️ Project Structure

The Shortas project is a monorepo containing several Rust microservices and supporting infrastructure.

```
.
├── docs/                  # Project documentation (Jekyll)
├── redirect/              # Core URL redirection services
│   ├── click-router/      # Main redirect service
│   ├── click-tracker/     # Click processing service
│   ├── click-aggregator/  # Analytics aggregation
│   ├── click-router-api/  # Route management API
│   └── click-aggregator-api/ # Analytics API
├── salvo/                 # Salvo web framework (dependency/fork)
├── infra/                 # Infrastructure setup (Docker, Terraform, AWS)
├── data/                  # Static data (GeoIP, UA parser)
├── docker-compose.yml     # Complete system deployment for local dev
└── makefile               # Enhanced build and development system
```

### Service-Specific Structures

Each Rust service (`click-router`, `click-tracker`, etc.) generally follows a similar internal structure:

```
<service-name>/
├── Cargo.toml             # Rust package manifest
├── src/                   # Source code
│   ├── adapters/          # Service integrations (DB, Cache, MQ)
│   ├── core/              # Core business logic
│   ├── model/             # Data models (structs, enums)
│   ├── settings.rs        # Configuration loading
│   └── main.rs            # Application entry point
├── config/                # Configuration files (default.toml, development.toml, etc.)
├── Dockerfile             # Docker build instructions
└── makefile               # Service-specific make commands
```

## 🛠️ Building the Project

Shortas uses `cargo` for Rust builds and `make` for an integrated build system.

### Building All Services

From the project root:
```bash
# Development build (debug mode)
make build

# Release build (optimized for performance)
make build-release

# Build all Docker images
make build-docker
```

### Building a Specific Service

You can build individual services:
```bash
make build-click-router
make build-click-router-api
# etc.
```

## 🧪 Testing

Comprehensive testing is crucial for Shortas.

### Running All Tests

From the project root:
```bash
make test
```

### Running Tests for a Specific Service

```bash
make test-click-router
make test-click-tracker
# etc.
```

### Test Coverage

Generate test coverage reports:
```bash
make test-coverage
```
This typically uses `cargo tarpaulin` and generates an HTML report.

## 📏 Code Quality

Maintain high code quality with formatting and linting.

### Formatting

```bash
make format # Formats all Rust code using rustfmt
```

### Linting

```bash
make lint # Runs clippy for linting
```

### Security Audit

Check for known vulnerabilities in dependencies:
```bash
cargo audit
```

## 🤝 Contributing Guidelines

We welcome contributions! Please follow these guidelines to ensure a smooth collaboration.

### 1. Fork the Repository

Fork the [Shortas GitHub repository](https://github.com/FlyingCow/shortas) to your own GitHub account.

### 2. Clone Your Fork

```bash
git clone https://github.com/YOUR_USERNAME/shortas.git
cd shortas
```

### 3. Create a Feature Branch

Always work on a new branch for your features or bug fixes:
```bash
git checkout -b feature/your-feature-name
# or
git checkout -b bugfix/issue-description
```

### 4. Make Your Changes

-   Implement your feature or fix the bug.
-   Ensure your code adheres to the existing code style.
-   Add or update tests to cover your changes.
-   Update documentation (code comments, READMEs, `docs/` files) as necessary.

### 5. Run Tests and Checks

Before committing, ensure all tests pass and code quality checks are clean:
```bash
make check # Runs lint, test, and build
```

### 6. Commit Your Changes

Write clear and concise commit messages.
```bash
git commit -m "feat: Add a new amazing feature"
# or
git commit -m "fix: Resolve issue with X"
```

### 7. Push to Your Fork

```bash
git push origin feature/your-feature-name
```

### 8. Open a Pull Request

-   Go to the original Shortas repository on GitHub.
-   You should see a prompt to open a new Pull Request from your branch.
-   Provide a detailed description of your changes, why they are needed, and any relevant issue numbers.
-   Ensure your PR passes all CI checks.

## 🐛 Debugging

### Local Debugging

-   **Enable Debug Logging**: Set the `RUST_LOG` environment variable.
    ```bash
    RUST_LOG=debug make dev-start
    # Or for a specific module:
    RUST_LOG=click_router::core::flow_router=debug make dev-start
    ```
-   **IDE Integration**: Use Rust's debugging tools with your IDE (e.g., VS Code with `rust-analyzer`).

### Docker Debugging

-   **Access Container Shell**:
    ```bash
    docker exec -it <container-id-or-name> /bin/bash
    ```
-   **View Container Logs**:
    ```bash
    docker logs <container-id-or-name>
    # Or using make:
    make logs-router
    ```

---

**Thank you for contributing to Shortas!** Your efforts help make this project better.
