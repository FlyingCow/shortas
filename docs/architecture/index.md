---
layout: page
title: Architecture Overview
permalink: /architecture/
---

# Architecture Overview

Shortas is built as a microservices architecture designed for high performance, scalability, and reliability. This document provides a comprehensive overview of the system architecture.

## 🏗️ System Architecture

### High-Level Overview

```mermaid
graph TB
    A[Client Request] --> B[Load Balancer]
    B --> C[Click Router]
    C --> D[Click Tracker]
    D --> E[Click Aggregator]
    E --> F[Analytics Storage]
    
    G[Click Router API] --> H[Route Management]
    I[Click Aggregator API] --> J[Analytics API]
    
    K[Database Layer] --> L[MongoDB]
    K --> M[ClickHouse]
    K --> N[Redis]
    
    O[Message Queue] --> P[Kafka/Fluvio]
    
    C --> K
    D --> O
    E --> K
    G --> K
    I --> K
```

### Core Components

Shortas consists of five main microservices:

1. **[Click Router](microservices/click-router/)** - Main redirect service
2. **[Click Tracker](microservices/click-tracker/)** - Real-time click processing
3. **[Click Aggregator](microservices/click-aggregator/)** - Analytics data processing
4. **[Click Router API](microservices/click-router-api/)** - Route management API
5. **[Click Aggregator API](microservices/click-aggregator-api/)** - Analytics API

## 🔄 Data Flow

### Request Processing Flow

```mermaid
sequenceDiagram
    participant Client
    participant Router
    participant Tracker
    participant Aggregator
    participant Storage
    
    Client->>Router: HTTP Request
    Router->>Router: Route Resolution
    Router->>Client: HTTP Redirect
    Router->>Tracker: Click Event
    Tracker->>Aggregator: Processed Data
    Aggregator->>Storage: Analytics Data
```

### Analytics Flow

```mermaid
graph LR
    A[Click Event] --> B[Click Tracker]
    B --> C[Data Enrichment]
    C --> D[Message Queue]
    D --> E[Click Aggregator]
    E --> F[ClickHouse]
    E --> G[Analytics API]
```

## 🧩 Microservices Architecture

### Click Router

**Purpose**: Main redirect service handling URL routing and redirects

**Key Features**:
- High-performance request routing
- Conditional routing based on user characteristics
- SSL certificate management
- Multi-database support (MongoDB, DynamoDB)
- Advanced caching with TTL

**Technology Stack**:
- Rust with Salvo web framework
- Async/await architecture
- Moka caching
- MongoDB/DynamoDB integration

### Click Tracker

**Purpose**: Real-time click processing and data enrichment

**Key Features**:
- Real-time click tracking
- Geographic data enrichment
- Device and browser analytics
- Bot detection and filtering
- Session tracking

**Technology Stack**:
- Rust with async processing
- GeoIP database integration
- User agent parsing
- Kafka/Fluvio streaming

### Click Aggregator

**Purpose**: Analytics data processing and storage

**Key Features**:
- Batch processing of click data
- Data aggregation and summarization
- ClickHouse integration
- Real-time analytics
- Historical data processing

**Technology Stack**:
- Rust with batch processing
- ClickHouse for OLAP
- Kafka/Fluvio consumption
- Data compression and optimization

### Click Router API

**Purpose**: REST API for route and settings management

**Key Features**:
- CRUD operations for routes
- SSL certificate management
- User settings management
- JWT authentication
- OpenAPI documentation

**Technology Stack**:
- Rust with Salvo
- JWT authentication
- OpenAPI/Swagger integration
- MongoDB/DynamoDB integration

### Click Aggregator API

**Purpose**: Analytics and reporting API

**Key Features**:
- Click stream analytics
- Route-specific analytics
- Aggregated statistics
- Real-time metrics
- Historical reporting

**Technology Stack**:
- Rust with Salvo
- ClickHouse integration
- JWT authentication
- Real-time data processing

## 🗄️ Data Architecture

### Database Layer

#### MongoDB
- **Purpose**: Primary document storage
- **Collections**: Routes, user settings, certificates
- **Features**: High availability, horizontal scaling
- **Use Cases**: Route management, user data, configuration

#### ClickHouse
- **Purpose**: Analytics and OLAP database
- **Tables**: Click stream data, aggregated metrics
- **Features**: Columnar storage, fast aggregations
- **Use Cases**: Analytics, reporting, data warehousing

#### Redis
- **Purpose**: Caching and session storage
- **Features**: In-memory storage, high performance
- **Use Cases**: Route caching, session management, rate limiting

### Message Queue Layer

#### Apache Kafka
- **Purpose**: Distributed streaming platform
- **Topics**: Hit stream, analytics events
- **Features**: High throughput, fault tolerance
- **Use Cases**: Real-time data streaming, event processing

#### Fluvio
- **Purpose**: Modern streaming platform
- **Topics**: Hit stream, analytics events
- **Features**: Cloud-native, easy management
- **Use Cases**: Real-time analytics, event streaming

