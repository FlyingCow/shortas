# Click Tracker Benchmarks

This document describes the benchmarks available for measuring the performance improvements from our optimizations.

## Quick Start

Run all benchmarks:
```bash
cd /home/max/dev/shortas/redirect/click-tracker
cargo bench
```

Run a specific benchmark:
```bash
cargo bench --bench user_agent_parsing
cargo bench --bench aggregate_module
cargo bench --bench context_creation
```

Run a specific test within a benchmark:
```bash
cargo bench --bench user_agent_parsing -- triple_parse
cargo bench --bench aggregate_module -- comparison
```

## Benchmark Suites

### 1. User Agent Parsing Benchmarks (`user_agent_parsing.rs`)

**Purpose:** Measure the performance improvement from parsing user agent strings once instead of three times.

**What it tests:**
- **OLD approach:** Calling `parse_user_agent()`, `parse_os()`, and `parse_device()` separately (3 parses)
- **NEW approach:** Single `parse_client()` call that returns all three at once

**Benchmark groups:**
1. `user_agent_triple_parse` - Tests old approach across different browsers/devices
2. `user_agent_single_parse` - Tests new approach across different browsers/devices
3. `user_agent_comparison` - Direct comparison on Chrome Desktop user agent
4. `user_agent_throughput` - Measures events per second for each approach

**Test user agents:**
- Chrome Desktop
- Firefox Desktop
- Safari Desktop
- Chrome Mobile (Android)
- Safari Mobile (iPhone)
- Edge
- Googlebot (spider detection)

**Expected results:**
- Single parse should be **2-3x faster** than triple parse
- Memory allocations reduced by ~66%
- Throughput improvement of 5-10%

**Example run:**
```bash
cargo bench --bench user_agent_parsing
```

**Reading results:**
```
user_agent_triple_parse/Chrome Desktop
                        time:   [45.123 µs 45.456 µs 45.789 µs]

user_agent_single_parse/Chrome Desktop
                        time:   [15.234 µs 15.567 µs 15.890 µs]
```
The single parse is ~3x faster (15µs vs 45µs).

---

### 2. Aggregate Module Benchmarks (`aggregate_module.rs`)

**Purpose:** Measure the performance improvement from reducing string cloning in the hot path.

**What it tests:**
- **OLD approach:** Using `.clone()` on every Option<String> field (10+ clones per event)
- **NEW approach:** Using `.as_ref().map()` and `.take()` to minimize allocations

**Benchmark groups:**
1. `aggregate_with_cloning` - Old approach with excessive cloning
2. `aggregate_optimized` - New approach with minimal cloning
3. `aggregate_comparison` - Direct side-by-side comparison
4. `aggregate_throughput` - Measures events per second
5. `aggregate_scaling` - Tests with minimal vs full data

**Test scenarios:**
- Minimal data (no optional fields)
- Full data (all fields populated)
- Realistic mix of populated fields

**Expected results:**
- Optimized approach should be **15-25% faster**
- Significantly fewer allocations (10+ fewer per event)
- Better scaling with full data payloads

**Example run:**
```bash
cargo bench --bench aggregate_module
```

**Reading results:**
```
aggregate_comparison/with_cloning
                        time:   [1.2345 µs 1.2678 µs 1.3012 µs]

aggregate_comparison/optimized
                        time:   [987.34 ns 1.0123 µs 1.0456 µs]
```
The optimized version is ~20% faster (1.01µs vs 1.27µs).

---

### 3. Context Creation Benchmarks (`context_creation.rs`)

**Purpose:** Measure the performance improvement from lazy HashMap initialization.

**What it tests:**
- **OLD approach:** Always allocating HashMap in `TrackingPipeContext::new()`
- **NEW approach:** Lazy initialization - only create HashMap when first item is added

**Benchmark groups:**
1. `context_with_hashmap` - Old approach with eager allocation
2. `context_lazy_hashmap` - New approach with lazy allocation
3. `context_comparison` - Direct comparison
4. `context_throughput` - Contexts created per second
5. `context_batch` - Batch creation (10, 100, 1000 contexts)
6. `context_memory` - Memory pressure test (1000 contexts)
7. `context_data_access` - Ensure lazy init doesn't hurt performance when used

**Expected results:**
- Lazy approach should be **10-15% faster** for creation
- 48+ bytes saved per context (HashMap overhead)
- No performance penalty when HashMap is actually needed
- Significant memory savings in high-throughput scenarios

**Example run:**
```bash
cargo bench --bench context_creation
```

**Reading results:**
```
context_comparison/with_hashmap_allocation
                        time:   [234.56 ns 241.23 ns 247.89 ns]

context_comparison/lazy_hashmap
                        time:   [198.45 ns 203.12 ns 207.78 ns]
```
The lazy approach is ~15% faster (203ns vs 241ns).

---

## Understanding the Results

### Criterion.rs Output Format

Criterion provides three time values:
```
time:   [lower_bound estimate upper_bound]
```

- **estimate**: Best estimate of the true mean
- **lower_bound**: Lower bound of 95% confidence interval
- **upper_bound**: Upper bound of 95% confidence interval

### Key Metrics

1. **Time per iteration:**
   - ns (nanoseconds) = 10^-9 seconds
   - µs (microseconds) = 10^-6 seconds
   - ms (milliseconds) = 10^-3 seconds

