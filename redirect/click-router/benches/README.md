# Click-Router Benchmarks

This directory contains performance benchmarks for the click-router redirect processing system.

## redirect_processing Benchmark

A comprehensive benchmark suite for testing full redirect processing with **no external dependencies**. This benchmark uses in-memory stores to provide realistic performance measurements without requiring DynamoDB, Redis, MongoDB, GeoIP databases, or other external services.

### Features

- **Zero External Dependencies**: Uses in-memory route and user settings stores
- **Realistic Flow**: Tests the complete redirect processing pipeline including:
  - Request parsing and routing
  - Route lookup with caching
  - Context creation and data extraction
  - Full processing flow (Start → URL Extract → Register → Build Result → End)
- **No I/O Overhead**: All data stored in memory (HashMap-backed stores)
- **Multiple Scenarios**: Tests various request patterns and configurations

### Benchmark Scenarios

1. **simple_redirect**: Basic redirect with minimal headers (~2.6 µs/iter)
2. **chrome_desktop**: Desktop Chrome user agent (~2.7 µs/iter)
3. **mobile_safari**: Mobile Safari user agent (~2.7 µs/iter)
4. **with_query_params**: Request with UTM parameters (~2.8 µs/iter)
5. **long_path**: Multi-segment URL path (~2.8 µs/iter)
6. **parallel_10_requests**: 10 concurrent requests (~31 µs/iter)

### Running the Benchmark

```bash
# Run the full benchmark suite
cargo bench --bench redirect_processing

# Run with specific sample size
cargo bench --bench redirect_processing -- --sample-size 50

# Run and save results as baseline
cargo bench --bench redirect_processing -- --save-baseline my-baseline

# Compare against a previous baseline
cargo bench --bench redirect_processing -- --baseline my-baseline
```

### Architecture

The benchmark creates an in-memory infrastructure:

```
FlowRouter
├── RoutesCacheType::Moka (memory-backed)
│   └── RoutesStoreType::InMemory (HashMap)
├── UserSettingsCacheType::Moka (memory-backed)
│   └── UserSettingsStoreType::InMemory (HashMap)
├── UserAgentDetectorType::None (no parsing)
├── LocationDetectorType::None (no GeoIP)
└── HitRegistrarType::None (no Kafka/Fluvio)
```

### Implementation Details

#### In-Memory Stores

Two new store implementations were added:

- **InMemoryRoutesStore** (`src/adapters/memory/routes_store.rs`)
  - HashMap-backed route storage
  - Thread-safe with RwLock
  - Supports insert, remove, clear operations

- **InMemoryUserSettingsStore** (`src/adapters/memory/user_settings_store.rs`)
  - HashMap-backed user settings storage
  - Thread-safe with RwLock
  - Supports insert, remove, clear operations

These stores were integrated into the `RoutesStoreType` and `UserSettingsStoreType` enums with new `InMemory` variants.

#### Route Key Format

Routes are stored using the format: `domain%2Fpath`

Example: A request to `http://localhost/test` is looked up as:
- Switch: `"main"`
- Key: `"localhost%2Ftest"` (URL-encoded format)

### Performance Results

On a typical development machine, the benchmark shows:

- **Single request latency**: ~2.6-2.8 µs per redirect
- **Throughput**: ~360,000 redirects/second (single-threaded)
- **Concurrent performance**: 10 parallel requests in ~31 µs (~320,000 req/s aggregate)

These results demonstrate the performance of the hot path (warm cache) without external service overhead.

### Use Cases

This benchmark is ideal for:

- **Development**: Quick performance testing without infrastructure setup
- **CI/CD**: Automated performance regression testing
- **Optimization**: Measuring the impact of code changes on core redirect logic
- **Profiling**: Identifying bottlenecks in the request processing pipeline

### Limitations

- Does not benchmark external service performance (DynamoDB, Redis, etc.)
- Cache is always warm (no cold-start scenarios)
- No actual network I/O
- Simplified request patterns (no complex routing rules or conditionals)

For benchmarking with real infrastructure, see the `flow_router` benchmark (requires localstack/MongoDB).
