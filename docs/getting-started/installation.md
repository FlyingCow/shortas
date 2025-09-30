---
layout: page
title: Installation
permalink: /getting-started/installation/
---

# Installation Guide

This guide covers detailed installation instructions for Shortas in various environments.

## 📋 Prerequisites

### System Requirements

- **Operating System**: Linux, macOS, or Windows (with WSL2)
- **Memory**: Minimum 4GB RAM (8GB recommended)
- **Storage**: 10GB free space
- **Network**: Internet connection for downloading dependencies

### Required Software

- **Rust**: 1.75+ (stable channel)
- **Docker**: 20.10+ with Docker Compose
- **Make**: GNU Make 4.0+
- **Git**: 2.30+
- **curl**: 7.68+

## 🦀 Rust Installation

### Install Rust

```bash
# Install Rust using rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Add to PATH
source ~/.cargo/env

# Verify installation
rustc --version
cargo --version
```

### Update Rust

```bash
rustup update stable
rustup default stable
```

## 🐳 Docker Installation

### Ubuntu/Debian

```bash
# Update package index
sudo apt-get update

# Install required packages
sudo apt-get install \
    ca-certificates \
    curl \
    gnupg \
    lsb-release

# Add Docker's official GPG key
sudo mkdir -p /etc/apt/keyrings
curl -fsSL https://download.docker.com/linux/ubuntu/gpg | sudo gpg --dearmor -o /etc/apt/keyrings/docker.gpg

# Add Docker repository
echo \
  "deb [arch=$(dpkg --print-architecture) signed-by=/etc/apt/keyrings/docker.gpg] https://download.docker.com/linux/ubuntu \
  $(lsb_release -cs) stable" | sudo tee /etc/apt/sources.list.d/docker.list > /dev/null

# Install Docker
sudo apt-get update
sudo apt-get install docker-ce docker-ce-cli containerd.io docker-buildx-plugin docker-compose-plugin

# Add user to docker group
sudo usermod -aG docker $USER
```

### macOS

```bash
# Install using Homebrew
brew install --cask docker

# Or download from Docker Desktop website
# https://www.docker.com/products/docker-desktop/
```

### Windows

1. Download Docker Desktop from [docker.com](https://www.docker.com/products/docker-desktop/)
2. Install and restart your computer
3. Enable WSL2 integration if using WSL

### Verify Docker Installation

```bash
docker --version
docker compose version
```

## 🔧 Make Installation

### Ubuntu/Debian

```bash
sudo apt-get install make
```

### macOS

```bash
# Make is pre-installed, or install via Homebrew
brew install make
```

### Windows

```bash
# Install via Chocolatey
choco install make

# Or via Scoop
scoop install make
```

## 📦 Clone Repository

```bash
# Clone the repository
git clone https://github.com/FlyingCow/shortas.git
cd shortas

# Verify you're in the right directory
ls -la
```

## 🏗️ Build Dependencies

### Install System Dependencies

```bash
# Ubuntu/Debian
sudo apt-get update
sudo apt-get install -y \
    build-essential \
    pkg-config \
    libssl-dev \
    libclang-dev \
    cmake

# macOS
xcode-select --install
brew install pkg-config openssl cmake

# CentOS/RHEL
sudo yum groupinstall "Development Tools"
sudo yum install pkgconfig openssl-devel cmake
```

### Install Rust Dependencies

```bash
# Install required Rust components
rustup component add rustfmt clippy

# Install cargo tools
cargo install cargo-tarpaulin  # For test coverage
cargo install cargo-audit      # For security auditing
```

## 🚀 Build Shortas

### Development Build

```bash
# Build all services in debug mode
make build

# Or build specific service
make build-click-router
make build-click-tracker
make build-click-aggregator
make build-click-router-api
make build-click-aggregator-api
```

### Release Build

```bash
# Build all services in release mode
make build-release

# Or build specific service
make build-release-click-router
```

### Verify Build

```bash
# Check if binaries were created
ls -la redirect/target/release/
```

## 🧪 Run Tests

```bash
# Run all tests
make test

# Run tests for specific service
make test-click-router
make test-click-tracker
make test-click-aggregator

# Run tests with coverage
make test-coverage
```

## 🔍 Validate Installation

```bash
# Validate system requirements
make validate

# Check health of all services
make health-check
```

## 🐳 Docker Installation (Alternative)

If you prefer to run everything in Docker:

### Build Docker Images

```bash
# Build all Docker images
make build-docker

# Build specific service image
make build-docker-click-router
```

### Run with Docker Compose

```bash
# Start all services
docker compose up -d

# Check status
docker compose ps

# View logs
docker compose logs
```

## 🔧 Development Tools

### Recommended VS Code Extensions

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

### Git Hooks (Optional)

```bash
# Install pre-commit hooks
make install-hooks

# This will run tests and linting before commits
```

## 🚨 Troubleshooting

### Common Installation Issues

**Rust installation fails:**
```bash
# Try with different method
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- --default-toolchain stable
```

**Docker permission denied:**
```bash
# Add user to docker group
sudo usermod -aG docker $USER
# Log out and back in
```

**Build fails with SSL errors:**
```bash
# Set SSL configuration
export SSL_CERT_FILE=/etc/ssl/certs/ca-certificates.crt
# Or on macOS
export SSL_CERT_FILE=/usr/local/etc/openssl/cert.pem
```

**Out of memory during build:**
```bash
# Increase swap space or build with fewer parallel jobs
export CARGO_BUILD_JOBS=2
make build
```

### Verify Installation

```bash
# Check all components
rustc --version
cargo --version
docker --version
make --version
git --version
curl --version

# Test build
make build
make test
```

## 📚 Next Steps

Now that you have Shortas installed:

1. [Configure your deployment](configuration/)
2. [Start your first service](first-steps/)
3. [Learn about the architecture](../architecture/)
4. [Explore the APIs](../api/)

---

**Having issues?** Check our [troubleshooting guide](../deployment/troubleshooting/) or [open an issue](https://github.com/FlyingCow/shortas/issues) on GitHub.
