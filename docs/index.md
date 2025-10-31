---
layout: vector-theme
title: Shortas Documentation
description: Comprehensive documentation for Shortas - a fast and scalable URL shortener with advanced analytics, multi-tenancy, and real-time click tracking capabilities.
---

<div class="hero-section">
  <h1>Welcome to Shortas Documentation</h1>
  <p class="lead">**Shortas** is a high-performance, enterprise-grade URL shortener built with Rust. Featuring advanced analytics, multi-tenancy, and real-time click tracking capabilities.</p>
  
  <div class="hero-badges">
    <span class="badge badge-success">Production Ready</span>
    <span class="badge badge-info">High Performance</span>
    <span class="badge badge-warning">Enterprise Grade</span>
  </div>
</div>

## 🚀 Quick Start

<div class="alert alert-info">
  <strong>Get up and running with Shortas in minutes!</strong> Our one-command setup will install dependencies, start infrastructure, build services, and run tests.
</div>

```bash
# Clone the repository
git clone https://github.com/FlyingCow/shortas.git
cd shortas

# Complete development setup
make dev-setup

# Start all services
make dev-start
```

<div class="text-center mt-3">
  <a href="/getting-started/" class="btn">Get Started</a>
  <a href="/getting-started/installation/" class="btn btn-secondary">Installation Guide</a>
</div>

## 🏗️ Architecture Overview

<div class="card">
  <div class="card-header">Microservices Architecture</div>
  <p>Shortas is built as a microservices architecture with five core components, each designed for specific responsibilities and optimized for maximum performance and scalability.</p>
</div>

<div class="feature-grid">
  <div class="feature-card">
    <div class="feature-icon">🚀</div>
    <h3>Click Router</h3>
    <p>A high-performance, intelligent URL redirection service with advanced routing capabilities, conditional logic, analytics, and multi-database support for enterprise-grade URL shortening.</p>
    <p class="feature-meta"><strong>Performance:</strong> 360,000+ req/s (CPU) | <strong>Latency:</strong> 2.6-2.8 µs</p>
    <a href="https://github.com/FlyingCow/shortas/tree/main/redirect/click-router" class="btn" target="_blank">Learn More</a>
  </div>
  
  <div class="feature-card">
    <div class="feature-icon">📊</div>
    <h3>Click Tracker</h3>
    <p>Real-time click processing and data enrichment with geographic and device analytics. Captures detailed metadata for comprehensive user behavior analysis.</p>
    <p class="feature-meta"><strong>Performance:</strong> 1.07M events/s (CPU) | <strong>Latency:</strong> 927 ns, 7,800/s with I/O</p>
    <a href="https://github.com/FlyingCow/shortas/tree/main/redirect/click-tracker" class="btn" target="_blank">Learn More</a>
  </div>
  
  <div class="feature-card">
    <div class="feature-icon">⚡</div>
    <h3>Click Aggregator</h3>
    <p>Analytics data processing and storage with high-performance batch processing. Optimized for OLAP queries and real-time aggregations.</p>
    <p class="feature-meta"><strong>Performance:</strong> 1.05M records/s (CPU) | <strong>Latency:</strong> ~950 ns</p>
    <a href="https://github.com/FlyingCow/shortas/tree/main/redirect/click-aggregator" class="btn" target="_blank">Learn More</a>
  </div>
  
  <div class="feature-card">
    <div class="feature-icon">🔧</div>
    <h3>Click Router API</h3>
    <p>High-performance, secure API with JWT authentication via Keycloak, comprehensive OpenAPI documentation, and support for multiple database backends.</p>
    <p class="feature-meta"><strong>Performance:</strong> 5,000+ req/s | <strong>Features:</strong> Route & Settings Management</p>
    <a href="https://github.com/FlyingCow/shortas/tree/main/redirect/click-router-api" class="btn" target="_blank">Learn More</a>
  </div>
  
  <div class="feature-card">
    <div class="feature-icon">📈</div>
    <h3>Click Aggregator API</h3>
    <p>High-performance, secure click aggregation API with JWT authentication via Keycloak, comprehensive OpenAPI documentation, and ClickHouse integration for analytics.</p>
    <p class="feature-meta"><strong>Performance:</strong> 5,000+ req/s | <strong>Features:</strong> Analytics & Reporting</p>
    <a href="https://github.com/FlyingCow/shortas/tree/main/redirect/click-aggregator-api" class="btn" target="_blank">Learn More</a>
  </div>
