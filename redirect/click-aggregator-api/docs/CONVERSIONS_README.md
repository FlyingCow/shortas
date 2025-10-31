# Conversions Documentation Index

This directory contains comprehensive documentation for the Shortas conversions functionality.

## 📚 Documentation Files

### 1. [CONVERSIONS_GUIDE.md](CONVERSIONS_GUIDE.md)
**Complete user guide and reference documentation**

- Overview and key features
- Architecture and data flow
- API endpoints (Click Router & Click Aggregator)
- Implementation guide
- Analytics examples
- Best practices

**Best for:** Understanding the full system and getting started

### 2. [CONVERSIONS_PIPELINE_INTEGRATION.md](CONVERSIONS_PIPELINE_INTEGRATION.md)
**Technical deep-dive into pipeline integration**

- Detailed data flow through Click Router → Fluvio → Click Tracker → Click Aggregator
- Integration points for each service
- Code examples for pipeline modules
- Monitoring and debugging guide
- Configuration details

**Best for:** Developers implementing or debugging the pipeline

### 3. [CONVERSIONS_IMPLEMENTATION_EXAMPLES.md](CONVERSIONS_IMPLEMENTATION_EXAMPLES.md)
**Real-world implementation examples**

- HTML/JavaScript integration
- React component examples
- Node.js/Express backend integration
- Python SDK implementation
- Complete e-commerce example

**Best for:** Developers looking for copy-paste code examples

### 4. [CONVERSIONS_ARCHITECTURE.md](CONVERSIONS_ARCHITECTURE.md)
**Architectural overview and design decisions**

- System architecture diagrams
- Data model design
- Database schema
- Materialized views
- Performance considerations

**Best for:** Understanding system design and architecture

### 5. [CONVERSIONS_PERFORMANCE.md](CONVERSIONS_PERFORMANCE.md)
**Performance metrics and optimization details**

- Benchmark results (1.05M conversions/sec CPU throughput)
- Performance comparisons (conversions vs clicks)
- Memory usage analysis (~200-450 bytes per conversion)
- Production capacity estimates
- Optimization strategies

**Best for:** Understanding performance characteristics and capacity planning

### 6. [../click-router/CONVERSIONS_API.md](../click-router/CONVERSIONS_API.md)
**Click Router API reference**

- Complete API endpoint documentation
- Request/response formats
- Example usage (JavaScript, cURL, Rust)
- Error handling
- Testing guide

**Best for:** Using the Click Router conversion tracking API

## 🚀 Quick Start

1. **Read the Guide**: Start with [CONVERSIONS_GUIDE.md](CONVERSIONS_GUIDE.md) for an overview
2. **Review Examples**: Check [CONVERSIONS_IMPLEMENTATION_EXAMPLES.md](CONVERSIONS_IMPLEMENTATION_EXAMPLES.md) for code samples
3. **Understand Integration**: Read [CONVERSIONS_PIPELINE_INTEGRATION.md](CONVERSIONS_PIPELINE_INTEGRATION.md) for technical details
4. **Reference API**: Use [CONVERSIONS_API.md](../click-router/CONVERSIONS_API.md) as API reference

## 🔑 Key Concepts

### Entry Points
- **Click Router API**: `/conversions/track` and `/conversions/funnel` - Send conversion events
- **Click Aggregator API**: `/v1/conversions/*` - Query conversion analytics

### Data Flow
```
User Action → Click Router → Fluvio → Click Tracker → Click Aggregator → ClickHouse
```

### Conversion Types
- **Purchase**: E-commerce transactions
- **Signup**: User registrations
- **Download**: File downloads
- **Form Submission**: Contact forms, lead generation
- **Custom Event**: Any user-defined action

### Attribution Models
- **Direct**: Immediate click-to-conversion
- **Session**: Within the same session
- **Time-based**: Within a time window
- **Multi-touch**: Multiple touchpoints credited

## 📖 Additional Resources

- [Click Router Documentation](../../click-router/README.md)
- [Click Aggregator API Documentation](../README.md)
- [Database Schema](../migrations/003_create_conversions_tables.sql)
- [Materialized Views](../migrations/004_create_conversion_materialized_views.sql)
- [Performance Optimizations](../../click-tracker/PERFORMANCE_OPTIMIZATIONS.md)
- [Benchmark Results](../../click-tracker/BENCHMARK_RESULTS.md)

## 🆘 Getting Help

If you encounter issues:

1. **Check Logs**: See [CONVERSIONS_PIPELINE_INTEGRATION.md](CONVERSIONS_PIPELINE_INTEGRATION.md#debugging)
2. **Review Examples**: Check [CONVERSIONS_IMPLEMENTATION_EXAMPLES.md](CONVERSIONS_IMPLEMENTATION_EXAMPLES.md)
3. **Test Endpoints**: Use the test script in [click-router/test_conversions.sh](../../click-router/test_conversions.sh)

## 📝 Version Information

- **API Version**: v1
- **Click Router**: Port 5800 (HTTPS)
- **Click Aggregator API**: Port varies (check configuration)
- **Database**: ClickHouse
- **Message Queue**: Fluvio

## ✅ Status

All conversion tracking functionality is **fully implemented and documented**:

- ✅ Click Router API endpoints
- ✅ Pipeline integration
- ✅ Click Tracker enrichment
- ✅ Click Aggregator storage
- ✅ ClickHouse tables and views
- ✅ Analytics endpoints
- ✅ Performance optimizations (1.05M conversions/sec)
- ✅ Complete documentation

