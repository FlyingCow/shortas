---
layout: vector-theme
title: Development Guide
permalink: /development/
---

<div class="hero-section">
  <h1>Development Guide</h1>
  <p class="lead">This guide covers development practices, contribution guidelines, and technical details for contributing to Shortas. Learn how to set up your development environment, write tests, and contribute code.</p>
</div>

<div class="alert alert-info">
  <strong>🤝 Contributing to Shortas</strong><br>
  We welcome contributions from the community! This guide will help you get started with development and contribute effectively.
</div>

## 🚀 Getting Started

<div class="card">
  <div class="card-header">Prerequisites</div>
  <ul>
    <li><strong>Rust</strong> 1.75+ (stable channel)</li>
    <li><strong>Docker</strong> & Docker Compose</li>
    <li><strong>Make</strong> (GNU Make 4.0+)</li>
    <li><strong>Git</strong> - for version control</li>
    <li><strong>Code Editor</strong> - VS Code with rust-analyzer recommended</li>
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

<div class="card">
  <div class="card-header">Directory Structure</div>
  <pre><code>.
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
└── makefile               # Enhanced build and development system</code></pre>
</div>

### Service-Specific Structures

Each Rust service (`click-router`, `click-tracker`, etc.) generally follows a similar internal structure:

<div class="card">
  <div class="card-header">Service Structure</div>
  <pre><code>&lt;service-name&gt;/
├── Cargo.toml             # Rust package manifest
├── src/                   # Source code
│   ├── adapters/          # Service integrations (DB, Cache, MQ)
│   ├── core/              # Core business logic
│   ├── model/             # Data models (structs, enums)
│   ├── settings.rs        # Configuration loading
│   └── main.rs            # Application entry point
├── config/                # Configuration files (default.toml, development.toml, etc.)
├── Dockerfile             # Docker build instructions
└── makefile               # Service-specific make commands</code></pre>
</div>

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
make build-click-tracker
make build-click-aggregator
make build-click-aggregator-api
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
make test-click-aggregator
make test-click-router-api
make test-click-aggregator-api
```

### Test Coverage

Generate test coverage reports:

```bash
make test-coverage
```

This typically uses `cargo tarpaulin` and generates an HTML report.

<div class="card">
  <div class="card-header">Test Types</div>
  <ul>
    <li><strong>Unit Tests:</strong> Test individual functions and modules</li>
    <li><strong>Integration Tests:</strong> Test service interactions</li>
    <li><strong>End-to-End Tests:</strong> Test complete workflows</li>
  </ul>
</div>

## 📏 Code Quality

Maintain high code quality with formatting and linting.

### Formatting

```bash
make format  # Formats all Rust code using rustfmt
```

### Linting

```bash
make lint  # Runs clippy for linting
```

### Security Audit

Check for known vulnerabilities in dependencies:

```bash
cargo audit
```

### Pre-commit Checks

Run all quality checks before committing:

```bash
make check  # Runs lint, test, and build
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

<div class="card">
  <div class="card-header">Branch Naming Conventions</div>
  <ul>
    <li><code>feature/</code> - New features</li>
    <li><code>bugfix/</code> - Bug fixes</li>
    <li><code>hotfix/</code> - Critical fixes</li>
    <li><code>docs/</code> - Documentation changes</li>
    <li><code>refactor/</code> - Code refactoring</li>
  </ul>
</div>

### 4. Make Your Changes

- Implement your feature or fix the bug
- Ensure your code adheres to the existing code style
- Add or update tests to cover your changes
- Update documentation (code comments, READMEs, `docs/` files) as necessary

### 5. Run Tests and Checks

Before committing, ensure all tests pass and code quality checks are clean:

```bash
make check  # Runs lint, test, and build
```

### 6. Commit Your Changes

Write clear and concise commit messages following conventional commits:

```bash
git commit -m "feat: Add a new amazing feature"
# or
git commit -m "fix: Resolve issue with X"
# or
git commit -m "docs: Update API documentation"
```

<div class="card">
  <div class="card-header">Commit Message Format</div>
  <p>Use conventional commit format:</p>
  <ul>
    <li><code>feat:</code> - New feature</li>
    <li><code>fix:</code> - Bug fix</li>
    <li><code>docs:</code> - Documentation changes</li>
    <li><code>style:</code> - Code style changes (formatting)</li>
    <li><code>refactor:</code> - Code refactoring</li>
    <li><code>test:</code> - Test changes</li>
    <li><code>chore:</code> - Build process or auxiliary tool changes</li>
  </ul>