</div>

## 📚 Documentation Sections

<div class="feature-grid">
  <div class="card">
    <div class="card-header">🚀 Getting Started</div>
    <p>Learn how to get Shortas up and running on your system.</p>
    <ul>
      <li><a href="getting-started/">Quick Start Guide</a></li>
      <li><a href="/getting-started/">Installation</a></li>
      <li><a href="/getting-started/">Configuration</a></li>
      <li><a href="/getting-started/">First Steps</a></li>
    </ul>
    <a href="getting-started/" class="btn btn-sm">View Guide →</a>
  </div>
  
  <div class="card">
    <div class="card-header">🏗️ Architecture</div>
    <p>Understand how Shortas is structured and how components interact.</p>
    <ul>
      <li><a href="architecture/">System Overview</a></li>
      <li><a href="/architecture/">Microservices</a></li>
      <li><a href="/architecture/">Data Flow</a></li>
      <li><a href="/architecture/">Security</a></li>
    </ul>
    <a href="architecture/" class="btn btn-sm">View Guide →</a>
  </div>
  
  <div class="card">
    <div class="card-header">📡 API Reference</div>
    <p>Complete API documentation with examples and integration guides.</p>
    <ul>
      <li><a href="/api/">Click Router API</a></li>
      <li><a href="/api/">Click Aggregator API</a></li>
      <li><a href="/api/">Authentication</a></li>
      <li><a href="/api/">Data Models</a></li>
    </ul>
    <a href="api/" class="btn btn-sm">View Guide →</a>
  </div>
  
  <div class="card">
    <div class="card-header">🚀 Deployment</div>
    <p>Deploy Shortas in various environments from local to production.</p>
    <ul>
      <li><a href="/deployment/">Local Development</a></li>
      <li><a href="/deployment/">Docker Deployment</a></li>
      <li><a href="/deployment/">Kubernetes</a></li>
      <li><a href="/deployment/">AWS Production</a></li>
    </ul>
    <a href="deployment/" class="btn btn-sm">View Guide →</a>
  </div>
  
  <div class="card">
    <div class="card-header">🛠️ Development</div>
    <p>Contributing guidelines and development practices for Shortas.</p>
    <ul>
      <li><a href="/development/">Contributing</a></li>
      <li><a href="/development/">Code Style</a></li>
      <li><a href="/development/">Testing</a></li>
      <li><a href="/development/">Debugging</a></li>
    </ul>
    <a href="development/" class="btn btn-sm">View Guide →</a>
  </div>
</div>

## 🛠️ Technology Stack

<div class="card">
  <div class="card-header">Modern Technology Stack</div>
  <div class="feature-grid">
    <div class="feature-card feature-card-small">
      <div class="feature-icon">🦀</div>
      <h4>Rust</h4>
      <p>Systems programming language for performance and safety</p>
    </div>
    <div class="feature-card feature-card-small">
      <div class="feature-icon">🌐</div>
      <h4>Salvo</h4>
      <p>Modern web framework for Rust</p>
    </div>
    <div class="feature-card feature-card-small">
      <div class="feature-icon">🗄️</div>
      <h4>MongoDB</h4>
      <p>Primary document database</p>
    </div>
    <div class="feature-card feature-card-small">
      <div class="feature-icon">⚡</div>
      <h4>ClickHouse</h4>
      <p>Analytics and OLAP database</p>
    </div>
    <div class="feature-card feature-card-small">
      <div class="feature-icon">🚀</div>
      <h4>Redis</h4>
      <p>Caching and session storage</p>
    </div>
    <div class="feature-card feature-card-small">
      <div class="feature-icon">☁️</div>
      <h4>AWS</h4>
      <p>Cloud infrastructure and services</p>
    </div>
  </div>