2. **Throughput:**
   - Shown as "elements/sec" or similar
   - Higher is better

3. **Change detection:**
   - Criterion compares against previous runs
   - Shows `change: [-15.23% -12.45% -9.67%]` if faster
   - Shows `change: [+5.12% +7.89% +10.23%]` if slower

### Comparing Old vs New

To see the improvement percentage:
```
improvement = (old_time - new_time) / old_time * 100
```

Example:
- Old: 1.27 µs
- New: 1.01 µs
- Improvement: (1.27 - 1.01) / 1.27 * 100 = **20.5%**

---

## Expected Performance Gains

Based on our optimizations, you should see:

| Benchmark | Expected Improvement | Impact |
|-----------|---------------------|---------|
| User Agent Triple → Single | 2-3x faster (66% time reduction) | High |
| Aggregate Cloning → Optimized | 15-25% faster | Critical |
| Context Eager → Lazy HashMap | 10-15% faster | Medium |
| **Combined Pipeline** | **25-40% higher throughput** | **Very High** |

---

## Running Benchmarks in CI/CD

### GitHub Actions Example

```yaml
name: Benchmarks

on:
  push:
    branches: [main]
  pull_request:
    branches: [main]

jobs:
  benchmark:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - name: Run benchmarks
        run: cargo bench --no-fail-fast
      - name: Store benchmark results
        uses: benchmark-action/github-action-benchmark@v1
        with:
          tool: 'cargo'
          output-file-path: target/criterion/*/new/estimates.json
```

---

## Profiling and Flamegraphs

For deeper analysis beyond benchmarks:

### 1. Generate Flamegraph

```bash
# Install cargo-flamegraph
cargo install flamegraph

# Generate flamegraph for the application
sudo cargo flamegraph --bin click-tracker

# Opens flamegraph.svg
firefox flamegraph.svg
```

### 2. Memory Profiling with Heaptrack

```bash
# Install heaptrack
sudo apt install heaptrack heaptrack-gui  # Ubuntu/Debian

# Profile the application
heaptrack ./target/release/click-tracker

# Analyze results
heaptrack_gui heaptrack.click-tracker.*.gz
```

### 3. Detailed Profiling with perf

```bash
# Record performance data
perf record -g --call-graph dwarf ./target/release/click-tracker

# Generate report
perf report

# Or convert to flamegraph
perf script | inferno-collapse-perf | inferno-flamegraph > perf-flamegraph.svg
```

---

## Benchmark Best Practices

### 1. Consistent Environment

- Run on dedicated hardware
- Disable CPU frequency scaling: `sudo cpupower frequency-set --governor performance`
- Close other applications
- Run multiple times to establish baseline

### 2. Statistical Significance

- Criterion runs each benchmark multiple times
- Uses statistical analysis to detect real changes
- Look for changes outside the confidence interval

### 3. Comparing Branches

```bash
# Baseline (before optimizations)
git checkout main
cargo bench --bench user_agent_parsing -- --save-baseline main

# Your changes
git checkout optimization-branch
cargo bench --bench user_agent_parsing -- --baseline main
```

This will show the difference from the baseline.

### 4. Micro-benchmarks vs Real-world

- Micro-benchmarks isolate specific functions
- Real-world performance depends on:
  - I/O operations (Redis, Fluvio/Kafka)
  - Network latency
  - Concurrent workloads
  - System resources

Always validate with integration tests and production monitoring.

---

## Interpreting Results for Production

### Throughput Calculation

If a single event takes 100µs to process:
```
Events per second = 1,000,000 µs / 100 µs = 10,000 events/sec
```

With 25% improvement (75µs per event):
```
Events per second = 1,000,000 µs / 75 µs = 13,333 events/sec
```

That's **3,333 more events per second** = **+33% throughput**!

### Cost Savings

Higher throughput means:
- Fewer servers needed
- Lower cloud costs
- Better user experience
- Higher scalability ceiling

---

## Troubleshooting

### Benchmarks Won't Compile

```bash
# Check that all dependencies are available
cargo check --benches

# Ensure criterion is in dev-dependencies
grep -A 2 "dev-dependencies" Cargo.toml
```

### Results Show No Improvement

Possible reasons:
1. Compiler optimizations already eliminated the problem
2. Bottleneck is elsewhere (I/O, not CPU)
3. Test data doesn't represent real workload
4. Need to run with `--release` profile

### Results Are Inconsistent

```bash
# Increase sample size
cargo bench -- --sample-size 1000

# Increase measurement time
cargo bench -- --measurement-time 30

# Reduce noise
sudo nice -n -20 cargo bench
```

---

## Next Steps

After running benchmarks:

1. **Document baseline:** Save initial benchmark results
2. **Track over time:** Run benchmarks on every PR
3. **Set targets:** Define acceptable performance levels
4. **Profile production:** Compare benchmarks to real-world metrics
5. **Iterate:** Use results to guide further optimizations

---

## References

- [Criterion.rs User Guide](https://bheisler.github.io/criterion.rs/book/)
- [Rust Performance Book](https://nnethercote.github.io/perf-book/)
- [Flamegraph Documentation](https://github.com/flamegraph-rs/flamegraph)

---

Generated: 2025-10-30