</div>

### 7. Push to Your Fork

```bash
git push origin feature/your-feature-name
```

### 8. Open a Pull Request

- Go to the original Shortas repository on GitHub
- You should see a prompt to open a new Pull Request from your branch
- Provide a detailed description of your changes, why they are needed, and any relevant issue numbers
- Ensure your PR passes all CI checks

<div class="card">
  <div class="card-header">Pull Request Checklist</div>
  <ul>
    <li>✅ All tests pass</li>
    <li>✅ Code follows project style guidelines</li>
    <li>✅ Documentation is updated</li>
    <li>✅ Commit messages follow conventional format</li>
    <li>✅ PR description explains the changes</li>
    <li>✅ Related issues are referenced</li>
  </ul>
</div>

## 🐛 Debugging

### Local Debugging

<div class="card">
  <div class="card-header">Debugging Tips</div>
  <ul>
    <li><strong>Enable Debug Logging:</strong> Set the <code>RUST_LOG</code> environment variable:
      <pre><code>RUST_LOG=debug make dev-start
# Or for a specific module:
RUST_LOG=click_router::core::flow_router=debug make dev-start</code></pre>
    </li>
    <li><strong>IDE Integration:</strong> Use Rust's debugging tools with your IDE (e.g., VS Code with <code>rust-analyzer</code>).</li>
    <li><strong>Print Debugging:</strong> Use <code>println!</code>, <code>dbg!</code>, or logging macros for debugging.</li>
  </ul>
</div>

### Docker Debugging

<div class="card">
  <div class="card-header">Docker Debugging</div>
  <ul>
    <li><strong>Access Container Shell:</strong>
      <pre><code>docker exec -it &lt;container-id-or-name&gt; /bin/bash</code></pre>
    </li>
    <li><strong>View Container Logs:</strong>
      <pre><code>docker logs &lt;container-id-or-name&gt;
# Or using make:
make logs-router</code></pre>
    </li>
    <li><strong>Inspect Running Containers:</strong>
      <pre><code>docker ps
docker inspect &lt;container-id-or-name&gt;</code></pre>
    </li>
  </ul>
</div>

## 📚 Code Style Guidelines

<div class="card">
  <div class="card-header">Rust Style Guidelines</div>
  <ul>
    <li>Follow Rust standard formatting (<code>rustfmt</code>)</li>
    <li>Run <code>cargo clippy</code> and fix warnings</li>
    <li>Use meaningful variable and function names</li>
    <li>Add comments for complex logic</li>
    <li>Keep functions small and focused</li>
    <li>Use error handling appropriately (<code>Result</code>, <code>Option</code>)</li>
  </ul>
</div>

## 🔍 Code Review Process

<div class="card">
  <div class="card-header">Review Criteria</div>
  <ul>
    <li>Code correctness and functionality</li>
    <li>Adherence to project style guidelines</li>
    <li>Test coverage</li>
    <li>Documentation updates</li>
    <li>Performance considerations</li>
    <li>Security implications</li>
  </ul>
</div>

## 📖 Documentation

When contributing, ensure documentation is updated:

<div class="card">
  <div class="card-header">Documentation Requirements</div>
  <ul>
    <li>Update README files if needed</li>
    <li>Add/update code comments</li>
    <li>Update API documentation</li>
    <li>Update architecture diagrams if structure changes</li>
    <li>Add examples if introducing new features</li>
  </ul>
</div>

## 🎯 Getting Help

If you need help or have questions:

<div class="feature-grid">
  <div class="card">
    <div class="card-header">GitHub Issues</div>
    <p>Open an issue on GitHub for bugs, feature requests, or questions.</p>
    <a href="https://github.com/FlyingCow/shortas/issues" class="btn btn-sm" target="_blank">Open Issue</a>
  </div>
  
  <div class="card">
    <div class="card-header">Documentation</div>
    <p>Check the documentation for detailed guides and references.</p>
    <a href="/getting-started/" class="btn btn-sm">View Docs</a>
  </div>
</div>

---

<div class="alert alert-success text-center">
  <strong>Thank you for contributing to Shortas!</strong> Your efforts help make this project better for everyone.
</div>