</div>

## 🚀 Key Features

<div class="feature-grid">
  <div class="card">
    <div class="card-header">🔄 Routing & Redirects</div>
    <ul>
      <li>Multiple redirect types (301, 302, proxy, retargeting)</li>
      <li>Domain-based routing with wildcard support</li>
      <li>SSL certificate management</li>
      <li>Deep link support</li>
      <li>A/B testing capabilities</li>
      <li>Conditional routing based on device, location, time</li>
    </ul>
  </div>
  
  <div class="card">
    <div class="card-header">📊 Analytics & Tracking</div>
    <ul>
      <li>Real-time click tracking</li>
      <li>Geographic analytics (country, continent, location)</li>
      <li>Device and browser analytics</li>
      <li>Session tracking and user behavior</li>
      <li>Bot detection and filtering</li>
      <li>Unique visitor tracking</li>
    </ul>
  </div>
  
  <div class="card">
    <div class="card-header">🏢 Multi-tenancy</div>
    <ul>
      <li>Workspace-based isolation</li>
      <li>User and creator role management</li>
      <li>Owner-based data segregation</li>
      <li>Custom user settings per workspace</li>
      <li>Fine-grained access control</li>
    </ul>
  </div>
</div>

## 📊 Performance

<div class="alert alert-success">
  <strong>High Performance Metrics</strong> - Shortas is designed for enterprise-scale performance with sub-millisecond response times and high throughput. All metrics are based on actual benchmark results.
</div>

<div class="feature-grid">
  <div class="feature-card feature-card-small">
    <div class="feature-icon">🚀</div>
    <h4>Click Router</h4>
    <p class="feature-stat"><strong>360,000+</strong> requests/second</p>
    <p class="feature-meta">CPU-only: 2.6-2.8 µs latency</p>
  </div>
  <div class="feature-card feature-card-small">
    <div class="feature-icon">📊</div>
    <h4>Click Tracker</h4>
    <p class="feature-stat"><strong>1.07M</strong> events/second</p>
    <p class="feature-meta">CPU: 927 ns, I/O: 7,800/s (8 workers)</p>
  </div>
  <div class="feature-card feature-card-small">
    <div class="feature-icon">⚡</div>
    <h4>Click Aggregator</h4>
    <p class="feature-stat"><strong>1.05M</strong> records/second</p>
    <p class="feature-meta">CPU-only: ~950 ns latency</p>
  </div>
  <div class="feature-card feature-card-small">
    <div class="feature-icon">🔧</div>
    <h4>APIs</h4>
    <p class="feature-stat"><strong>5,000+</strong> requests/second</p>
    <p class="feature-meta">Production estimate: &lt;5ms p95</p>
  </div>
</div>

## 🔗 Quick Links

<div class="text-center mt-4">
  <a href="https://github.com/FlyingCow/shortas" class="btn" target="_blank">GitHub Repository</a>
  <a href="https://github.com/FlyingCow/shortas/issues" class="btn btn-secondary" target="_blank">Issue Tracker</a>
  <a href="development/" class="btn btn-secondary">Contributing Guide</a>
  <a href="api/" class="btn btn-secondary">API Documentation</a>
</div>

## 📄 License

<div class="card">
  <p>This project is licensed under the <strong>MIT License</strong> - see the <a href="LICENSE">LICENSE</a> file for details.</p>
</div>

---

<div class="alert alert-info text-center">
  <strong>Need help?</strong> Check out our <a href="getting-started/">Getting Started Guide</a> or <a href="https://github.com/FlyingCow/shortas/issues" target="_blank">open an issue</a> on GitHub.
</div>