## 🔒 Security Architecture

### Authentication & Authorization

```mermaid
graph TB
    A[Client] --> B[JWT Token]
    B --> C[Keycloak]
    C --> D[Token Validation]
    D --> E[API Access]
    
    F[Role-Based Access] --> G[User Permissions]
    G --> H[Resource Access]
```

### Security Features

- **JWT Authentication**: Secure token-based authentication
- **Role-Based Access Control**: Fine-grained permissions
- **Rate Limiting**: Protection against abuse
- **Input Validation**: Comprehensive request validation
- **Security Headers**: Automatic security header injection
- **TLS/SSL**: End-to-end encryption

## 📊 Performance Architecture

### Caching Strategy

```mermaid
graph TB
    A[Request] --> B[L1 Cache]
    B --> C{Cache Hit?}
    C -->|Yes| D[Return Cached Data]
    C -->|No| E[L2 Cache]
    E --> F{Cache Hit?}
    F -->|Yes| G[Return Cached Data]
    F -->|No| H[Database Query]
    H --> I[Cache Result]
    I --> J[Return Data]
```

### Performance Optimizations

- **Multi-Level Caching**: L1 (Moka), L2 (Redis), L3 (Database)
- **Connection Pooling**: Efficient database connections
- **Async Processing**: Non-blocking I/O operations
- **Batch Processing**: Efficient data processing
- **Compression**: Data compression for storage and transmission

## 🌐 Deployment Architecture

### Container Architecture

```mermaid
graph TB
    A[Docker Compose] --> B[Click Router]
    A --> C[Click Tracker]
    A --> D[Click Aggregator]
    A --> E[Click Router API]
    A --> F[Click Aggregator API]
    
    G[Infrastructure] --> H[MongoDB]
    G --> I[ClickHouse]
    G --> J[Redis]
    G --> K[Kafka]
```

### Kubernetes Architecture

```mermaid
graph TB
    A[Kubernetes Cluster] --> B[Ingress Controller]
    B --> C[Click Router Service]
    B --> D[API Services]
    
    E[StatefulSets] --> F[MongoDB]
    E --> G[ClickHouse]
    
    H[Deployments] --> I[Click Router]
    H --> J[Click Tracker]
    H --> K[Click Aggregator]
```

## 🔧 Configuration Architecture

### Environment-Based Configuration

```toml
# Base configuration
[default]
server.threads = 8
database.url = "mongodb://localhost:27017/"

# Environment-specific overrides
[development]
server.threads = 4
debug.enabled = true

[production]
server.threads = 16
debug.enabled = false
```

### Service Discovery

- **Consul**: Service discovery and configuration
- **Kubernetes**: Native service discovery
- **Docker Compose**: Service networking
- **Load Balancers**: Traffic distribution

## 📈 Monitoring Architecture

### Observability Stack

```mermaid
graph TB
    A[Applications] --> B[Metrics]
    A --> C[Logs]
    A --> D[Traces]
    
    B --> E[Prometheus]
    C --> F[ELK Stack]
    D --> G[Jaeger]
    
    E --> H[Grafana]
    F --> I[Kibana]
    G --> J[Jaeger UI]
```

### Monitoring Components

- **Prometheus**: Metrics collection and storage
- **Grafana**: Metrics visualization and dashboards
- **ELK Stack**: Log aggregation and analysis
- **Jaeger**: Distributed tracing
- **Health Checks**: Service health monitoring

## 🔄 Scalability Architecture

### Horizontal Scaling

```mermaid
graph TB
    A[Load Balancer] --> B[Click Router 1]
    A --> C[Click Router 2]
    A --> D[Click Router N]
    
    E[Message Queue] --> F[Click Tracker 1]
    E --> G[Click Tracker 2]
    E --> H[Click Tracker N]
```

### Auto-Scaling

- **Kubernetes HPA**: Horizontal Pod Autoscaler
- **Kubernetes VPA**: Vertical Pod Autoscaler
- **Custom Metrics**: Application-specific scaling
- **Load Testing**: Performance validation

## 🚀 Development Architecture

### Development Workflow

```mermaid
graph LR
    A[Code] --> B[Build]
    B --> C[Test]
    C --> D[Deploy]
    D --> E[Monitor]
    E --> A
```

### CI/CD Pipeline

- **GitHub Actions**: Continuous integration
- **Docker**: Containerization
- **Kubernetes**: Deployment
- **Monitoring**: Health checks and alerts

## 📚 Additional Resources

- [Microservices Details](microservices/) - Detailed microservice documentation
- [Data Flow](data-flow/) - Data flow patterns and processing
- [Security](security/) - Security architecture and implementation
- [Deployment](../deployment/) - Deployment strategies and configurations

---

**Need more details?** Check out our [microservices documentation](microservices/) or [deployment guide](../deployment/).
