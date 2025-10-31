# Benchmarks Quick Start

## Prerequisites

Before running benchmarks, ensure the project compiles:

```bash
cargo check
```

## Running Benchmarks

Once the project compiles:

```bash
# Run all benchmarks
cargo bench

# Run specific benchmark suite
cargo bench --bench user_agent_parsing
cargo bench --bench aggregate_module
cargo bench --bench context_creation
cargo bench --bench full_pipeline

# Run specific test pattern
cargo bench -- user_agent
cargo bench -- comparison
cargo bench -- pipeline
```

## What Each Benchmark Tests

### 1. user_agent_parsing.rs
Compares parsing user agent string 3 times vs once using `parse_client()`

### 2. aggregate_module.rs
Compares excessive cloning vs optimized string handling in ClickStreamItem creation

### 3. context_creation.rs
Compares eager HashMap allocation vs lazy initialization in TrackingPipeContext

### 4. full_pipeline.rs ⭐ NEW
**End-to-end pipeline benchmark with NO external dependencies**
- Simulates the complete event processing pipeline
- Uses mock implementations (no Redis, Fluvio, GeoIP, etc.)
- Measures full pipeline throughput
- Identifies bottlenecks in each processing step
- Tests different event patterns (full data, minimal data, etc.)
- Memory pressure testing with 10k events

## Reading Results

See [BENCHMARKS.md](../BENCHMARKS.md) for detailed documentation on:
- Understanding Criterion output
- Interpreting performance metrics
- Expected improvements from optimizations
- Advanced profiling techniques

## Quick Example

```bash
$ cargo bench --bench user_agent_parsing -- comparison

user_agent_comparison/triple_parse
                        time:   [45.123 µs 45.456 µs 45.789 µs]

user_agent_comparison/single_parse_client
                        time:   [15.234 µs 15.567 µs 15.890 µs]
```

The single parse is ~3x faster! 🚀

## Benchmark Files

- `user_agent_parsing.rs` - User agent parser optimization benchmarks
- `aggregate_module.rs` - String cloning optimization benchmarks
- `context_creation.rs` - HashMap allocation optimization benchmarks

All benchmarks use [Criterion.rs](https://github.com/bheisler/criterion.rs) for statistical analysis.
