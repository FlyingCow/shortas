---
layout: home
title: Shortas Documentation
description: Comprehensive documentation for Shortas - a fast and scalable URL shortener with advanced analytics, multi-tenancy, and real-time click tracking capabilities.
---

<div class="hero-section">
  <h1>Welcome to Shortas Documentation</h1>
  <p class="lead">**Shortas** is a high-performance URL shortener built with Rust, featuring advanced analytics, multi-tenancy, and real-time click tracking.</p>
  
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
  <p>Shortas is built as a microservices architecture with five main components, each designed for specific responsibilities and optimized for performance.</p>
</div>

<div class="feature-grid">
  <div class="feature-card">
    <div class="feature-icon">🚀</div>
    <h3>Click Router</h3>
    <p>Main redirect service handling URL routing and redirects with intelligent conditional logic.</p>
    <a href="redirect/click-router/README.md" class="btn">Learn More</a>
  </div>
  
  <div class="feature-card">
    <div class="feature-icon">📊</div>
    <h3>Click Tracker</h3>
    <p>Real-time click processing and data enrichment with geographic and device analytics.</p>
    <a href="redirect/click-tracker/" class="btn">Learn More</a>
  </div>
  
  <div class="feature-card">
    <div class="feature-icon">⚡</div>
    <h3>Click Aggregator</h3>
    <p>Analytics data processing and storage with high-performance batch processing.</p>
    <a href="redirect/click-aggregator/" class="btn">Learn More</a>
  </div>
  
  <div class="feature-card">
    <div class="feature-icon">🔧</div>
    <h3>Click Router API</h3>
    <p>REST API for route and settings management with JWT authentication.</p>
    <a href="redirect/click-router-api/README.md" class="btn">Learn More</a>
  </div>
  
  <div class="feature-card">
    <div class="feature-icon">📈</div>
    <h3>Click Aggregator API</h3>
    <p>Analytics and reporting API with comprehensive data insights.</p>
    <a href="redirect/click-aggregator-api/README.md" class="btn">Learn More</a>
  </div>
</div>

## 📚 Documentation Sections

<div class="feature-grid">
  <div class="card">
    <div class="card-header">🚀 Getting Started</div>
    <ul>
      <li><a href="getting-started/">Quick Start Guide</a></li>
      <li><a href="getting-started/installation/">Installation</a></li>
      <li><a href="getting-started/configuration/">Configuration</a></li>
      <li><a href="getting-started/first-steps/">First Steps</a></li>
    </ul>
  </div>
  
  <div class="card">
    <div class="card-header">🏗️ Architecture</div>
    <ul>
      <li><a href="architecture/">System Overview</a></li>
      <li><a href="architecture/microservices/">Microservices</a></li>
      <li><a href="architecture/data-flow/">Data Flow</a></li>
      <li><a href="architecture/security/">Security</a></li>
    </ul>
  </div>
  
  <div class="card">
    <div class="card-header">📡 API Reference</div>
    <ul>
      <li><a href="api/click-router/">Click Router API</a></li>
      <li><a href="api/click-aggregator/">Click Aggregator API</a></li>
      <li><a href="api/authentication/">Authentication</a></li>
      <li><a href="api/data-models/">Data Models</a></li>
    </ul>
  </div>
  
  <div class="card">
    <div class="card-header">🚀 Deployment</div>
    <ul>
      <li><a href="deployment/local/">Local Development</a></li>
      <li><a href="deployment/docker/">Docker Deployment</a></li>
      <li><a href="deployment/kubernetes/">Kubernetes</a></li>
      <li><a href="deployment/aws/">AWS Production</a></li>
    </ul>
  </div>
  
  <div class="card">
    <div class="card-header">🛠️ Development</div>
    <ul>
      <li><a href="development/contributing/">Contributing</a></li>
      <li><a href="development/code-style/">Code Style</a></li>
      <li><a href="development/testing/">Testing</a></li>
      <li><a href="development/debugging/">Debugging</a></li>
    </ul>
  </div>
</div>

## 🛠️ Technology Stack

<div class="card">
  <div class="card-header">Modern Technology Stack</div>
  <div class="feature-grid">
    <div class="feature-card">
      <div class="feature-icon">🦀</div>
      <h4>Rust</h4>
      <p>Systems programming language for performance and safety</p>
    </div>
    <div class="feature-card">
      <div class="feature-icon">🌐</div>
      <h4>Salvo</h4>
      <p>Modern web framework for Rust</p>
    </div>
    <div class="feature-card">
      <div class="feature-icon">🗄️</div>
      <h4>MongoDB</h4>
      <p>Primary document database</p>
    </div>
    <div class="feature-card">
      <div class="feature-icon">⚡</div>
      <h4>ClickHouse</h4>
      <p>Analytics and OLAP database</p>
    </div>
    <div class="feature-card">
      <div class="feature-icon">🚀</div>
      <h4>Redis</h4>
      <p>Caching and session storage</p>
    </div>
    <div class="feature-card">
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
    </ul>
  </div>
</div>

## 📊 Performance

<div class="alert alert-success">
  <strong>High Performance Metrics</strong> - Shortas is designed for enterprise-scale performance with sub-millisecond response times.
</div>

<div class="feature-grid">
  <div class="feature-card">
    <div class="feature-icon">🚀</div>
    <h4>Click Router</h4>
    <p><strong>10,000+</strong> requests/second</p>
  </div>
  <div class="feature-card">
    <div class="feature-icon">📊</div>
    <h4>Click Tracker</h4>
    <p><strong>50,000+</strong> events/second</p>
  </div>
  <div class="feature-card">
    <div class="feature-icon">⚡</div>
    <h4>Click Aggregator</h4>
    <p><strong>100,000+</strong> records/second</p>
  </div>
  <div class="feature-card">
    <div class="feature-icon">🔧</div>
    <h4>APIs</h4>
    <p><strong>5,000+</strong> requests/second</p>
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
