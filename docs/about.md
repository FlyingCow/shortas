---
layout: page
title: About Shortas
permalink: /about/
---

<div class="hero-section">
  <h1>About Shortas</h1>
  <p class="lead">**Shortas** is a fast and scalable URL shortener built with Rust, designed for enterprise-grade performance and reliability. It provides advanced analytics, multi-tenancy, and real-time click tracking capabilities.</p>
</div>

<div class="card">
  <div class="card-header">🎯 Mission</div>
  <p>To provide a high-performance, scalable URL shortening service that can handle millions of redirects with comprehensive analytics and enterprise-grade security.</p>
</div>

<div class="card">
  <div class="card-header">🏗️ Architecture Philosophy</div>
  <p>Shortas is built on the principles of:</p>
  <ul>
    <li><strong>Microservices Architecture</strong>: Each component has a single responsibility</li>
    <li><strong>High Performance</strong>: Built with Rust for maximum speed and efficiency</li>
    <li><strong>Scalability</strong>: Designed to handle massive traffic loads</li>
    <li><strong>Observability</strong>: Comprehensive monitoring and analytics</li>
    <li><strong>Security</strong>: Enterprise-grade security features</li>
  </ul>
</div>

## 🚀 Key Features

### Core Functionality
- **High-Performance Routing**: Async/await architecture for maximum throughput
- **Intelligent Redirection**: Conditional routing based on user characteristics
- **Analytics & Tracking**: Comprehensive hit tracking and user behavior analysis
- **Multi-Database Support**: MongoDB and DynamoDB integration
- **Advanced Caching**: Multi-level caching with TTL and invalidation
- **TLS Support**: Custom certificate management for HTTPS

### Advanced Routing
- **Conditional Routing**: Route users based on device, location, time, and custom expressions
- **Multiple Routing Policies**: Basic, conditional, challenge-based, file serving, and mirroring
- **A/B Testing**: Built-in support for traffic splitting

### Analytics & Monitoring
- **Hit Tracking**: Every click is logged with detailed metadata
- **Real-time Analytics**: Kafka and Fluvio integration
- **Geographic Analytics**: Country and region-based insights
- **Device Analytics**: Browser, OS, and device tracking
- **Debug Mode**: Conditional debug information for development

## 🛠️ Technology Stack

### Core Technologies
- **Rust**: Systems programming language for performance and safety
- **Salvo**: Modern web framework for Rust
- **Tokio**: Async runtime for Rust

### Databases
- **MongoDB**: Primary document database
- **ClickHouse**: Analytics and OLAP database
- **Redis**: Caching and session storage
- **DynamoDB**: AWS NoSQL database (alternative)

### Message Streaming
- **Apache Kafka**: Distributed streaming platform
- **Fluvio**: Modern streaming platform

### Infrastructure
- **Docker**: Containerization
- **Kubernetes**: Container orchestration
- **Terraform**: Infrastructure as code
- **AWS**: Cloud services

## 📊 Performance Characteristics

- **Click Router**: 10,000+ requests/second
- **Click Tracker**: 50,000+ events/second
- **Click Aggregator**: 100,000+ records/second
- **APIs**: 5,000+ requests/second
- **Latency**: Sub-millisecond response times
- **Memory**: Efficient memory usage with caching

## 🔒 Security Features

- **Input Validation**: Comprehensive URL and parameter validation
- **Rate Limiting**: Built-in abuse protection
- **Secure Headers**: Proper HTTP security headers
- **TLS Termination**: Custom certificate management
- **Authentication**: JWT and OAuth integration
- **Authorization**: Role-based access control

## 🌍 Use Cases

### Enterprise URL Shortening
- Corporate link management
- Marketing campaign tracking
- Internal tool integration

### Analytics & Insights
- Click-through rate analysis
- Geographic performance metrics
- Device and browser analytics
- User behavior tracking

### Multi-tenant SaaS
- Workspace isolation
- User management
- Custom branding
- API access control

## 🤝 Contributing

We welcome contributions from the community! Please see our [Contributing Guide](development/contributing/) for details on how to get involved.

### Ways to Contribute
- **Code**: Bug fixes, new features, performance improvements
- **Documentation**: Improve guides, add examples, fix typos
- **Testing**: Write tests, report bugs, improve test coverage
- **Community**: Help others, answer questions, share knowledge

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](https://github.com/FlyingCow/shortas/blob/main/LICENSE) file for details.

## 🔗 Links

- **GitHub Repository**: [https://github.com/FlyingCow/shortas](https://github.com/FlyingCow/shortas)
- **Issue Tracker**: [https://github.com/FlyingCow/shortas/issues](https://github.com/FlyingCow/shortas/issues)
- **Documentation**: [https://docs.shortas.com](https://docs.shortas.com)

## 🆘 Support

For support and questions:

- **Documentation**: Check our comprehensive documentation
- **Issues**: Report bugs and feature requests via GitHub Issues
- **Security**: Report security issues privately to security@shortas.com
- **Community**: Join our community discussions

---

**Built with ❤️ using Rust and modern web technologies**
